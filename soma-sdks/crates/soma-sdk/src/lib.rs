//! soma-flags Rust SDK.
//!
//! Two evaluation modes, both OpenFeature-shaped:
//! - **Server-side eval** (`evaluate`, `resolve_*`): one round-trip per call to
//!   `POST /api/client/v1/evaluate`. Always fresh; no local state.
//! - **Local eval** (`refresh` + `evaluate_local`, `resolve_*_local`): fetch the
//!   compiled flagset once via `GET /flagset` (ETag-cheap), then evaluate
//!   in-process with the SAME engine the server uses (`soma_flags_core::evaluate`)
//!   — zero per-call latency, works offline. Call `refresh()` periodically to
//!   pick up rule changes.
//!
//! The SDK depends only on `soma-flags-core` (serde + the pure engine), not the
//! DB layer, so it stays light and is portable to edge/wasm targets.

#![forbid(unsafe_code)]

use std::collections::{BTreeMap, HashMap};
use std::sync::Mutex;

use anyhow::Result;
use thiserror::Error;

pub use soma_flags_core::{Decision, EvalContext, FlagSet, Reason};

// ── Error ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum SdkError {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("server error {status}: {body}")]
    Server { status: u16, body: String },
    #[error("deserialization error: {0}")]
    Deserialize(String),
    #[error("flagset not fetched — call refresh() first")]
    NotFetched,
}

// ── Response types ────────────────────────────────────────────────────────────

/// OpenFeature-shaped resolution result.
#[derive(Debug, Clone)]
pub struct ResolutionDetails<T> {
    pub value: T,
    pub reason: Reason,
    pub variant: Option<String>,
    pub error_code: Option<String>,
}

/// Map of flag_key → Decision.
pub type Decisions = HashMap<String, Decision>;

// ── Wire types for server responses ──────────────────────────────────────────

#[derive(serde::Deserialize)]
struct EvaluateResponse {
    flags: HashMap<String, Decision>,
}

#[derive(serde::Serialize)]
struct EvaluateRequest<'a> {
    environment: &'a str,
    context: &'a EvalContext,
    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<Vec<&'a str>>,
}

// ── Client ────────────────────────────────────────────────────────────────────

pub struct FlagsClient {
    http: reqwest::Client,
    base: String,
    token: String,
    env: String,
    /// Last ETag from GET /flagset, for If-None-Match.
    last_etag: Mutex<Option<String>>,
    /// Cached compiled flagset for local eval (populated by `fetch_flagset`/`refresh`).
    cached_flagset: Mutex<Option<FlagSet>>,
}

impl FlagsClient {
    /// Build a new client. Uses soma_infra's rustls-backed reqwest client.
    pub fn new(
        base: impl Into<String>,
        client_token: impl Into<String>,
        environment: impl Into<String>,
    ) -> Result<Self> {
        let http = soma_infra::http::client()?;
        Ok(Self {
            http,
            base: base.into(),
            token: client_token.into(),
            env: environment.into(),
            last_etag: Mutex::new(None),
            cached_flagset: Mutex::new(None),
        })
    }

    // ── Server-side eval (round-trip per call) ───────────────────────────────

    /// Server-side eval: POST /api/client/v1/evaluate.
    pub async fn evaluate(
        &self,
        ctx: &EvalContext,
        flags: Option<&[&str]>,
    ) -> Result<Decisions, SdkError> {
        let url = format!("{}/api/client/v1/evaluate", self.base);
        let body = EvaluateRequest {
            environment: &self.env,
            context: ctx,
            flags: flags.map(|f| f.to_vec()),
        };
        let resp = self
            .http
            .post(&url)
            .bearer_auth(&self.token)
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(SdkError::Server { status, body });
        }

