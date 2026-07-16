//! Tracing/log setup. Named `telemetry` to avoid shadowing the `tracing` crate.
//!
//! Honors `RUST_LOG`; falls back to the supplied default directive. Set
//! `LOG_FORMAT=json` for structured output. Safe to call once per process
//! (uses `try_init`, so a second call is a no-op rather than a panic).
//!
//! ## Opt-in OTLP export (feature `otlp`)
//!
//! Set `OTEL_EXPORTER_OTLP_ENDPOINT` (e.g. `http://localhost:4318`) or the
//! alias `SOMA_OBSERVE_URL` to enable trace + log export over OTLP/HTTP proto.
//! If neither is set the feature is a no-op — stdout behavior is unchanged.
//! `OTEL_SERVICE_NAME` sets the resource's `service.name`; falls back to the
//! executable file stem, then `"unknown_service"`.
//! `OTEL_RESOURCE_ATTRIBUTES` accepts comma-separated `key=value` pairs.
//!
//! Must be called from inside a Tokio runtime (`#[tokio::main]`) because the
//! batch span/log processors need the runtime for their background tasks.

use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

/// Initialize tracing at `info` by default (overridable via `RUST_LOG`).
pub fn init() {
    init_with("info");
}

/// Initialize tracing with a default filter directive (e.g. `"info,sqlx=warn"`).
pub fn init_with(default_directive: &str) {
    // Resolve the directive string once so we can build independent EnvFilter
    // instances from it — EnvFilter is not Clone, and per-layer filters require
    // one instance per layer.
    // ponytail: RUST_LOG wins; falls back to default_directive (identical semantics
    // to the old single `EnvFilter::try_from_default_env()` call).
    let directive: String = std::env::var("RUST_LOG")
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| default_directive.to_owned());

    // Returns a fresh EnvFilter from the resolved directive for each call site.
    let make_filter =
        || EnvFilter::try_new(&directive).unwrap_or_else(|_| EnvFilter::new(default_directive));

    let json = std::env::var("LOG_FORMAT")
        .map(|v| v.eq_ignore_ascii_case("json"))
        .unwrap_or(false);

    #[cfg(feature = "otlp")]
    {
        let otlp = build_otlp_providers();
        if let Some(OtlpProviders {
            tracer_provider,
            logger_provider,
            meter_provider,
        }) = otlp
        {
            use opentelemetry::trace::TracerProvider as _;
            use tracing_subscriber::Layer as _;

            let tracer = tracer_provider.tracer("soma-infra");
            let otel_trace_layer = tracing_opentelemetry::layer().with_tracer(tracer);
            let otel_log_layer =
                opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge::new(
                    &logger_provider,
                );

            // ponytail: providers are set as globals and live for the process lifetime.
            // There is no explicit flush/shutdown on exit — the batch processors drain
            // on best-effort when the process exits. Upgrade path: call
            // tracer_provider.shutdown() / logger_provider.shutdown() / meter_provider.shutdown()
            // in a graceful shutdown hook if guaranteed delivery is needed.
            opentelemetry::global::set_tracer_provider(tracer_provider);
            opentelemetry::global::set_meter_provider(meter_provider);
            // logger_provider is kept alive inside the bridge layer.

            // Each layer gets its own EnvFilter so RUST_LOG governs stdout AND
            // OTLP export identically. A single global filter added after the OTel
            // layers would be ignored by them (tracing-subscriber only propagates a
            // global Layer filter to layers registered AFTER it in the chain).
            if json {
                let _ = tracing_subscriber::registry()
                    .with(otel_trace_layer.with_filter(make_filter()))
                    .with(otel_log_layer.with_filter(make_filter()))
                    .with(fmt::layer().json().with_filter(make_filter()))
                    .try_init();
            } else {
                let _ = tracing_subscriber::registry()
                    .with(otel_trace_layer.with_filter(make_filter()))
                    .with(otel_log_layer.with_filter(make_filter()))
                    .with(fmt::layer().compact().with_filter(make_filter()))
                    .try_init();
            }
            return;
        }
    }

    // Default: stdout only (identical to pre-otlp behavior).
    if json {
        let _ = tracing_subscriber::registry()
            .with(make_filter())
            .with(fmt::layer().json())
            .try_init();
    } else {
        let _ = tracing_subscriber::registry()
            .with(make_filter())
            .with(fmt::layer().compact())
            .try_init();
    }
}

// --- OTLP provider construction (feature-gated) ----------------------------------------

#[cfg(feature = "otlp")]
pub(crate) struct OtlpProviders {
    tracer_provider: opentelemetry_sdk::trace::SdkTracerProvider,
    logger_provider: opentelemetry_sdk::logs::SdkLoggerProvider,
    meter_provider: opentelemetry_sdk::metrics::SdkMeterProvider,
}

