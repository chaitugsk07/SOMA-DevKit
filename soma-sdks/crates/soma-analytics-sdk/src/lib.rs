//! Rust client for the soma-analytics HTTP API.
//!
//! [`SomaClient`] is the single entry point. Every method maps one-to-one to a
//! REST endpoint; the required role (`Reader` / `Editor` / `Admin`) is stated
//! in each method's documentation. All I/O methods are `async` and require a
//! Tokio runtime.
//!
//! # Key types
//!
//! - [`SomaClient`] — authenticated HTTP client; create once per process.
//! - [`SemanticQuery`] — query input (re-exported from [`query`]).
//! - [`ResultSet`] — tabular result with column metadata and JSON rows.
//! - [`FullModel`] / [`FullCube`] — Editor+ export of the complete tenant model.
//! - [`SdkError`] — the single error type for every fallible operation.
//! - [`TrackClient`] — buffered non-blocking usage-event emitter.
//!
//! # Quick start
//!
//! ```rust,no_run
//! use soma_analytics_sdk::{SomaClient, SemanticQuery};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), soma_analytics_sdk::SdkError> {
//!     let client = SomaClient::new("http://localhost:8080", "your-api-token");
//!     let cubes = client.list_cubes().await?;
//!     println!("{cubes:?}");
//!
//!     let q = SemanticQuery {
//!         cube: "orders".into(),
//!         measures: vec!["orders.count".into()],
//!         dimensions: vec!["orders.status".into()],
//!         filters: vec![],
//!         segments: vec![],
//!         time_dimension: None,
//!         order: vec![],
//!         limit: Some(100),
//!         offset: None,
//!     };
//!     let rs = client.query(&q).await?;
//!     println!("{} rows", rs.meta.row_count);
//!     Ok(())
//! }
//! ```

#![forbid(unsafe_code)]

pub mod query;
pub mod track;

pub use query::{Filter, FilterOp, Granularity, Order, RowFilter, SemanticQuery, TimeDimension};
pub use track::{TrackClient, TrackEvent};

use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

// ── Error ─────────────────────────────────────────────────────────────────────

/// All errors soma-sdk can return.
#[derive(Debug, thiserror::Error)]
pub enum SdkError {
    /// The server returned a non-2xx response.
    #[error("api error {status}: {body}")]
    Api { status: u16, body: String },

    /// A network / transport error from reqwest.
    #[error("request failed: {0}")]
    Transport(#[from] reqwest::Error),

    /// JSON deserialization failed.
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}

// ── Response DTOs ─────────────────────────────────────────────────────────────

/// A measure's metadata from `GET /api/v1/meta`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasureMeta {
    pub name: String,
    /// Aggregation type, e.g. `"count"`, `"sum"`, `"avg"`, `"min"`, `"max"`,
    /// `"count_distinct"`, `"number"`.
    pub agg_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub synonyms: Vec<String>,
}

/// A dimension's metadata from `GET /api/v1/meta`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionMeta {
    pub name: String,
    /// The data type string: `"string"`, `"number"`, `"time"`, `"boolean"`.
    #[serde(rename = "type")]
    pub data_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub synonyms: Vec<String>,
}

/// Per-cube metadata from `GET /api/v1/meta`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CubeMeta {
    pub name: String,
    pub measures: Vec<MeasureMeta>,
    pub dimensions: Vec<DimensionMeta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Response from `GET /api/v1/meta`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaResponse {
    pub cubes: Vec<CubeMeta>,
}

/// Metadata about a result column — mirrors `soma_analytics_storage::types::ColumnMeta`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ColumnMeta {
    pub name: String,
    /// `"string"` | `"number"` | `"time"` | `"boolean"`
    pub data_type: String,
}

/// Per-result-set metadata — mirrors `soma_analytics_storage::types::ResultMeta`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultMeta {
    /// `"hit"` or `"miss"`.
    pub cache: String,
    /// sha256_hex fingerprint of the canonical cache key.
    pub query_fingerprint: String,
    pub row_count: usize,
}

/// A query result — mirrors `soma_analytics_storage::types::ResultSet`.
///
/// Returned by [`SomaClient::query`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultSet {
    pub columns: Vec<ColumnMeta>,
    /// Row data. Each inner `Vec` corresponds to `columns` positionally:
    /// `rows[i][j]` is the value in `columns[j]` for row `i`.
    pub rows: Vec<Vec<serde_json::Value>>,
    pub meta: ResultMeta,
}

/// Result from `POST /api/v1/query/compile`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileResult {
    /// The compiled SQL statement with positional parameters (`$1`, `$2`, …).
    pub sql: String,
    pub columns: Vec<ColumnMeta>,
    /// Number of positional bind parameters in `sql`.
    pub param_count: usize,
}

/// Returned by `create_*` operations: the new entity's id and name.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatedEntity {
    pub id: Uuid,
    pub name: String,
}