        let parsed: EvaluateResponse = resp
            .json()
            .await
            .map_err(|e| SdkError::Deserialize(e.to_string()))?;
        Ok(parsed.flags)
    }

    /// Generic typed accessor — shared by all server-eval resolve_* methods.
    async fn resolve<T: Clone>(
        &self,
        flag: &str,
        default: T,
        ctx: &EvalContext,
        extract: impl Fn(&Decision) -> Option<T>,
    ) -> ResolutionDetails<T> {
        match self.evaluate(ctx, Some(&[flag])).await {
            Err(_) => err_details(default, "EVALUATION_ERROR"),
            Ok(decisions) => from_decision(decisions.get(flag), default, &extract),
        }
    }

    /// OpenFeature-shaped bool accessor (server eval). `default` on any error/missing flag.
    pub async fn resolve_bool(
        &self,
        flag: &str,
        default: bool,
        ctx: &EvalContext,
    ) -> ResolutionDetails<bool> {
        self.resolve(flag, default, ctx, |d| d.value.as_bool())
            .await
    }

    /// OpenFeature-shaped string accessor (server eval).
    pub async fn resolve_string(
        &self,
        flag: &str,
        default: &str,
        ctx: &EvalContext,
    ) -> ResolutionDetails<String> {
        self.resolve(flag, default.to_string(), ctx, |d| {
            d.value.as_str().map(|s| s.to_string())
        })
        .await
    }

    /// OpenFeature-shaped integer accessor (server eval).
    pub async fn resolve_int(
        &self,
        flag: &str,
        default: i64,
        ctx: &EvalContext,
    ) -> ResolutionDetails<i64> {
        self.resolve(flag, default, ctx, |d| d.value.as_f64().map(|f| f as i64))
            .await
    }

    /// OpenFeature-shaped JSON accessor (server eval).
    pub async fn resolve_json(
        &self,
        flag: &str,
        default: serde_json::Value,
        ctx: &EvalContext,
    ) -> ResolutionDetails<serde_json::Value> {
        self.resolve(flag, default, ctx, |d| Some(d.value.clone()))
            .await
    }

    // ── Local eval (fetch the bundle once, evaluate in-process) ───────────────

    /// Fetch/refresh the compiled flagset for this environment and cache it for
    /// local eval. ETag-cheap: sends If-None-Match; on 304 keeps the cache.
    /// Call periodically (e.g. on a timer) to pick up rule changes.
    pub async fn refresh(&self) -> Result<(), SdkError> {
        self.fetch_flagset().await.map(|_| ())
    }

    /// GET /api/client/v1/flagset with ETag caching. On 200 caches the FlagSet and
    /// returns it; on 304 returns the cached FlagSet (rules unchanged).
    pub async fn fetch_flagset(&self) -> Result<FlagSet, SdkError> {
        let url = format!(
            "{}/api/client/v1/flagset?environment={}",
            self.base, self.env
        );
        let etag = self
            .last_etag
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone();

        let mut req = self.http.get(&url).bearer_auth(&self.token);
        if let Some(ref e) = etag {
            req = req.header(reqwest::header::IF_NONE_MATCH, e.as_str());
        }

        let resp = req.send().await?;

        if resp.status() == reqwest::StatusCode::NOT_MODIFIED {
            // Rules unchanged — return the cached flagset.
            let guard = self
                .cached_flagset
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            return guard.clone().ok_or(SdkError::NotFetched);
        }

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(SdkError::Server { status, body });
        }

        if let Some(new_etag) = resp
            .headers()
            .get(reqwest::header::ETAG)
            .and_then(|v| v.to_str().ok())
        {
            *self.last_etag.lock().unwrap_or_else(|e| e.into_inner()) = Some(new_etag.to_owned());
        }

        let flagset: FlagSet = resp
            .json()
            .await
            .map_err(|e| SdkError::Deserialize(e.to_string()))?;
        *self
            .cached_flagset
            .lock()
            .unwrap_or_else(|e| e.into_inner()) = Some(flagset.clone());
        Ok(flagset)
    }

    /// Evaluate all flags locally against the cached flagset using the same engine
    /// as the server. Errors with `NotFetched` until `refresh()`/`fetch_flagset()`
    /// has run. No network call.
    pub fn evaluate_local(
        &self,
        ctx: &EvalContext,
    ) -> Result<BTreeMap<String, Decision>, SdkError> {
        let guard = self
            .cached_flagset
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let fs = guard.as_ref().ok_or(SdkError::NotFetched)?;
        Ok(soma_flags_core::evaluate(fs, ctx))
    }

    /// Generic local typed accessor — shared by all local resolve_* methods.
    fn resolve_local<T: Clone>(
        &self,
        flag: &str,
        default: T,
        ctx: &EvalContext,
        extract: impl Fn(&Decision) -> Option<T>,
    ) -> ResolutionDetails<T> {
        match self.evaluate_local(ctx) {
            Err(_) => err_details(default, "EVALUATION_ERROR"),
            Ok(decisions) => from_decision(decisions.get(flag), default, &extract),
        }
    }

    /// OpenFeature-shaped bool accessor (local eval).
    pub fn resolve_bool_local(
        &self,
        flag: &str,
        default: bool,
        ctx: &EvalContext,
    ) -> ResolutionDetails<bool> {
        self.resolve_local(flag, default, ctx, |d| d.value.as_bool())
    }

    /// OpenFeature-shaped string accessor (local eval).
    pub fn resolve_string_local(
        &self,
        flag: &str,
        default: &str,
        ctx: &EvalContext,
    ) -> ResolutionDetails<String> {
        self.resolve_local(flag, default.to_string(), ctx, |d| {
            d.value.as_str().map(|s| s.to_string())
        })
    }

    /// OpenFeature-shaped integer accessor (local eval).
    pub fn resolve_int_local(
        &self,
        flag: &str,
        default: i64,
        ctx: &EvalContext,
    ) -> ResolutionDetails<i64> {
        self.resolve_local(flag, default, ctx, |d| d.value.as_f64().map(|f| f as i64))
    }

    /// OpenFeature-shaped JSON accessor (local eval).
    pub fn resolve_json_local(
        &self,
        flag: &str,
        default: serde_json::Value,
        ctx: &EvalContext,
    ) -> ResolutionDetails<serde_json::Value> {
        self.resolve_local(flag, default, ctx, |d| Some(d.value.clone()))
    }
}