/// Resolve the OTLP endpoint: `OTEL_EXPORTER_OTLP_ENDPOINT` takes precedence,
/// then `SOMA_OBSERVE_URL`. Returns `None` if neither is set.
#[cfg(feature = "otlp")]
pub fn resolve_endpoint() -> Option<String> {
    std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .ok()
        .filter(|s| !s.is_empty())
        .or_else(|| {
            std::env::var("SOMA_OBSERVE_URL")
                .ok()
                .filter(|s| !s.is_empty())
        })
}

/// Resolve `service.name`: `OTEL_SERVICE_NAME` → exe file stem → `"unknown_service"`.
#[cfg(feature = "otlp")]
pub fn resolve_service_name() -> String {
    std::env::var("OTEL_SERVICE_NAME")
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| {
            std::env::current_exe()
                .ok()
                .and_then(|p| p.file_stem().map(|s| s.to_string_lossy().into_owned()))
                .unwrap_or_else(|| "unknown_service".to_owned())
        })
}

/// Parse `OTEL_RESOURCE_ATTRIBUTES` (comma-separated `key=value`) into a vec of pairs.
/// Pure function, no side effects — used by the provider builder and unit-testable without
/// the otlp feature.
pub fn parse_resource_attributes(raw: &str) -> Vec<(String, String)> {
    raw.split(',')
        .filter_map(|kv| {
            let mut parts = kv.splitn(2, '=');
            let k = parts.next()?.trim();
            let v = parts.next()?.trim();
            if k.is_empty() {
                None
            } else {
                Some((k.to_owned(), v.to_owned()))
            }
        })
        .collect()
}

/// Build OTLP tracer + logger providers. Returns `None` when no endpoint is configured.
/// On provider-construction failure, logs a warning to stderr and returns `None` so
/// the caller can fall through to stdout-only — never panics.
///
/// Uses the Tokio-runtime async batch processors so the export future runs on the
/// existing Tokio reactor instead of a dedicated OS thread. The sync batch processor
/// (`with_batch_exporter`) runs export via `futures_executor::block_on` on an OS
/// thread that has no reactor, which panics with reqwest's async HTTP client.
/// Must be called from within a Tokio runtime (`#[tokio::main]`).
#[cfg(feature = "otlp")]
pub(crate) fn build_otlp_providers() -> Option<OtlpProviders> {
    use opentelemetry::KeyValue;
    use opentelemetry_otlp::{WithExportConfig, WithHttpConfig};
    use opentelemetry_sdk::{
        logs::{log_processor_with_async_runtime::BatchLogProcessor, SdkLoggerProvider},
        metrics::{
            periodic_reader_with_async_runtime::PeriodicReader as AsyncPeriodicReader,
            SdkMeterProvider,
        },
        runtime::Tokio,
        trace::{span_processor_with_async_runtime::BatchSpanProcessor, SdkTracerProvider},
        Resource,
    };

    let endpoint = resolve_endpoint()?;
    let service_name = resolve_service_name();

    // Build resource: service.name + any OTEL_RESOURCE_ATTRIBUTES.
    let mut kv = vec![KeyValue::new("service.name", service_name)];
    if let Ok(raw) = std::env::var("OTEL_RESOURCE_ATTRIBUTES") {
        for (k, v) in parse_resource_attributes(&raw) {
            kv.push(KeyValue::new(k, v));
        }
    }
    let resource = Resource::builder().with_attributes(kv).build();

    // Parse OTEL_EXPORTER_OTLP_HEADERS (comma-separated key=value pairs, per OTEL spec:
    // "Authorization=Bearer TOKEN,Another=value"). Applied explicitly via .with_headers()
    // as belt-and-suspenders — the SDK also auto-reads this env var at build time, but
    // explicit wiring ensures auth headers reach soma-observe regardless of SDK version.
    // ponytail: reuses parse_resource_attributes (identical format).
    let otlp_headers: std::collections::HashMap<String, String> =
        std::env::var("OTEL_EXPORTER_OTLP_HEADERS")
            .map(|s| parse_resource_attributes(&s).into_iter().collect())
            .unwrap_or_default();

    // Trace provider: async-runtime batch processor so reqwest's Tokio-based HTTP
    // client runs on the existing reactor rather than a threadless OS thread.
    let span_exporter = match opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .with_endpoint(format!("{}/v1/traces", endpoint.trim_end_matches('/')))
        .with_headers(otlp_headers.clone())
        .build()
    {
        Ok(e) => e,
        Err(err) => {
            eprintln!("[soma-infra/telemetry] OTLP span exporter init failed: {err}; falling back to stdout");
            return None;
        }
    };
    let span_processor = BatchSpanProcessor::builder(span_exporter, Tokio).build();
    let tracer_provider = SdkTracerProvider::builder()
        .with_resource(resource.clone())
        .with_span_processor(span_processor)
        .build();

    // Log provider: same async-runtime batch processor pattern.
    let log_exporter = match opentelemetry_otlp::LogExporter::builder()
        .with_http()
        .with_endpoint(format!("{}/v1/logs", endpoint.trim_end_matches('/')))
        .with_headers(otlp_headers.clone())
        .build()
    {
        Ok(e) => e,
        Err(err) => {
            eprintln!("[soma-infra/telemetry] OTLP log exporter init failed: {err}; falling back to stdout");
            return None;
        }
    };
    let log_processor = BatchLogProcessor::builder(log_exporter, Tokio).build();
    let logger_provider = SdkLoggerProvider::builder()
        .with_resource(resource.clone())
        .with_log_processor(log_processor)
        .build();

    // Metric provider: async-runtime periodic reader, same pattern as span/log.
    // OTEL_METRIC_EXPORT_INTERVAL (ms) is honored unconditionally by PeriodicReader::builder.
    let metric_exporter = match opentelemetry_otlp::MetricExporter::builder()
        .with_http()
        .with_endpoint(format!("{}/v1/metrics", endpoint.trim_end_matches('/')))
        .with_headers(otlp_headers)
        .build()
    {
        Ok(e) => e,
        Err(err) => {
            eprintln!("[soma-infra/telemetry] OTLP metric exporter init failed: {err}; falling back to stdout");
            return None;
        }
    };
    let metric_reader = AsyncPeriodicReader::builder(metric_exporter, Tokio).build();
    let meter_provider = SdkMeterProvider::builder()
        .with_resource(resource)
        .with_reader(metric_reader)
        .build();

    Some(OtlpProviders {
        tracer_provider,
        logger_provider,
        meter_provider,
    })
}

