//! Wire-format mirror of the soma-analytics query API request types.
//! Field names and serde attrs are the API contract; keep in lockstep with the
//! server (soma-analytics-semantic).

use serde::{Deserialize, Serialize};

/// Sort direction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Order {
    Asc,
    Desc,
}

/// Time-series granularity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Granularity {
    Day,
    Week,
    Month,
    Quarter,
    Year,
}

impl Granularity {
    /// The Postgres `date_trunc` unit string.
    pub fn trunc_unit(&self) -> &'static str {
        match self {
            Granularity::Day => "day",
            Granularity::Week => "week",
            Granularity::Month => "month",
            Granularity::Quarter => "quarter",
            Granularity::Year => "year",
        }
    }
}

/// Filter operator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterOp {
    Equals,
    NotEquals,
    Contains,
    Gt,
    Gte,
    Lt,
    Lte,
    Set,
    NotSet,
    InDateRange,
}

/// A filter applied to a member value. Values become bind parameters — never interpolated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    /// The `"cube.member"` reference being filtered, e.g. `"orders.status"`.
    pub member: String,
    /// The comparison operator.
    pub operator: FilterOp,
    /// Operand strings. Arity depends on the operator:
    /// - `Equals` / `NotEquals` / `Contains` — one or more values.
    /// - `Gt` / `Gte` / `Lt` / `Lte` — exactly one value.
    /// - `InDateRange` — exactly two values `[inclusive_lower, inclusive_upper]`.
    /// - `Set` / `NotSet` — ignored; leave empty.
    #[serde(default)]
    pub values: Vec<String>,
}

/// A time-dimension selector with optional granularity and date range.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeDimension {
    /// The `"cube.member"` reference to a `Time`-typed dimension used for
    /// bucketing and optional date-range filtering.
    pub member: String,
    /// The `date_trunc` bucket size for the grouped time column. The output
    /// column label is `"cube.member.grain"`, e.g. `"orders.created_at.month"`.
    pub granularity: Granularity,
    /// `[inclusive_lower, inclusive_upper]` date strings (`YYYY-MM-DD`).
    pub date_range: Option<[String; 2]>,
}

/// A semantic query — the public caller input.
///
/// All member strings use `"cube.member"` dotted notation. Members may
/// reference dimensions or measures on the root cube **or** on cubes that are
/// reachable via a single join hop from the root cube.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemanticQuery {
    /// Root cube name as declared in the model.
    pub cube: String,
    /// `"cube.measure"` references to aggregate.
    #[serde(default)]
    pub measures: Vec<String>,
    /// `"cube.dimension"` references to group by and project.
    #[serde(default)]
    pub dimensions: Vec<String>,
    /// User-supplied filters; all values become bind parameters.
    #[serde(default)]
    pub filters: Vec<Filter>,
    /// `"cube.segment"` references whose SQL predicates are inlined verbatim
    /// (model-authored, trusted content).
    #[serde(default)]
    pub segments: Vec<String>,
    /// Optional time-dimension bucketing and date-range filter.
    pub time_dimension: Option<TimeDimension>,
    /// Sort order. Each element is `("cube.member", direction)` where the
    /// member must be one of the selected `measures` or `dimensions`.
    #[serde(default)]
    pub order: Vec<(String, Order)>,
    /// Maximum rows to return; emitted as a `$N` bind (`LIMIT $N`).
    pub limit: Option<u32>,
    /// Row offset; emitted as a `$N` bind (`OFFSET $N`).
    pub offset: Option<u32>,
}

/// A security-scoped row filter, resolved from the embed token by the API layer.
///
/// `member` must resolve through the cube's dimension whitelist — the compiler
/// validates this before emitting the predicate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RowFilter {
    /// `"cube.member"` reference; must be a dimension on a cube reachable from
    /// the query's root cube.
    pub member: String,
    /// The exact value to match; bound as a text parameter, cast to the column
    /// type by Postgres.
    pub value: String,
}