/// Returned by [`SomaClient::create_token`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTokenResponse {
    pub id: Uuid,
    pub name: String,
    pub role: String,
    /// Plaintext token — shown once; store securely.
    pub token: String,
}

/// Returned by [`SomaClient::create_write_key`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWriteKeyResponse {
    pub id: Uuid,
    pub name: String,
    /// Plaintext `wk_…` write key — shown once; store securely.
    pub token: String,
}

/// Returned by [`SomaClient::mint_embed_token`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedTokenResponse {
    /// Embed token; pass as `Authorization: Bearer <token>` on requests.
    /// Lifetime is server-clamped to [60, 86400] seconds; check `expires_at`.
    pub token: String,
    /// RFC 3339 timestamp when this token expires.
    pub expires_at: String,
}

// ── Request bodies (pub so the CLI can build them) ────────────────────────────

/// Body for `POST /api/v1/cubes`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCubeBody {
    pub data_source_id: Uuid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Schema-qualified table name this cube reads from, e.g. `"public.orders"`.
    /// Mutually exclusive with `base_sql`; exactly one must be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_table: Option<String>,
    /// Custom SQL subquery this cube reads from (no trailing semicolon).
    /// Mutually exclusive with `sql_table`; exactly one must be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_sql: Option<String>,
    /// Column name used as the row identifier for `count_distinct` deduplication.
    pub primary_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_ttl_secs: Option<i32>,
    /// Column the server uses to enforce tenant isolation. `None` disables
    /// row-level tenancy for this cube.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_column: Option<String>,
}

/// Body for `POST /api/v1/cubes/{id}/dimensions`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDimensionBody {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub synonyms: Vec<String>,
    /// SQL expression for this dimension. Use `{CUBE}` as a placeholder for
    /// the cube's table/subquery alias, e.g. `"{CUBE}.status"`.
    pub sql_expr: String,
    /// Data type: `"string"`, `"number"`, `"time"`, `"boolean"`.
    pub data_type: String,
}

/// Body for `POST /api/v1/cubes/{id}/measures`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMeasureBody {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub synonyms: Vec<String>,
    /// SQL expression for the aggregated value. Use `{CUBE}` as the cube alias.
    /// May be `None` for `agg_type = "count"` (counts all rows).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_expr: Option<String>,
    /// Aggregation applied to `sql_expr`. One of `"count"`, `"count_distinct"`,
    /// `"sum"`, `"avg"`, `"min"`, `"max"`, `"number"`.
    pub agg_type: String,
}

/// Body for `POST /api/v1/cubes/{id}/joins`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateJoinBody {
    pub target_cube_id: Uuid,
    pub name: String,
    /// Join cardinality. One of `"many_to_one"`, `"one_to_many"`, `"one_to_one"`.
    pub relationship: String,
    /// SQL ON expression. Use `{CUBE}` for this cube and `{TARGET}` for the
    /// target cube, e.g. `"{CUBE}.id = {TARGET}.order_id"`.
    pub sql_on: String,
}

/// Body for `POST /api/v1/cubes/{id}/segments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSegmentBody {
    pub name: String,
    /// SQL WHERE fragment applied when the segment is active. Use `{CUBE}` as
    /// the cube alias, e.g. `"{CUBE}.status = 'active'"`.
    pub sql_expr: String,
}

// ── Full entity types (returned by list and PATCH endpoints, Editor+) ─────────

/// A dimension with full SQL fields — returned by `GET /cubes/{id}/dimensions`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullDimension {
    pub id: String,
    pub name: String,
    /// The SQL expression (uses `{CUBE}` token).
    pub sql: String,
    /// Data type: `"string"`, `"number"`, `"time"`, `"boolean"`.
    #[serde(rename = "type")]
    pub data_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default)]
    pub synonyms: Vec<String>,
}

/// A measure with full SQL fields — returned by `GET /cubes/{id}/measures`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullMeasure {
    pub id: String,
    pub name: String,
    /// Aggregation type. One of `"count"`, `"count_distinct"`, `"sum"`, `"avg"`,
    /// `"min"`, `"max"`, `"number"`.
    pub agg_type: String,
    /// The SQL expression (uses `{CUBE}` token). `None` for `agg_type = "count"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default)]
    pub synonyms: Vec<String>,
}

/// A join — returned by `GET /cubes/{id}/joins`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullJoin {
    pub id: String,
    pub name: String,
    pub target_cube: String,
    /// Join cardinality: `"many_to_one"`, `"one_to_many"`, or `"one_to_one"`.
    pub relationship: String,
    /// The ON SQL expression (uses `{CUBE}` and `{TARGET}` tokens).
    pub sql: String,
}

/// A segment — returned by `GET /cubes/{id}/segments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullSegment {
    pub id: String,
    pub name: String,
    /// SQL WHERE fragment (uses `{CUBE}` token).
    pub sql: String,
}

