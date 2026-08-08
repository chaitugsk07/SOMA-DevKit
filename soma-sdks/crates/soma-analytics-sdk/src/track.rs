//! Buffered non-blocking usage-event emitter for the soma-analytics ingest path.
//!
//! Usage analytics is lossy-tolerant by design; this client trades delivery
//! guarantees for zero back-pressure on callers. Events that arrive when the
//! internal buffer is full are silently dropped (a `WARN` is logged). The
//! flusher batches events and POSTs them to `POST /api/v1/track` every
//! [`FLUSH_INTERVAL`] or when [`BATCH_SIZE`] events accumulate.

use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;
use serde::Serialize;
use tokio::sync::mpsc;
use uuid::Uuid;

const BUFFER_CAP: usize = 10_000;
const BATCH_SIZE: usize = 200;
const FLUSH_INTERVAL: Duration = Duration::from_secs(10);

// ── WriteKey ──────────────────────────────────────────────────────────────────

/// Write-key newtype. Debug output is redacted so the key never appears in logs.
struct WriteKey(String);

impl fmt::Debug for WriteKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "wk_[REDACTED]")
    }
}

// ── Public event type ─────────────────────────────────────────────────────────

/// A single usage event submitted to soma-analytics via [`TrackClient::track`].
///
/// `insert_id` (UUIDv4) and `ts` (`Utc::now()`) are stamped automatically at
/// enqueue time; callers do not set them.
pub struct TrackEvent {
    /// Event name, e.g. `"button_clicked"`.
    pub event_name: String,
    /// Authenticated user ID, when known.
    pub user_id: Option<String>,
    /// Immutable device / anonymous ID assigned by the client SDK.
    pub device_id: Option<String>,
    /// Arbitrary JSON properties object.
    pub properties: serde_json::Value,
}

// ── Wire format ───────────────────────────────────────────────────────────────

/// Internal fully-stamped event sent over the channel and serialized to JSON.
///
/// Field names are the wire contract for `POST /api/v1/track`; must stay in
/// lockstep with `soma_analytics_events::TrackEvent` on the server.
#[derive(Serialize)]
struct WireEvent {
    event_name: String,
    device_id: Option<String>,
    user_id: Option<String>,
    insert_id: String,
    ts: chrono::DateTime<Utc>,
    properties: serde_json::Value,
}

// ── Inner / client ────────────────────────────────────────────────────────────

struct Inner {
    tx: mpsc::Sender<WireEvent>,
}

/// Buffered, non-blocking usage-event emitter.
///
/// `Clone` is cheap — all state is behind `Arc`. When every clone is dropped
/// the background flusher performs a final flush and exits.
///
/// # Quick start
///
/// ```rust,no_run
/// use soma_analytics_sdk::TrackClient;
/// use soma_analytics_sdk::TrackEvent;
///
/// // Create once at process start; clone cheaply into handlers.
/// let client = TrackClient::new(
///     "http://localhost:8080".into(),
///     "wk_your-write-key".into(),
/// );
///
/// client.track(TrackEvent {
///     event_name: "button_clicked".into(),
///     user_id: Some("user-123".into()),
///     device_id: None,
///     properties: serde_json::json!({ "button": "signup" }),
/// });
/// ```
#[derive(Clone)]
pub struct TrackClient(Arc<Inner>);

impl TrackClient {
    /// Create a new `TrackClient` and spawn the background flusher.
    ///
    /// `base_url` is the soma-analytics server root (e.g. `"http://localhost:8080"`).
    /// `write_key` must be a `wk_…` key issued by `POST /api/v1/write-keys`.
    ///
    /// # Panics
    ///
    /// Panics if the `reqwest::Client` builder fails (e.g. TLS initialisation failure).
    pub fn new(base_url: String, write_key: String) -> Self {
        let (tx, rx) = mpsc::channel(BUFFER_CAP);
        let key = WriteKey(write_key);
        tokio::spawn(flusher(base_url, key, rx));
        Self(Arc::new(Inner { tx }))
    }

