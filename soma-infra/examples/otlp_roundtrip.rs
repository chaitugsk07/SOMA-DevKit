//! Manual e2e: emit one span + log + metric via soma-infra's OTLP feature, then exit.
//! Run with: cargo run --example otlp_roundtrip --features otlp
use tracing::{info, info_span};

#[tokio::main]
async fn main() {
    soma_infra::telemetry::init(); // reads OTEL_EXPORTER_OTLP_ENDPOINT + OTEL_SERVICE_NAME from env
    {
        let span = info_span!("roundtrip_test_span", test.marker = "soma-otlp-rt");
        let _e = span.enter();
        info!(target: "roundtrip", "soma-otlp-roundtrip-log-marker hello from emitter");
        // do a tiny bit of "work" inside the span
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }

    // Emit one test metric to exercise the metrics pipeline.
    {
        use opentelemetry::KeyValue;
        let meter = opentelemetry::global::meter("soma-infra-example");
        let histogram = meter
            .f64_histogram("http.server.request.duration")
            .with_unit("s")
            .build();
        histogram.record(
            0.042,
            &[
                KeyValue::new("http.request.method", "GET"),
                KeyValue::new("http.route", "/healthz"),
                KeyValue::new("http.response.status_code", 200_i64),
            ],
        );
    }

    // Give the batch exporter time to flush before the process exits (no explicit
    // shutdown hook in init() yet — see the ponytail note in telemetry.rs).
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    eprintln!("emitter done");
}