// ── PATCH request body types ──────────────────────────────────────────────────

/// Body for `PATCH /api/v1/cubes/{id}`.
///
/// All fields are optional: absent = keep existing.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchCubeBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_ttl_secs: Option<i32>,
}

/// Body for `PATCH /api/v1/cubes/{id}/dimensions/{dim_id}`.
///
/// All fields are optional: absent = keep existing.
/// `synonyms: Some([])` clears synonyms; `synonyms: None` (absent) keeps existing.
/// `name` must not be set — the server rejects it with 422.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchDimensionBody {
    /// SQL expression with `{CUBE}` placeholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_expr: Option<String>,
    /// Values: `"string"`, `"number"`, `"time"`, `"boolean"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// `Some([])` clears synonyms; `None` omits the field (keep existing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synonyms: Option<Vec<String>>,
}

/// Body for `PATCH /api/v1/cubes/{id}/measures/{meas_id}`.
///
/// `name` must not be set — the server rejects it with 422.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchMeasureBody {
    /// SQL expression with `{CUBE}` placeholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_expr: Option<String>,
    /// Values: `"count"`, `"count_distinct"`, `"sum"`, `"avg"`, `"min"`, `"max"`, `"number"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agg_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// `Some([])` clears synonyms; `None` omits the field (keep existing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synonyms: Option<Vec<String>>,
}

/// Body for `PATCH /api/v1/cubes/{id}/joins/{join_id}`.
///
/// `name` and `target_cube` must not be set — the server rejects them with 422.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchJoinBody {
    /// Values: `"many_to_one"`, `"one_to_many"`, `"one_to_one"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<String>,
    /// SQL ON expression with `{CUBE}` and `{TARGET}` placeholders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_on: Option<String>,
}

/// Body for `PATCH /api/v1/cubes/{id}/segments/{seg_id}`.
///
/// `name` must not be set — the server rejects it with 422.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchSegmentBody {
    /// SQL WHERE fragment with `{CUBE}` placeholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_expr: Option<String>,
}

// ── Full model types (GET /api/v1/model — Editor+ export) ─────────────────────

/// A data source entry from `GET /api/v1/datasources` (Admin+).
///
/// Used by [`SomaClient::list_data_sources`] to build the name→id map for `apply`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSourceInfo {
    pub id: Uuid,
    pub name: String,
    /// Connection driver, e.g. `"postgres"`.
    pub driver: String,
}

/// A cube from `GET /api/v1/model` (Editor+ export).
///
/// The `data_source` field is the UUID string of the owning data source.
/// Use [`SomaClient::list_data_sources`] to resolve UUIDs to names.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FullCube {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// UUID string of the owning data source.
    pub data_source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_table: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_sql: Option<String>,
    /// Column used as the row identifier for `count_distinct` deduplication.
    pub primary_key: String,
    /// Column used for tenant isolation. Empty string means no tenancy enforcement.
    pub tenant_column: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_ttl_secs: Option<i32>,
    #[serde(default)]
    pub dimensions: Vec<FullDimension>,
    #[serde(default)]
    pub measures: Vec<FullMeasure>,
    #[serde(default)]
    pub joins: Vec<FullJoin>,
    #[serde(default)]
    pub segments: Vec<FullSegment>,
}

/// Full tenant model from `GET /api/v1/model` (Editor+).
///
/// Exposes raw SQL expressions on every entity; Editor+ role required.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FullModel {
    #[serde(default)]
    pub cubes: Vec<FullCube>,
}

// ── Validate types (POST /api/v1/model/validate) ──────────────────────────────

/// Dimension entry for `POST /api/v1/model/validate`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ValidateDimDef {
    pub name: String,
    /// SQL expression with `{CUBE}` placeholder.
    pub sql: String,
    /// Data type: `"string"`, `"number"`, `"time"`, `"boolean"`.
    #[serde(rename = "type")]
    pub data_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub synonyms: Vec<String>,
}

/// Measure entry for `POST /api/v1/model/validate`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ValidateMeasDef {
    pub name: String,
    /// Aggregation type. One of `"count"`, `"count_distinct"`, `"sum"`, `"avg"`,
    /// `"min"`, `"max"`, `"number"`.
    #[serde(rename = "type")]
    pub agg_type: String,
    /// SQL expression with `{CUBE}` placeholder. `None` for `agg_type = "count"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub synonyms: Vec<String>,
}

/// Join entry for `POST /api/v1/model/validate`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ValidateJoinDef {
    pub name: String,
    pub target_cube: String,
    /// Join cardinality: `"many_to_one"`, `"one_to_many"`, `"one_to_one"`.
    pub relationship: String,
    /// SQL ON expression with `{CUBE}` and `{TARGET}` placeholders.
    pub sql_on: String,
}