// ── shared resolution helpers ─────────────────────────────────────────────────

fn err_details<T>(default: T, code: &str) -> ResolutionDetails<T> {
    ResolutionDetails {
        value: default,
        reason: Reason::Error,
        variant: None,
        error_code: Some(code.into()),
    }
}

fn from_decision<T: Clone>(
    decision: Option<&Decision>,
    default: T,
    extract: &impl Fn(&Decision) -> Option<T>,
) -> ResolutionDetails<T> {
    match decision {
        None => err_details(default, "FLAG_NOT_FOUND"),
        Some(d) => ResolutionDetails {
            value: extract(d).unwrap_or(default),
            reason: d.reason.clone(),
            variant: d.variation_key.clone(),
            error_code: None,
        },
    }
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use soma_flags_core::{
        evaluate, CompiledClause, CompiledFlag, CompiledRule, CompiledTarget, CompiledVariation,
        EvalContext, FlagSet, Op, Reason, ValueType,
    };
    use std::collections::HashMap;
    use uuid::Uuid;

    fn sample_flagset() -> FlagSet {
        let mut variations = HashMap::new();
        variations.insert(
            "on".to_string(),
            CompiledVariation {
                variation_key: "on".into(),
                value: "SHOW".into(),
            },
        );
        variations.insert(
            "off".to_string(),
            CompiledVariation {
                variation_key: "off".into(),
                value: "HIDE".into(),
            },
        );
        let flag = CompiledFlag {
            flag_id: Uuid::nil(),
            flag_key: "banner".into(),
            value_type: ValueType::String,
            is_active: true,
            enabled: true,
            variations,
            off_variation_key: Some("off".into()),
            default_variation_key: Some("off".into()),
            targets: vec![CompiledTarget {
                target_attribute: "key".into(),
                target_value: "vip".into(),
                variation_key: "on".into(),
            }],
            rules: vec![CompiledRule {
                rule_key: "r0".into(),
                sort_order: 0,
                served_variation_key: Some("on".into()),
                rollout: None,
                clauses: vec![CompiledClause {
                    attribute: "plan".into(),
                    operator: Op::Eq,
                    value: "pro".into(),
                    is_negated: false,
                }],
            }],
            prerequisites: vec![],
        };
        let mut flags = HashMap::new();
        flags.insert("banner".to_string(), flag);
        FlagSet {
            environment_id: Uuid::nil(),
            tenant_id: Uuid::nil(),
            env_key: "prod".into(),
            version: 1,
            flags,
            segments: HashMap::new(),
        }
    }

    fn ctx(key: &str, attrs: &[(&str, &str)]) -> EvalContext {
        let mut attributes = serde_json::Map::new();
        for (k, v) in attrs {
            attributes.insert(
                (*k).to_string(),
                serde_json::Value::String((*v).to_string()),
            );
        }
        EvalContext {
            targeting_key: key.into(),
            attributes,
        }
    }

    /// The compiled flagset survives a JSON round-trip (the GET /flagset wire format)
    /// and the same `evaluate` engine yields identical decisions — i.e. local SDK eval
    /// matches server eval by construction.
    #[test]
    fn wire_round_trip_parity() {
        let fs = sample_flagset();
        let c = ctx("u1", &[("plan", "pro")]);

        let direct = evaluate(&fs, &c);

        // Serialize as the server's GET /flagset does, deserialize as the SDK does.
        let wire = serde_json::to_string(&fs).expect("serialize flagset");
        let fs2: FlagSet = serde_json::from_str(&wire).expect("deserialize flagset");
        let round_trip = evaluate(&fs2, &c);

        assert_eq!(
            serde_json::to_value(&direct).unwrap(),
            serde_json::to_value(&round_trip).unwrap(),
            "decisions must be identical after a JSON round-trip",
        );
        // and the test must actually exercise a rule (not just defaults)
        assert!(matches!(
            round_trip.get("banner").unwrap().reason,
            Reason::RuleMatch
        ));
    }

    /// Different eval paths fire as expected through the round-tripped flagset.
    #[test]
    fn local_eval_paths() {
        let wire = serde_json::to_string(&sample_flagset()).unwrap();
        let fs: FlagSet = serde_json::from_str(&wire).unwrap();

        // individual target wins (key=vip → on)
        assert!(matches!(
            evaluate(&fs, &ctx("vip", &[]))
                .get("banner")
                .unwrap()
                .reason,
            Reason::TargetMatch
        ));
        // rule match (plan=pro → on)
        assert!(matches!(
            evaluate(&fs, &ctx("u1", &[("plan", "pro")]))
                .get("banner")
                .unwrap()
                .reason,
            Reason::RuleMatch
        ));
        // fallthrough default (no match → off)
        assert!(matches!(
            evaluate(&fs, &ctx("u2", &[])).get("banner").unwrap().reason,
            Reason::Default
        ));
    }
}
