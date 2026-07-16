//! Pure eval types — no I/O, no DB derives.

use std::collections::HashMap;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

// ── Operator ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Op {
    In,
    NotIn,
    Eq,
    Ne,
    Contains,
    StartsWith,
    EndsWith,
    Gt,
    Gte,
    Lt,
    Lte,
    Matches,
    InSegment,
    Exists,
    NotExists,
    Before,
    After,
    SemverEq,
    SemverGt,
    SemverLt,
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "in" => Ok(Op::In),
            "not_in" => Ok(Op::NotIn),
            "eq" => Ok(Op::Eq),
            "ne" => Ok(Op::Ne),
            "contains" => Ok(Op::Contains),
            "starts_with" => Ok(Op::StartsWith),
            "ends_with" => Ok(Op::EndsWith),
            "gt" => Ok(Op::Gt),
            "gte" => Ok(Op::Gte),
            "lt" => Ok(Op::Lt),
            "lte" => Ok(Op::Lte),
            "matches" => Ok(Op::Matches),
            "in_segment" => Ok(Op::InSegment),
            "exists" => Ok(Op::Exists),
            "not_exists" => Ok(Op::NotExists),
            "before" => Ok(Op::Before),
            "after" => Ok(Op::After),
            "semver_eq" => Ok(Op::SemverEq),
            "semver_gt" => Ok(Op::SemverGt),
            "semver_lt" => Ok(Op::SemverLt),
            other => Err(format!("unknown operator: {other}")),
        }
    }
}

// ── ValueType ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValueType {
    Boolean,
    String,
    Number,
    Json,
}

impl FromStr for ValueType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "boolean" => Ok(ValueType::Boolean),
            "string" => Ok(ValueType::String),
            "number" => Ok(ValueType::Number),
            "json" => Ok(ValueType::Json),
            other => Err(format!("unknown value_type: {other}")),
        }
    }
}

// ── Eval types ────────────────────────────────────────────────────────────────

/// Caller-supplied evaluation context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalContext {
    pub targeting_key: String,
    #[serde(default)]
    pub attributes: serde_json::Map<String, Value>,
}

/// Result of evaluating a single flag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub flag_key: String,
    pub value: Value,
    pub variation_key: Option<String>,
    pub reason: Reason,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Reason {
    Disabled,
    TargetMatch,
    RuleMatch,
    Split,
    Default,
    Error,
    PrerequisiteFailed,
}

/// Compiled, versioned per-environment snapshot for evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlagSet {
    pub environment_id: Uuid,
    pub tenant_id: Uuid,
    pub env_key: String,
    pub version: i64,
    pub flags: HashMap<String, CompiledFlag>,
    pub segments: HashMap<String, CompiledSegment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledPrerequisite {
    pub flag_key: String,
    pub required_variation_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledFlag {
    pub flag_id: Uuid,
    pub flag_key: String,
    pub value_type: ValueType,
    pub is_active: bool,
    pub enabled: bool,
    /// All variations for this flag, keyed by variation_key. Always populated
    /// by the flagset loader; off/default variation keys index into this map.
    pub variations: HashMap<String, CompiledVariation>,
    pub off_variation_key: Option<String>,
    pub default_variation_key: Option<String>,
    pub targets: Vec<CompiledTarget>,
    pub rules: Vec<CompiledRule>,
    #[serde(default)]
    pub prerequisites: Vec<CompiledPrerequisite>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledVariation {
    pub variation_key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledTarget {
    pub target_attribute: String,
    pub target_value: String,
    pub variation_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RolloutWeight {
    pub variation_key: String,
    pub weight: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledRollout {
    pub attribute: Option<String>,
    pub variations: Vec<RolloutWeight>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledRule {
    pub rule_key: String,
    pub sort_order: i32,
    pub served_variation_key: Option<String>,
    pub rollout: Option<CompiledRollout>,
    pub clauses: Vec<CompiledClause>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledClause {
    pub attribute: String,
    pub operator: Op,
    pub value: String,
    pub is_negated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledSegment {
    pub segment_key: String,
    /// Explicit member list (matched against the context targeting_key).
    #[serde(default)]
    pub members: Vec<String>,
    pub clauses: Vec<CompiledClause>,
}