/// Segment entry for `POST /api/v1/model/validate`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ValidateSegDef {
    pub name: String,
    /// SQL WHERE fragment with `{CUBE}` placeholder.
    pub sql: String,
}

/// Cube entry for `POST /api/v1/model/validate`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ValidateCubeDef {
    pub name: String,
    /// UUID string of the owning data source.
    pub data_source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_table: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_sql: Option<String>,
    /// Column used as the row identifier for `count_distinct` deduplication.
    pub primary_key: String,
    /// Column used for tenant isolation. Empty string disables row-level tenancy.
    pub tenant_column: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<ValidateDimDef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub measures: Vec<ValidateMeasDef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub joins: Vec<ValidateJoinDef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub segments: Vec<ValidateSegDef>,
}

/// Request body for `POST /api/v1/model/validate`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ValidateModelRequest {
    #[serde(default)]
    pub cubes: Vec<ValidateCubeDef>,
}

/// Response from `POST /api/v1/model/validate`.
///
/// The server returns 200 when the model is valid and 422 when it is not.
/// Both status codes carry a `ValidateResponse` body.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateResponse {
    pub valid: bool,
    /// Non-empty on invalid models. Each string names the cube/entity and the problem.
    #[serde(default)]
    pub errors: Vec<String>,
}

// ── Private helper structs ────────────────────────────────────────────────────

/// `GET /api/v1/cubes` returns `{"cubes": ["name1", "name2"]}`.
#[derive(Deserialize)]
struct CubesListResponse {
    cubes: Vec<String>,
}

#[derive(Serialize)]
struct CreateDataSourceBody<'a> {
    name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    driver: Option<&'a str>,
}

#[derive(Serialize)]
struct CreateTokenBody<'a> {
    name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<&'a str>,
}

#[derive(Serialize)]
struct MintEmbedTokenBody {
    user_id: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    row_filters: Vec<RowFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cube: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl_secs: Option<u64>,
}

// ── Client ────────────────────────────────────────────────────────────────────

/// Async client for the soma-analytics REST API.
///
/// Built on `reqwest::Client` with connection pooling handled internally.
/// Do not wrap this in an `Arc` pool; clone it cheaply instead.
///
/// # Construction
///
/// ```rust,no_run
/// let client = soma_analytics_sdk::SomaClient::new("http://localhost:8080", "your-api-token");
/// ```
pub struct SomaClient {
    http: reqwest::Client,
    base_url: String,
    api_key: String,
}

impl SomaClient {
    /// Create a new client.
    ///
    /// `base_url` is the soma-analytics server root (e.g. `"http://localhost:8080"`).
    /// A trailing slash is stripped automatically. `api_key` is a Bearer token
    /// with at least Reader role.
    ///
    /// # Panics
    ///
    /// Panics if the `reqwest::Client` builder fails (e.g. TLS initialisation failure).
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Self {
        let http = reqwest::Client::builder()
            .use_rustls_tls()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .build()
            .expect("reqwest client build failed");
        Self {
            http,
            base_url: base_url.into().trim_end_matches('/').to_owned(),
            api_key: api_key.into(),
        }
    }

    // ── Private helpers ───────────────────────────────────────────────────────

    fn auth_header(&self) -> String {
        format!("Bearer {}", self.api_key)
    }

    async fn check_response(resp: reqwest::Response) -> Result<reqwest::Response, SdkError> {
        if resp.status().is_success() {
            return Ok(resp);
        }
        let status = resp.status().as_u16();
        let body = resp.text().await.unwrap_or_default();
        Err(SdkError::Api { status, body })
    }

    async fn get(&self, path: &str) -> Result<reqwest::Response, SdkError> {
        let url = format!("{}/api/v1{}", self.base_url, path);
        let resp = self
            .http
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;
        Self::check_response(resp).await
    }

    async fn post<B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<reqwest::Response, SdkError> {
        let url = format!("{}/api/v1{}", self.base_url, path);
        let resp = self
            .http
            .post(&url)
            .header("Authorization", self.auth_header())
            .json(body)
            .send()
            .await?;
        Self::check_response(resp).await
    }