    /// Enqueue `event` for delivery. Non-blocking: stamps `insert_id` and `ts`
    /// immediately, then hands off to the background flusher.
    ///
    /// If the internal buffer is full the event is dropped and a `WARN` is
    /// logged. Callers must not rely on delivery guarantees.
    pub fn track(&self, event: TrackEvent) {
        let wire = WireEvent {
            event_name: event.event_name,
            device_id: event.device_id,
            user_id: event.user_id,
            insert_id: Uuid::new_v4().to_string(),
            ts: Utc::now(),
            properties: event.properties,
        };
        if self.0.tx.try_send(wire).is_err() {
            tracing::warn!("TrackClient: channel full, dropping event");
        }
    }
}

// ── Background flusher ────────────────────────────────────────────────────────

async fn flusher(base_url: String, key: WriteKey, mut rx: mpsc::Receiver<WireEvent>) {
    let http = reqwest::Client::builder()
        .use_rustls_tls()
        .build()
        .expect("reqwest client build failed");
    let mut buf: Vec<WireEvent> = Vec::with_capacity(BATCH_SIZE);

    loop {
        match tokio::time::timeout(FLUSH_INTERVAL, rx.recv()).await {
            Ok(Some(ev)) => {
                buf.push(ev);
                if buf.len() >= BATCH_SIZE {
                    flush(&http, &base_url, &key, &mut buf).await;
                }
            }
            Ok(None) => {
                // All TrackClient clones dropped; final flush then exit.
                flush(&http, &base_url, &key, &mut buf).await;
                break;
            }
            Err(_timeout) => {
                flush(&http, &base_url, &key, &mut buf).await;
            }
        }
    }
}

async fn flush(http: &reqwest::Client, base_url: &str, key: &WriteKey, buf: &mut Vec<WireEvent>) {
    if buf.is_empty() {
        return;
    }
    let count = buf.len();
    let url = format!("{}/api/v1/track", base_url.trim_end_matches('/'));
    match http
        .post(&url)
        .header("Authorization", format!("Bearer {}", key.0))
        .json(buf.as_slice())
        .send()
        .await
    {
        Ok(resp) if resp.status().is_success() => {}
        Ok(resp) => {
            let status = resp.status().as_u16();
            tracing::warn!(status, count, "track flush: server error, dropping batch");
        }
        Err(e) => {
            tracing::warn!(count, error = %e, "track flush: transport error, dropping batch");
        }
    }
    buf.clear();
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wire_event_json_shape() {
        let wire = WireEvent {
            event_name: "button_clicked".into(),
            device_id: Some("device-1".into()),
            user_id: None,
            insert_id: "insert-uuid-1234".into(),
            ts: Utc::now(),
            properties: serde_json::json!({ "button": "signup" }),
        };
        let v = serde_json::to_value(&wire).unwrap();
        assert_eq!(v["event_name"], "button_clicked");
        assert_eq!(v["device_id"], "device-1");
        assert!(v.get("user_id").is_some(), "user_id field must be present");
        assert!(v.get("insert_id").is_some(), "insert_id must be present");
        assert!(v.get("ts").is_some(), "ts must be present");
        assert!(v.get("properties").is_some(), "properties must be present");
        // Verify snake_case field names (wire contract)
        assert!(v.get("event_name").is_some());
        assert!(v.get("device_id").is_some());
    }

    #[test]
    fn write_key_debug_redacted() {
        let key = WriteKey("my-super-secret-key".into());
        let dbg = format!("{key:?}");
        assert_eq!(dbg, "wk_[REDACTED]");
        assert!(!dbg.contains("my-super-secret-key"));
    }

    #[tokio::test]
    async fn buffer_cap_overflow_drops_without_blocking() {
        // Point at an unroutable address so the flusher cannot drain the channel.
        let client = TrackClient::new("http://127.0.0.1:1".into(), "wk_test".into());
        let make_event = || TrackEvent {
            event_name: "overflow_test".into(),
            user_id: Some("u1".into()),
            device_id: None,
            properties: serde_json::json!({}),
        };
        // Send more events than BUFFER_CAP; none should block.
        for _ in 0..(BUFFER_CAP + 100) {
            client.track(make_event());
        }
        // Reaching here without hanging proves try_send never blocks.
    }
}