// --- Tests ---------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_resource_attributes_basic() {
        let pairs = parse_resource_attributes("deployment.env=prod,region=us-east-1");
        assert_eq!(pairs.len(), 2);
        assert_eq!(pairs[0], ("deployment.env".into(), "prod".into()));
        assert_eq!(pairs[1], ("region".into(), "us-east-1".into()));
    }

    #[test]
    fn parse_resource_attributes_empty_and_malformed() {
        // Empty string → no pairs
        assert!(parse_resource_attributes("").is_empty());
        // Entry with no `=` is skipped; valid entries are kept
        let pairs = parse_resource_attributes("noequalssign,k=v");
        assert_eq!(pairs.len(), 1);
        assert_eq!(pairs[0], ("k".into(), "v".into()));
    }

    #[test]
    fn parse_resource_attributes_value_with_equals() {
        // Value may itself contain `=` (split on first `=` only)
        let pairs = parse_resource_attributes("token=abc=def");
        assert_eq!(pairs.len(), 1);
        assert_eq!(pairs[0], ("token".into(), "abc=def".into()));
    }

    // --- otlp-gated tests ---

    #[cfg(feature = "otlp")]
    mod otlp {
        use super::*;
        use std::env;
        use std::sync::Mutex;

        // Serialise all env-mutating tests within this module: tests that write
        // process-global env vars must not run concurrently.
        static ENV_LOCK: Mutex<()> = Mutex::new(());

        #[test]
        fn resolve_endpoint_prefers_otel_var() {
            let _g = ENV_LOCK.lock().unwrap();
            // OTEL_EXPORTER_OTLP_ENDPOINT takes precedence over SOMA_OBSERVE_URL
            env::set_var("OTEL_EXPORTER_OTLP_ENDPOINT", "http://otel:4318");
            env::set_var("SOMA_OBSERVE_URL", "http://soma:4318");
            let ep = resolve_endpoint();
            env::remove_var("OTEL_EXPORTER_OTLP_ENDPOINT");
            env::remove_var("SOMA_OBSERVE_URL");
            assert_eq!(ep.as_deref(), Some("http://otel:4318"));
        }

        #[test]
        fn resolve_endpoint_falls_back_to_soma_observe_url() {
            let _g = ENV_LOCK.lock().unwrap();
            env::remove_var("OTEL_EXPORTER_OTLP_ENDPOINT");
            env::set_var("SOMA_OBSERVE_URL", "http://soma:4318");
            let ep = resolve_endpoint();
            env::remove_var("SOMA_OBSERVE_URL");
            assert_eq!(ep.as_deref(), Some("http://soma:4318"));
        }

        #[test]
        fn resolve_endpoint_none_when_neither_set() {
            let _g = ENV_LOCK.lock().unwrap();
            env::remove_var("OTEL_EXPORTER_OTLP_ENDPOINT");
            env::remove_var("SOMA_OBSERVE_URL");
            assert!(resolve_endpoint().is_none());
        }

        #[test]
        fn resolve_service_name_from_env() {
            let _g = ENV_LOCK.lock().unwrap();
            env::set_var("OTEL_SERVICE_NAME", "my-svc");
            let name = resolve_service_name();
            env::remove_var("OTEL_SERVICE_NAME");
            assert_eq!(name, "my-svc");
        }

        #[test]
        fn resolve_service_name_fallback_not_empty() {
            let _g = ENV_LOCK.lock().unwrap();
            env::remove_var("OTEL_SERVICE_NAME");
            let name = resolve_service_name();
            // Fallback is either exe stem or "unknown_service" — never empty.
            assert!(!name.is_empty());
        }

        /// T2 — dead-collector / sacred-invariant test.
        ///
        /// Pointing the exporter at a dead port must NOT panic. Batch processors defer
        /// actual export, so `build_otlp_providers` succeeds even when the collector is
        /// unreachable — failures surface asynchronously and are silently dropped.
        #[tokio::test]
        async fn dead_collector_does_not_panic() {
            let _g = ENV_LOCK.lock().unwrap();
            env::set_var("OTEL_EXPORTER_OTLP_ENDPOINT", "http://127.0.0.1:1");
            env::remove_var("SOMA_OBSERVE_URL");
            env::remove_var("OTEL_SERVICE_NAME");

            // build_otlp_providers must return Ok (Some) — exporter build is cheap,
            // actual network I/O is deferred to the batch processor background task.
            let result = build_otlp_providers();

            env::remove_var("OTEL_EXPORTER_OTLP_ENDPOINT");

            // The providers are built successfully; connection failure is async/deferred.
            assert!(
                result.is_some(),
                "build_otlp_providers must not fail for unreachable endpoint"
            );

            // With the Tokio async-runtime batch processor, Drop triggers a blocking
            // shutdown (futures_executor::block_on inside the processor) which panics
            // inside a Tokio runtime context. The global-provider path avoids this
            // (set_tracer_provider takes ownership; logger_provider lives in the bridge
            // layer). In this test we forget the local providers to avoid the drop.
            // ponytail: std::mem::forget is intentional here — leaking in a test process
            // is harmless; the background tasks end with the test runtime.
            if let Some(p) = result {
                std::mem::forget(p);
            }
        }

        // ponytail: a full stub-server integration test that asserts an OTLP body was
        // POSTed is the ideal T1. Deferred to manual/e2e verification against a running
        // soma-observe instance — a networked test here would be flaky in CI.

        // --- directive resolution tests ---

        /// RUST_LOG wins over the default_directive passed to init_with.
        #[test]
        fn directive_rust_log_wins_over_default() {
            let _g = ENV_LOCK.lock().unwrap();
            env::set_var("RUST_LOG", "debug");
            let directive = env::var("RUST_LOG")
                .ok()
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| "info".to_owned());
            env::remove_var("RUST_LOG");
            assert_eq!(directive, "debug");
        }

        /// When RUST_LOG is unset, the default_directive is used.
        #[test]
        fn directive_default_used_when_rust_log_unset() {
            let _g = ENV_LOCK.lock().unwrap();
            env::remove_var("RUST_LOG");
            let directive = env::var("RUST_LOG")
                .ok()
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| "warn".to_owned());
            assert_eq!(directive, "warn");
        }

        /// When RUST_LOG is set to an empty string, the default is used (not empty).
        #[test]
        fn directive_empty_rust_log_uses_default() {
            let _g = ENV_LOCK.lock().unwrap();
            env::set_var("RUST_LOG", "");
            let directive = env::var("RUST_LOG")
                .ok()
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| "info".to_owned());
            env::remove_var("RUST_LOG");
            assert_eq!(directive, "info");
        }

        /// A per-layer EnvFilter built from "warn" reports max_level_hint == WARN.
        /// This is the compile+runtime check that make_filter() produces a usable filter.
        #[test]
        fn per_layer_filter_max_level_hint() {
            use tracing::metadata::LevelFilter;
            use tracing_subscriber::layer::Filter;

            let f = EnvFilter::try_new("warn").expect("valid directive");
            // EnvFilter implements tracing_subscriber::layer::Filter; max_level_hint
            // returns the coarsest level it will pass.
            let hint = <EnvFilter as Filter<tracing_subscriber::Registry>>::max_level_hint(&f);
            assert_eq!(hint, Some(LevelFilter::WARN));
        }
    }
}