    async fn delete_req(&self, path: &str) -> Result<reqwest::Response, SdkError> {
        let url = format!("{}/api/v1{}", self.base_url, path);
        let resp = self
            .http
            .delete(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;
        Self::check_response(resp).await
    }

    async fn patch_req<B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<reqwest::Response, SdkError> {
        let url = format!("{}/api/v1{}", self.base_url, path);
        let resp = self
            .http
            .patch(&url)
            .header("Authorization", self.auth_header())
            .json(body)
            .send()
            .await?;
        Self::check_response(resp).await
    }

    // ── Public API ────────────────────────────────────────────────────────────

    /// `GET /api/v1/meta` — governed model (cubes, measures, dimensions with descriptions).
    ///
    /// Requires Reader role or higher.
    pub async fn meta(&self) -> Result<MetaResponse, SdkError> {
        let resp = self.get("/meta").await?;
        Ok(resp.json().await?)
    }

    /// `POST /api/v1/query` — execute a semantic query.
    ///
    /// Requires Reader role or higher. Results may be served from cache.
    pub async fn query(&self, q: &SemanticQuery) -> Result<ResultSet, SdkError> {
        let resp = self.post("/query", q).await?;
        Ok(resp.json().await?)
    }

    /// `POST /api/v1/query/compile` — dry-run compile; returns SQL without executing.
    ///
    /// On a compile error the server returns 422 with `{"error": "compile_error", "detail": "..."}`.
    /// The caller receives `Err(SdkError::Api { status: 422, body })` and should parse the body.
    ///
    /// Requires Reader role or higher.
    pub async fn compile_query(&self, q: &SemanticQuery) -> Result<CompileResult, SdkError> {
        let resp = self.post("/query/compile", q).await?;
        Ok(resp.json().await?)
    }

    /// `GET /api/v1/cubes` — list cube names for the authenticated tenant.
    ///
    /// Requires Reader role or higher.
    pub async fn list_cubes(&self) -> Result<Vec<String>, SdkError> {
        let resp = self.get("/cubes").await?;
        let body: CubesListResponse = resp.json().await?;
        Ok(body.cubes)
    }

    /// `POST /api/v1/datasources` — register a data source.
    ///
    /// Requires Admin role. `driver` defaults to `"postgres"` server-side.
    pub async fn create_data_source(
        &self,
        name: &str,
        driver: Option<&str>,
    ) -> Result<CreatedEntity, SdkError> {
        let body = CreateDataSourceBody { name, driver };
        let resp = self.post("/datasources", &body).await?;
        Ok(resp.json().await?)
    }

    /// `POST /api/v1/cubes` — create a cube.
    ///
    /// Requires Editor role or higher.
    pub async fn create_cube(&self, body: &CreateCubeBody) -> Result<CreatedEntity, SdkError> {
        let resp = self.post("/cubes", body).await?;
        Ok(resp.json().await?)
    }

    /// `POST /api/v1/cubes/{cube_id}/dimensions` — add a dimension to a cube.
    ///
    /// Requires Editor role or higher.
    pub async fn create_dimension(
        &self,
        cube_id: Uuid,
        body: &CreateDimensionBody,
    ) -> Result<CreatedEntity, SdkError> {
        let path = format!("/cubes/{cube_id}/dimensions");
        let resp = self.post(&path, body).await?;
        Ok(resp.json().await?)
    }

    /// `POST /api/v1/cubes/{cube_id}/measures` — add a measure to a cube.
    ///
    /// Requires Editor role or higher.
    pub async fn create_measure(
        &self,
        cube_id: Uuid,
        body: &CreateMeasureBody,
    ) -> Result<CreatedEntity, SdkError> {
        let path = format!("/cubes/{cube_id}/measures");
        let resp = self.post(&path, body).await?;
        Ok(resp.json().await?)
    }

    /// `POST /api/v1/cubes/{cube_id}/joins` — add a join to a cube.
    ///
    /// Requires Editor role or higher.
    pub async fn create_join(
        &self,
        cube_id: Uuid,
        body: &CreateJoinBody,
    ) -> Result<CreatedEntity, SdkError> {
        let path = format!("/cubes/{cube_id}/joins");
        let resp = self.post(&path, body).await?;
        Ok(resp.json().await?)
    }

    /// `POST /api/v1/cubes/{cube_id}/segments` — add a segment to a cube.
    ///
    /// Requires Editor role or higher.
    pub async fn create_segment(
        &self,
        cube_id: Uuid,
        body: &CreateSegmentBody,
    ) -> Result<CreatedEntity, SdkError> {
        let path = format!("/cubes/{cube_id}/segments");
        let resp = self.post(&path, body).await?;
        Ok(resp.json().await?)
    }

    // ── List entity endpoints (Editor+) ───────────────────────────────────────

    /// `GET /api/v1/cubes/{cube_id}/dimensions` — list all dimensions for a cube.
    ///
    /// Returns full objects including `sql` expressions. Requires Editor role or higher.
    pub async fn list_dimensions(&self, cube_id: Uuid) -> Result<Vec<FullDimension>, SdkError> {
        let path = format!("/cubes/{cube_id}/dimensions");
        let resp = self.get(&path).await?;
        Ok(resp.json().await?)
    }

    /// `GET /api/v1/cubes/{cube_id}/measures` — list all measures for a cube.
    ///
    /// Returns full objects including `sql` expressions. Requires Editor role or higher.
    pub async fn list_measures(&self, cube_id: Uuid) -> Result<Vec<FullMeasure>, SdkError> {
        let path = format!("/cubes/{cube_id}/measures");
        let resp = self.get(&path).await?;
        Ok(resp.json().await?)
    }

    /// `GET /api/v1/cubes/{cube_id}/joins` — list all joins for a cube.
    ///
    /// Returns full objects including `sql` expressions. Requires Editor role or higher.
    pub async fn list_joins(&self, cube_id: Uuid) -> Result<Vec<FullJoin>, SdkError> {
        let path = format!("/cubes/{cube_id}/joins");
        let resp = self.get(&path).await?;
        Ok(resp.json().await?)
    }

    /// `GET /api/v1/cubes/{cube_id}/segments` — list all segments for a cube.
    ///
    /// Returns full objects including `sql` expressions. Requires Editor role or higher.
    pub async fn list_segments(&self, cube_id: Uuid) -> Result<Vec<FullSegment>, SdkError> {
        let path = format!("/cubes/{cube_id}/segments");
        let resp = self.get(&path).await?;
        Ok(resp.json().await?)
    }

    // ── PATCH entity endpoints (Editor+) ──────────────────────────────────────

    /// `PATCH /api/v1/cubes/{cube_id}/dimensions/{dim_id}` — partial update a dimension.
    ///
    /// Absent fields are unchanged. `synonyms: Some([])` clears synonyms.
    /// `name` must be absent — the server rejects it with 422.
    /// Requires Editor role or higher.
    pub async fn patch_dimension(
        &self,
        cube_id: Uuid,
        dim_id: Uuid,
        body: &PatchDimensionBody,
    ) -> Result<CreatedEntity, SdkError> {
        let path = format!("/cubes/{cube_id}/dimensions/{dim_id}");
        let resp = self.patch_req(&path, body).await?;
        Ok(resp.json().await?)
    }

    /// `PATCH /api/v1/cubes/{cube_id}/measures/{meas_id}` — partial update a measure.
    ///
    /// `name` must be absent — the server rejects it with 422.
    /// Changing `agg_type` from `count` to another type while `sql_expr` is null returns 422.
    /// Requires Editor role or higher.
    pub async fn patch_measure(
        &self,
        cube_id: Uuid,
        meas_id: Uuid,
        body: &PatchMeasureBody,
    ) -> Result<CreatedEntity, SdkError> {
        let path = format!("/cubes/{cube_id}/measures/{meas_id}");
        let resp = self.patch_req(&path, body).await?;
        Ok(resp.json().await?)
    }

    /// `PATCH /api/v1/cubes/{cube_id}/joins/{join_id}` — partial update a join.
    ///
    /// `name` and `target_cube` must be absent — the server rejects them with 422.
    /// Requires Editor role or higher.
    pub async fn patch_join(
        &self,
        cube_id: Uuid,
        join_id: Uuid,
        body: &PatchJoinBody,
    ) -> Result<CreatedEntity, SdkError> {
        let path = format!("/cubes/{cube_id}/joins/{join_id}");
        let resp = self.patch_req(&path, body).await?;
        Ok(resp.json().await?)
    }

    /// `PATCH /api/v1/cubes/{cube_id}/segments/{seg_id}` — partial update a segment.
    ///
    /// `name` must be absent — the server rejects it with 422.
    /// Requires Editor role or higher.
    pub async fn patch_segment(
        &self,
        cube_id: Uuid,
        seg_id: Uuid,
        body: &PatchSegmentBody,
    ) -> Result<CreatedEntity, SdkError> {
        let path = format!("/cubes/{cube_id}/segments/{seg_id}");
        let resp = self.patch_req(&path, body).await?;
        Ok(resp.json().await?)
    }

    /// `PATCH /api/v1/cubes/{cube_id}` — partial update a cube's patchable fields.
    ///
    /// Patchable: `title`, `description`, `cache_ttl_secs`.
    /// Immutable: `name`, `sql_table`, `base_sql`, `primary_key`, `tenant_column`, `data_source`.
    /// Requires Editor role or higher.
    pub async fn patch_cube(
        &self,
        cube_id: Uuid,
        body: &PatchCubeBody,
    ) -> Result<CreatedEntity, SdkError> {
        let path = format!("/cubes/{cube_id}");
        let resp = self.patch_req(&path, body).await?;
        Ok(resp.json().await?)
    }

    /// `DELETE /api/v1/cubes/{cube_id}/dimensions/{dim_id}` — delete a dimension.
    ///
    /// Requires Editor role or higher.
    pub async fn delete_dimension(&self, cube_id: Uuid, dim_id: Uuid) -> Result<(), SdkError> {
        let path = format!("/cubes/{cube_id}/dimensions/{dim_id}");
        self.delete_req(&path).await?;
        Ok(())
    }

    /// `DELETE /api/v1/cubes/{cube_id}/measures/{meas_id}` — delete a measure.
    ///
    /// Requires Editor role or higher.
    pub async fn delete_measure(&self, cube_id: Uuid, meas_id: Uuid) -> Result<(), SdkError> {
        let path = format!("/cubes/{cube_id}/measures/{meas_id}");
        self.delete_req(&path).await?;
        Ok(())
    }

    /// `DELETE /api/v1/cubes/{cube_id}/joins/{join_id}` — delete a join.
    ///
    /// Requires Editor role or higher.
    pub async fn delete_join(&self, cube_id: Uuid, join_id: Uuid) -> Result<(), SdkError> {
        let path = format!("/cubes/{cube_id}/joins/{join_id}");
        self.delete_req(&path).await?;
        Ok(())
    }

    /// `DELETE /api/v1/cubes/{cube_id}/segments/{seg_id}` — delete a segment.
    ///
    /// Requires Editor role or higher.
    pub async fn delete_segment(&self, cube_id: Uuid, seg_id: Uuid) -> Result<(), SdkError> {
        let path = format!("/cubes/{cube_id}/segments/{seg_id}");
        self.delete_req(&path).await?;
        Ok(())
    }

    /// `DELETE /api/v1/cubes/{cube_id}` — delete a cube and all its children.
    ///
    /// Requires Editor role or higher.
    pub async fn delete_cube(&self, cube_id: Uuid) -> Result<(), SdkError> {
        let path = format!("/cubes/{cube_id}");
        self.delete_req(&path).await?;
        Ok(())
    }

    /// `DELETE /api/v1/datasources/{id}` — delete a data source.
    ///
    /// Requires Admin role.
    pub async fn delete_data_source(&self, ds_id: Uuid) -> Result<(), SdkError> {
        let path = format!("/datasources/{ds_id}");
        self.delete_req(&path).await?;
        Ok(())
    }

    /// `GET /api/v1/model` — export the full model for the caller's tenant (Editor+).
    ///
    /// Returns every cube with its SQL expressions, dimensions, measures, joins, and segments.
    /// The `data_source` field on each cube is a UUID string; pair with
    /// [`SomaClient::list_data_sources`] to resolve names.
    pub async fn get_model(&self) -> Result<FullModel, SdkError> {
        let resp = self.get("/model").await?;
        Ok(resp.json().await?)
    }

    /// `GET /api/v1/datasources` — list all data sources for the tenant (Admin+).
    pub async fn list_data_sources(&self) -> Result<Vec<DataSourceInfo>, SdkError> {
        let resp = self.get("/datasources").await?;
        Ok(resp.json().await?)
    }

    /// `POST /api/v1/model/validate` — validate a model definition without writing to DB.
    ///
    /// Returns `Ok(ValidateResponse { valid: true, .. })` on a valid model.
    /// Returns `Ok(ValidateResponse { valid: false, errors })` on a validation failure.
    /// A 422 from the server is treated as a structured validation response, not a transport error.
    ///
    /// Requires Editor role.
    pub async fn validate_model(
        &self,
        body: &ValidateModelRequest,
    ) -> Result<ValidateResponse, SdkError> {
        let url = format!("{}/api/v1/model/validate", self.base_url);
        let resp = self
            .http
            .post(&url)
            .header("Authorization", self.auth_header())
            .json(body)
            .send()
            .await?;
        let status = resp.status();
        if status.is_success() || status.as_u16() == 422 {
            // Both 200 (valid) and 422 (invalid) carry a ValidateResponse body.
            Ok(resp.json().await?)
        } else {
            let body_text = resp.text().await.unwrap_or_default();
            Err(SdkError::Api {
                status: status.as_u16(),
                body: body_text,
            })
        }
    }

    /// `POST /api/v1/tokens` — create an API token.
    ///
    /// Requires Admin role. The plaintext token is returned once — store securely.
    pub async fn create_token(
        &self,
        name: &str,
        role: Option<&str>,
    ) -> Result<CreateTokenResponse, SdkError> {
        let body = CreateTokenBody { name, role };
        let resp = self.post("/tokens", &body).await?;
        Ok(resp.json().await?)
    }

    /// `POST /api/v1/write-keys` — create a write key.
    ///
    /// Requires Admin role. The plaintext `wk_…` token is returned once —
    /// store securely and use it as `Authorization: Bearer wk_…` on ingest calls.
    pub async fn create_write_key(&self, name: &str) -> Result<CreateWriteKeyResponse, SdkError> {
        let body = serde_json::json!({ "name": name });
        let resp = self.post("/write-keys", &body).await?;
        Ok(resp.json().await?)
    }

    /// `POST /api/v1/embed/token` — mint a scoped embed token.
    ///
    /// Requires Editor role or higher. `user_id` identifies the end-user in audit
    /// logs. `row_filters` are injected as bound equality conditions on every query
    /// (never raw SQL). `cube` optionally locks the token to a single cube.
    ///
    /// `ttl_secs` sets the token lifetime. The server clamps it to [60, 86400];
    /// pass `None` to use the server default (600 s / 10 minutes).
    pub async fn mint_embed_token(
        &self,
        user_id: &str,
        row_filters: Vec<RowFilter>,
        cube: Option<String>,
        ttl_secs: Option<u64>,
    ) -> Result<EmbedTokenResponse, SdkError> {
        let body = MintEmbedTokenBody {
            user_id: user_id.to_owned(),
            row_filters,
            cube,
            ttl_secs,
        };
        let resp = self.post("/embed/token", &body).await?;
        Ok(resp.json().await?)
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_round_trip_semantic_query() {
        let q = SemanticQuery {
            cube: "orders".into(),
            measures: vec!["orders.count".into(), "orders.total_revenue".into()],
            dimensions: vec!["orders.status".into()],
            filters: vec![],
            segments: vec![],
            time_dimension: None,
            order: vec![],
            limit: Some(100),
            offset: None,
        };
        let json = serde_json::to_string(&q).unwrap();
        let q2: SemanticQuery = serde_json::from_str(&json).unwrap();
        assert_eq!(q.cube, q2.cube);
        assert_eq!(q.measures, q2.measures);
        assert_eq!(q.dimensions, q2.dimensions);
        assert_eq!(q.limit, q2.limit);
    }

    #[test]
    fn serde_round_trip_result_set() {
        let rs = ResultSet {
            columns: vec![
                ColumnMeta {
                    name: "orders.status".into(),
                    data_type: "string".into(),
                },
                ColumnMeta {
                    name: "orders.count".into(),
                    data_type: "number".into(),
                },
            ],
            rows: vec![vec![serde_json::json!("completed"), serde_json::json!(42)]],
            meta: ResultMeta {
                cache: "miss".into(),
                query_fingerprint: "sha256:abc123".into(),
                row_count: 1,
            },
        };
        let json = serde_json::to_string(&rs).unwrap();
        let rs2: ResultSet = serde_json::from_str(&json).unwrap();
        assert_eq!(rs2.columns.len(), 2);
        assert_eq!(rs2.rows[0][0], serde_json::json!("completed"));
        assert_eq!(rs2.meta.row_count, 1);
        assert_eq!(rs2.meta.cache, "miss");
    }

    #[test]
    fn serde_round_trip_full_dimension() {
        let dim = FullDimension {
            id: "dim-1".into(),
            name: "status".into(),
            sql: "{CUBE}.status".into(),
            data_type: "string".into(),
            description: Some("Order status".into()),
            title: None,
            synonyms: vec!["state".into()],
        };
        let json = serde_json::to_string(&dim).unwrap();
        let dim2: FullDimension = serde_json::from_str(&json).unwrap();
        assert_eq!(dim2.id, "dim-1");
        assert_eq!(dim2.sql, "{CUBE}.status");
        assert_eq!(dim2.data_type, "string");
        assert_eq!(dim2.synonyms, vec!["state"]);
    }

    #[test]
    fn serde_round_trip_full_measure() {
        let m = FullMeasure {
            id: "meas-1".into(),
            name: "count".into(),
            agg_type: "count".into(),
            sql: None,
            description: None,
            title: None,
            synonyms: vec![],
        };
        let json = serde_json::to_string(&m).unwrap();
        let m2: FullMeasure = serde_json::from_str(&json).unwrap();
        assert_eq!(m2.agg_type, "count");
        assert!(m2.sql.is_none());
    }

    #[test]
    fn patch_dimension_body_synonyms_some_empty_serializes() {
        let body = PatchDimensionBody {
            synonyms: Some(vec![]),
            ..Default::default()
        };
        let v = serde_json::to_value(&body).unwrap();
        // synonyms: [] must appear in the JSON (present-empty = clear)
        assert_eq!(v["synonyms"], serde_json::json!([]));
    }

    #[test]
    fn patch_dimension_body_synonyms_none_absent_from_json() {
        let body = PatchDimensionBody {
            sql_expr: Some("{CUBE}.x".into()),
            ..Default::default()
        };
        let v = serde_json::to_value(&body).unwrap();
        // synonyms must be absent (skip_serializing_if = None)
        assert!(
            v.get("synonyms").is_none(),
            "absent synonyms must not appear in JSON"
        );
    }

    #[test]
    fn serde_round_trip_meta_response() {
        let meta = MetaResponse {
            cubes: vec![CubeMeta {
                name: "orders".into(),
                description: Some("Order facts".into()),
                measures: vec![MeasureMeta {
                    name: "count".into(),
                    agg_type: "count".into(),
                    description: None,
                    title: None,
                    synonyms: vec![],
                }],
                dimensions: vec![DimensionMeta {
                    name: "status".into(),
                    data_type: "string".into(),
                    description: None,
                    title: None,
                    synonyms: vec![],
                }],
            }],
        };
        let json = serde_json::to_string(&meta).unwrap();
        let meta2: MetaResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(meta2.cubes[0].name, "orders");
        assert_eq!(meta2.cubes[0].measures[0].agg_type, "count");
    }
}
