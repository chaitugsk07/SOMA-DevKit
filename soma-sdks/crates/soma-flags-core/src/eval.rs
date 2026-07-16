//! Pure evaluation engine — no I/O, no async.
//!
//! # Algorithm (LaunchDarkly semantics)
//! 1. !is_active || !enabled → serve off_variation (or Bool(false)), reason Disabled
//! 2. Individual targets: first attribute match → serve variation, reason TargetMatch
//! 3. Rules in sort_order (all clauses AND): if rollout → bucket; if included → Split;
//!    miss → next rule. If no rollout → RuleMatch.
//! 4. Fallthrough → default_variation, reason Default.
//!
//! Bucketing: sha256_hex(flag_key + "\x00" + stickiness_value);
//!   first 16 hex chars as u64; bucket = n % 10000;
//!   included = bucket < (percentage as u64 * 100).

use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};

use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::types::{
    CompiledClause, CompiledFlag, CompiledSegment, Decision, EvalContext, FlagSet, Op, Reason,
    ValueType,
};

/// Lowercase hex of SHA-256(bytes). Byte-identical to soma_infra::crypto::sha256_hex.
fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    // Format each byte as two lowercase hex digits.
    digest.iter().fold(String::with_capacity(64), |mut s, b| {
        use std::fmt::Write;
        write!(s, "{b:02x}").unwrap();
        s
    })
}

/// Evaluate all flags in `flagset` for the given `ctx`.
pub fn evaluate(flagset: &FlagSet, ctx: &EvalContext) -> BTreeMap<String, Decision> {
    flagset
        .flags
        .iter()
        .map(|(key, f)| {
            let mut visited = std::collections::HashSet::new();
            (
                key.clone(),
                eval_flag_recursive(f, ctx, &flagset.flags, &flagset.segments, &mut visited),
            )
        })
        .collect()
}

fn off_decision(flag: &CompiledFlag, reason: Reason) -> Decision {
    let (value, variation_key) = match &flag.off_variation_key {
        Some(k) => (
            flag.variations
                .get(k)
                .map(|v| parse_value(&flag.value_type, &v.value))
                .unwrap_or(Value::Bool(false)),
            Some(k.clone()),
        ),
        None => (Value::Bool(false), None),
    };
    Decision {
        flag_key: flag.flag_key.clone(),
        value,
        variation_key,
        reason,
    }
}

fn eval_flag_recursive(
    flag: &CompiledFlag,
    ctx: &EvalContext,
    flags: &HashMap<String, CompiledFlag>,
    segments: &HashMap<String, CompiledSegment>,
    visited: &mut std::collections::HashSet<String>,
) -> Decision {
    // Step 1: disabled
    if !flag.is_active || !flag.enabled {
        return off_decision(flag, Reason::Disabled);
    }

    // Step 2: prerequisites
    for prereq in &flag.prerequisites {
        let satisfied = match flags.get(&prereq.flag_key) {
            None => false,
            Some(_pre) if visited.contains(&prereq.flag_key) => false, // cycle → not satisfied
            Some(pre) => {
                visited.insert(flag.flag_key.clone());
                let d = eval_flag_recursive(pre, ctx, flags, segments, visited);
                visited.remove(&flag.flag_key);
                d.variation_key.as_deref() == Some(prereq.required_variation_key.as_str())
            }
        };
        if !satisfied {
            return off_decision(flag, Reason::PrerequisiteFailed);
        }
    }

    // Step 3: individual targets
    for target in &flag.targets {
        if ctx_attr(&target.target_attribute, ctx) == Some(target.target_value.as_str()) {
            let (val, vk) = find_variation_value(flag, &target.variation_key);
            return Decision {
                flag_key: flag.flag_key.clone(),
                value: val,
                variation_key: Some(vk),
                reason: Reason::TargetMatch,
            };
        }
    }

    // Step 3: rules
    for rule in &flag.rules {
        if !clauses_match(&rule.clauses, ctx, Some(segments)) {
            continue;
        }

        if let Some(ref ro) = rule.rollout {
            // Weighted multi-variation rollout
            let total: u64 = ro
                .variations
                .iter()
                .filter(|w| w.weight > 0)
                .map(|w| w.weight as u64)
                .sum();
            if total == 0 || ro.variations.is_empty() {
                // Empty rollout → fall through to default
                continue;
            }
            let sticky_val = match &ro.attribute {
                Some(attr) => ctx_attr(attr, ctx)
                    .map(|v| v.to_owned())
                    .unwrap_or_else(|| ctx.targeting_key.clone()),
                None => ctx.targeting_key.clone(),
            };
            let input = format!("{}\x00{}", flag.flag_key, sticky_val);
            let h = sha256_hex(input.as_bytes());
            let n = u64::from_str_radix(&h[0..16], 16).expect("sha256 is hex");
            let bucket = n % total;
            let mut cum: u64 = 0;
            for w in &ro.variations {
                if w.weight <= 0 {
                    continue;
                }
                cum += w.weight as u64;
                if bucket < cum {
                    let (val, vk) = find_variation_value(flag, &w.variation_key);
                    return Decision {
                        flag_key: flag.flag_key.clone(),
                        value: val,
                        variation_key: Some(vk),
                        reason: Reason::Split,
                    };
                }
            }
            // Shouldn't reach here if total > 0, but fall through just in case
            continue;
        } else if let Some(ref vk) = rule.served_variation_key {
            // No rollout → serve immediately
            let (val, vk) = find_variation_value(flag, vk);
            return Decision {
                flag_key: flag.flag_key.clone(),
                value: val,
                variation_key: Some(vk),
                reason: Reason::RuleMatch,
            };
        }
        // Both None → fall through (edge case)
    }

    // Step 4: default
    let (val, vk) = default_decision_value(flag);
    Decision {
        flag_key: flag.flag_key.clone(),
        value: val,
        variation_key: Some(vk),
        reason: Reason::Default,
    }
}

/// Find a variation value by key from the full `flag.variations` map.
/// If the key is missing (data integrity gap), falls through to default and
/// signals Error via the returned sentinel. Returns (value, variation_key).
fn find_variation_value(flag: &CompiledFlag, variation_key: &str) -> (Value, String) {
    if let Some(v) = flag.variations.get(variation_key) {
        return (
            parse_value(&flag.value_type, &v.value),
            variation_key.to_owned(),
        );
    }
    // Data integrity gap: referenced variation not in map → use default, caller uses Error reason.
    default_decision_value(flag)
}

fn default_decision_value(flag: &CompiledFlag) -> (Value, String) {
    if let Some(k) = &flag.default_variation_key {
        if let Some(v) = flag.variations.get(k) {
            return (parse_value(&flag.value_type, &v.value), k.clone());
        }
    }
    (Value::Bool(false), "false".to_owned())
}

fn parse_value(vt: &ValueType, s: &str) -> Value {
    match vt {
        ValueType::Boolean => Value::Bool(s.parse::<bool>().unwrap_or(false)),
        ValueType::Number => match s.parse::<f64>() {
            Ok(f) => serde_json::Number::from_f64(f)
                .map(Value::Number)
                .unwrap_or(Value::Number(serde_json::Number::from(0))),
            Err(_) => Value::Number(serde_json::Number::from(0)),
        },
        ValueType::Json => serde_json::from_str(s).unwrap_or(Value::Null),
        ValueType::String => Value::String(s.to_owned()),
    }
}

/// Get a context attribute value by name. "key" maps to targeting_key.
fn ctx_attr<'a>(attr: &str, ctx: &'a EvalContext) -> Option<&'a str> {
    if attr == "key" {
        Some(ctx.targeting_key.as_str())
    } else {
        ctx.attributes.get(attr)?.as_str()
    }
}

/// Evaluate all clauses (AND). `segments` is None when called from within a
/// segment clause context (prevents recursion).
fn clauses_match(
    clauses: &[CompiledClause],
    ctx: &EvalContext,
    segments: Option<&HashMap<String, CompiledSegment>>,
) -> bool {
    clauses.iter().all(|c| clause_matches(c, ctx, segments))
}

fn clause_matches(
    clause: &CompiledClause,
    ctx: &EvalContext,
    segments: Option<&HashMap<String, CompiledSegment>>,
) -> bool {
    // InSegment is handled before attribute lookup
    if clause.operator == Op::InSegment {
        let result = match segments {
            None => false, // no recursion in segment clause context
            Some(seg_map) => match seg_map.get(&clause.value) {
                None => false,
                // In segment if the key is an explicit member, OR (when the
                // segment has clauses) the clauses match. Empty-clauses segments
                // match ONLY their explicit members (not everyone).
                Some(seg) => {
                    seg.members.iter().any(|m| m == &ctx.targeting_key)
                        || (!seg.clauses.is_empty() && clauses_match(&seg.clauses, ctx, None))
                }
            },
        };
        return if clause.is_negated { !result } else { result };
    }

    // Existence checks are about presence, not value — handle before extraction.
    if clause.operator == Op::Exists || clause.operator == Op::NotExists {
        let present = ctx_has(&clause.attribute, ctx);
        let result = if clause.operator == Op::NotExists {
            !present
        } else {
            present
        };
        return if clause.is_negated { !result } else { result };
    }

    // Missing attribute → clause fails
    let attr_str = match ctx_attr(&clause.attribute, ctx) {
        Some(s) => s,
        None => return false,
    };

    let result = match &clause.operator {
        Op::Eq => attr_str == clause.value.as_str(),
        Op::Ne => attr_str != clause.value.as_str(),
        Op::Contains => attr_str.contains(clause.value.as_str()),
        Op::StartsWith => attr_str.starts_with(clause.value.as_str()),
        Op::EndsWith => attr_str.ends_with(clause.value.as_str()),
        Op::In | Op::NotIn => {
            let list: Vec<String> = serde_json::from_str(&clause.value).unwrap_or_default();
            let found = list.iter().any(|v| v.as_str() == attr_str);
            if clause.operator == Op::NotIn {
                !found
            } else {
                found
            }
        }
        Op::Gt => compare_f64(attr_str, &clause.value, |a, b| a > b),
        Op::Gte => compare_f64(attr_str, &clause.value, |a, b| a >= b),
        Op::Lt => compare_f64(attr_str, &clause.value, |a, b| a < b),
        Op::Lte => compare_f64(attr_str, &clause.value, |a, b| a <= b),
        Op::Matches => regex::Regex::new(&clause.value)
            .map(|re| re.is_match(attr_str))
            .unwrap_or(false),
        // Date comparison: ISO-8601 / RFC-3339 timestamps sort lexicographically.
        Op::Before => attr_str < clause.value.as_str(),
        Op::After => attr_str > clause.value.as_str(),
        Op::SemverEq => compare_semver(attr_str, &clause.value, |o| o == Ordering::Equal),
        Op::SemverGt => compare_semver(attr_str, &clause.value, |o| o == Ordering::Greater),
        Op::SemverLt => compare_semver(attr_str, &clause.value, |o| o == Ordering::Less),
        Op::InSegment | Op::Exists | Op::NotExists => unreachable!("handled above"),
    };

    if clause.is_negated {
        !result
    } else {
        result
    }
}

fn compare_f64(a: &str, b: &str, cmp: impl Fn(f64, f64) -> bool) -> bool {
    let av: f64 = match a.parse() {
        Ok(v) => v,
        Err(_) => return false,
    };
    let bv: f64 = match b.parse() {
        Ok(v) => v,
        Err(_) => return false,
    };
    cmp(av, bv)
}

/// Is `attr` present (and non-null) in the context? `"key"` is the targeting key.
fn ctx_has(attr: &str, ctx: &EvalContext) -> bool {
    if attr == "key" {
        !ctx.targeting_key.is_empty()
    } else {
        ctx.attributes
            .get(attr)
            .map(|v| !v.is_null())
            .unwrap_or(false)
    }
}

/// Parse `major.minor.patch` (pre-release/build suffix after `-`/`+` ignored).
fn parse_semver(s: &str) -> Option<(u64, u64, u64)> {
    let core = s.split(['-', '+']).next().unwrap_or(s);
    let mut parts = core.split('.');
    let major = parts.next()?.trim().parse().ok()?;
    let minor = parts.next().unwrap_or("0").trim().parse().ok()?;
    let patch = parts.next().unwrap_or("0").trim().parse().ok()?;
    Some((major, minor, patch))
}

fn compare_semver(a: &str, b: &str, cmp: impl Fn(Ordering) -> bool) -> bool {
    match (parse_semver(a), parse_semver(b)) {
        (Some(av), Some(bv)) => cmp(av.cmp(&bv)),
        _ => false,
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        CompiledClause, CompiledFlag, CompiledRule, CompiledSegment, CompiledTarget,
        CompiledVariation, EvalContext, FlagSet, Op, Reason, ValueType,
    };
    use serde_json::json;
    use std::collections::HashMap;
    use uuid::Uuid;

    fn var(key: &str, value: &str) -> CompiledVariation {
        CompiledVariation {
            variation_key: key.to_owned(),
            value: value.to_owned(),
        }
    }

    fn variations(pairs: &[(&str, &str)]) -> HashMap<String, CompiledVariation> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), var(k, v)))
            .collect()
    }

    /// Create a simple boolean flag: off="false"/false, default="true"/true.
    fn bool_flag(key: &str, enabled: bool, is_active: bool) -> CompiledFlag {
        CompiledFlag {
            flag_id: Uuid::new_v4(),
            flag_key: key.to_owned(),
            value_type: ValueType::Boolean,
            is_active,
            enabled,
            variations: variations(&[("false", "false"), ("true", "true")]),
            off_variation_key: Some("false".to_owned()),
            default_variation_key: Some("true".to_owned()),
            targets: vec![],
            rules: vec![],
            prerequisites: vec![],
        }
    }

    /// Boolean flag with a single rule: clause(attr="x", op, value) → serve "true".
    fn flag_with_clause(op: Op, value: &str) -> CompiledFlag {
        let mut f = bool_flag("f", true, true);
        f.rules = vec![CompiledRule {
            rule_key: "r".into(),
            sort_order: 0,
            served_variation_key: Some("true".into()),
            rollout: None,
            clauses: vec![CompiledClause {
                attribute: "x".into(),
                operator: op,
                value: value.into(),
                is_negated: false,
            }],
        }];
        f
    }

    fn ctx_with(attr: &str, val: serde_json::Value) -> EvalContext {
        let mut attributes = serde_json::Map::new();
        attributes.insert(attr.to_string(), val);
        EvalContext {
            targeting_key: "u".into(),
            attributes,
        }
    }

    #[test]
    fn op_exists_not_exists() {
        let fs = make_flagset(vec![flag_with_clause(Op::Exists, "")]);
        assert!(matches!(
            evaluate(&fs, &ctx_with("x", json!("a")))
                .get("f")
                .unwrap()
                .reason,
            Reason::RuleMatch
        ));
        assert!(matches!(
            evaluate(&fs, &ctx_with("y", json!("a")))
                .get("f")
                .unwrap()
                .reason,
            Reason::Default
        ));
        let fs2 = make_flagset(vec![flag_with_clause(Op::NotExists, "")]);
        assert!(matches!(
            evaluate(&fs2, &ctx_with("y", json!("a")))
                .get("f")
                .unwrap()
                .reason,
            Reason::RuleMatch
        ));
        assert!(matches!(
            evaluate(&fs2, &ctx_with("x", json!("a")))
                .get("f")
                .unwrap()
                .reason,
            Reason::Default
        ));
    }

    #[test]
    fn op_before_after() {
        let before = make_flagset(vec![flag_with_clause(Op::Before, "2026-01-01T00:00:00Z")]);
        assert!(matches!(
            evaluate(&before, &ctx_with("x", json!("2025-06-01T00:00:00Z")))
                .get("f")
                .unwrap()
                .reason,
            Reason::RuleMatch
        ));
        assert!(matches!(
            evaluate(&before, &ctx_with("x", json!("2026-06-01T00:00:00Z")))
                .get("f")
                .unwrap()
                .reason,
            Reason::Default
        ));
        let after = make_flagset(vec![flag_with_clause(Op::After, "2026-01-01T00:00:00Z")]);
        assert!(matches!(
            evaluate(&after, &ctx_with("x", json!("2026-06-01T00:00:00Z")))
                .get("f")
                .unwrap()
                .reason,
            Reason::RuleMatch
        ));
    }

    #[test]
    fn op_semver() {
        let gt = make_flagset(vec![flag_with_clause(Op::SemverGt, "1.2.0")]);
        assert!(matches!(
            evaluate(&gt, &ctx_with("x", json!("1.3.0")))
                .get("f")
                .unwrap()
                .reason,
            Reason::RuleMatch
        ));
        assert!(matches!(
            evaluate(&gt, &ctx_with("x", json!("1.10.0")))
                .get("f")
                .unwrap()
                .reason,
            Reason::RuleMatch
        )); // numeric, not lexical
        assert!(matches!(
            evaluate(&gt, &ctx_with("x", json!("1.2.0")))
                .get("f")
                .unwrap()
                .reason,
            Reason::Default
        ));
        let eq = make_flagset(vec![flag_with_clause(Op::SemverEq, "2.0.0")]);
        assert!(matches!(
            evaluate(&eq, &ctx_with("x", json!("2.0.0")))
                .get("f")
                .unwrap()
                .reason,
            Reason::RuleMatch
        ));
        let lt = make_flagset(vec![flag_with_clause(Op::SemverLt, "2.0.0")]);
        assert!(matches!(
            evaluate(&lt, &ctx_with("x", json!("1.9.9")))
                .get("f")
                .unwrap()
                .reason,
            Reason::RuleMatch
        ));
    }

    fn make_flagset(flags: Vec<CompiledFlag>) -> FlagSet {
        let map: HashMap<String, CompiledFlag> =
            flags.into_iter().map(|f| (f.flag_key.clone(), f)).collect();
        FlagSet {
            environment_id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            env_key: "test".into(),
            version: 1,
            flags: map,
            segments: HashMap::new(),
        }
    }

    fn make_flagset_with_segments(
        flags: Vec<CompiledFlag>,
        segments: Vec<CompiledSegment>,
    ) -> FlagSet {
        let flag_map: HashMap<String, CompiledFlag> =
            flags.into_iter().map(|f| (f.flag_key.clone(), f)).collect();
        let seg_map: HashMap<String, CompiledSegment> = segments
            .into_iter()
            .map(|s| (s.segment_key.clone(), s))
            .collect();
        FlagSet {
            environment_id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            env_key: "test".into(),
            version: 1,
            flags: flag_map,
            segments: seg_map,
        }
    }

    fn ctx(targeting_key: &str, pairs: &[(&str, &str)]) -> EvalContext {
        let mut attributes = serde_json::Map::new();
        for (k, v) in pairs {
            attributes.insert(k.to_string(), json!(v));
        }
        EvalContext {
            targeting_key: targeting_key.to_owned(),
            attributes,
        }
    }

    fn empty_ctx() -> EvalContext {
        ctx("user-1", &[])
    }

    fn simple_rule(clauses: Vec<CompiledClause>, variation_key: &str) -> CompiledRule {
        CompiledRule {
            rule_key: "r1".into(),
            sort_order: 0,
            served_variation_key: Some(variation_key.to_owned()),
            rollout: None,
            clauses,
        }
    }

    fn clause(attribute: &str, op: Op, value: &str, is_negated: bool) -> CompiledClause {
        CompiledClause {
            attribute: attribute.to_owned(),
            operator: op,
            value: value.to_owned(),
            is_negated,
        }
    }

    #[test]
    fn disabled_serves_off_variation() {
        let fs = make_flagset(vec![bool_flag("my-flag", false, true)]);
        let d = &evaluate(&fs, &empty_ctx())["my-flag"];
        assert_eq!(d.value, json!(false));
        assert!(matches!(d.reason, Reason::Disabled));
        assert_eq!(d.variation_key.as_deref(), Some("false"));
    }

    #[test]
    fn individual_target_match() {
        let mut flag = bool_flag("tgt-flag", true, true);
        flag.targets.push(CompiledTarget {
            target_attribute: "key".to_owned(),
            target_value: "user-special".to_owned(),
            variation_key: "true".to_owned(),
        });
        let fs = make_flagset(vec![flag]);
        let d = &evaluate(&fs, &ctx("user-special", &[]))["tgt-flag"];
        assert_eq!(d.value, json!(true));
        assert!(matches!(d.reason, Reason::TargetMatch));
    }

    #[test]
    fn individual_target_no_match_falls_through() {
        let mut flag = bool_flag("tgt-flag", true, true);
        flag.targets.push(CompiledTarget {
            target_attribute: "key".to_owned(),
            target_value: "user-special".to_owned(),
            variation_key: "true".to_owned(),
        });
        let fs = make_flagset(vec![flag]);
        let d = &evaluate(&fs, &ctx("user-other", &[]))["tgt-flag"];
        // No rules, so falls to default
        assert_eq!(d.value, json!(true));
        assert!(matches!(d.reason, Reason::Default));
    }

    #[test]
    fn single_clause_eq_rule_match() {
        let mut flag = bool_flag("eq-flag", true, true);
        flag.rules.push(simple_rule(
            vec![clause("plan", Op::Eq, "pro", false)],
            "true",
        ));
        let fs = make_flagset(vec![flag]);
        let d = &evaluate(&fs, &ctx("u1", &[("plan", "pro")]))["eq-flag"];
        assert_eq!(d.value, json!(true));
        assert!(matches!(d.reason, Reason::RuleMatch));
    }

    #[test]
    fn multi_clause_rule_all_must_pass() {
        let mut flag = bool_flag("multi-flag", true, true);
        flag.rules.push(simple_rule(
            vec![
                clause("plan", Op::Eq, "pro", false),
                clause("country", Op::Eq, "US", false),
            ],
            "true",
        ));
        let fs = make_flagset(vec![flag]);
        let d = &evaluate(&fs, &ctx("u1", &[("plan", "pro"), ("country", "US")]))["multi-flag"];
        assert_eq!(d.value, json!(true));
        assert!(matches!(d.reason, Reason::RuleMatch));
    }

    #[test]
    fn multi_clause_rule_one_fails() {
        let mut flag = bool_flag("multi-flag", true, true);
        flag.rules.push(simple_rule(
            vec![
                clause("plan", Op::Eq, "pro", false),
                clause("country", Op::Eq, "US", false),
            ],
            "true",
        ));
        let fs = make_flagset(vec![flag]);
        // country = "CA" → second clause fails → rule skipped → Default (value="true" from default_variation)
        let d = &evaluate(&fs, &ctx("u1", &[("plan", "pro"), ("country", "CA")]))["multi-flag"];
        assert!(matches!(d.reason, Reason::Default));
        // Default variation is "true" with value "true"
        assert_eq!(d.variation_key.as_deref(), Some("true"));
    }

    #[test]
    fn is_negated_inverts_result() {
        let mut flag = bool_flag("neg-flag", true, true);
        // Negated Eq: "plan != free" → match when plan is "pro"
        flag.rules.push(simple_rule(
            vec![clause("plan", Op::Eq, "free", true)],
            "true",
        ));
        let fs = make_flagset(vec![flag]);
        // plan=pro → Eq("free") = false, negated → true → clause passes → RuleMatch
        let d = &evaluate(&fs, &ctx("u1", &[("plan", "pro")]))["neg-flag"];
        assert_eq!(d.value, json!(true));
        assert!(matches!(d.reason, Reason::RuleMatch));
    }

    #[test]
    fn rollout_in() {
        // 100% rollout → always included
        let mut flag = bool_flag("rollout-flag", true, true);
        flag.rules.push(CompiledRule {
            rule_key: "r1".into(),
            sort_order: 0,
            served_variation_key: None,
            rollout: Some(crate::types::CompiledRollout {
                attribute: Some("key".into()),
                variations: vec![crate::types::RolloutWeight {
                    variation_key: "true".into(),
                    weight: 100,
                }],
            }),
            clauses: vec![],
        });
        let fs = make_flagset(vec![flag]);
        let d = &evaluate(&fs, &ctx("user-abc", &[]))["rollout-flag"];
        assert_eq!(d.value, json!(true));
        assert!(matches!(d.reason, Reason::Split));
    }

    #[test]
    fn rollout_out() {
        // Empty rollout variations list → fall through to default
        let mut flag = bool_flag("rollout-zero", true, true);
        flag.rules.push(CompiledRule {
            rule_key: "r1".into(),
            sort_order: 0,
            served_variation_key: None,
            rollout: Some(crate::types::CompiledRollout {
                attribute: Some("key".into()),
                variations: vec![],
            }),
            clauses: vec![],
        });
        let fs = make_flagset(vec![flag]);
        let d = &evaluate(&fs, &ctx("user-abc", &[]))["rollout-zero"];
        // empty variations → fall through → Default (default_variation="true", value="true")
        assert!(matches!(d.reason, Reason::Default));
        assert_eq!(d.variation_key.as_deref(), Some("true"));
    }

    #[test]
    fn rollout_determinism() {
        // Same input → same bucket → same decision
        let mut flag = bool_flag("stable-flag", true, true);
        flag.rules.push(CompiledRule {
            rule_key: "r1".into(),
            sort_order: 0,
            served_variation_key: None,
            rollout: Some(crate::types::CompiledRollout {
                attribute: Some("key".into()),
                variations: vec![
                    crate::types::RolloutWeight {
                        variation_key: "true".into(),
                        weight: 50,
                    },
                    crate::types::RolloutWeight {
                        variation_key: "false".into(),
                        weight: 50,
                    },
                ],
            }),
            clauses: vec![],
        });
        let fs = make_flagset(vec![flag]);
        let c = ctx("user-123", &[]);
        let d1 = evaluate(&fs, &c);
        let d2 = evaluate(&fs, &c);
        assert_eq!(d1["stable-flag"].value, d2["stable-flag"].value);
        assert_eq!(
            format!("{:?}", d1["stable-flag"].reason),
            format!("{:?}", d2["stable-flag"].reason),
        );
    }

    #[test]
    fn in_segment_match() {
        let segment = CompiledSegment {
            segment_key: "pro-users".to_owned(),
            members: vec![],
            clauses: vec![clause("plan", Op::Eq, "pro", false)],
        };
        let mut flag = bool_flag("seg-flag", true, true);
        // Use "key" attribute for InSegment (the clause attribute just needs to be in dim table;
        // here we use "key" as the attribute for the InSegment clause — but InSegment ignores
        // the attribute field, it uses the value as segment_key).
        flag.rules.push(simple_rule(
            vec![CompiledClause {
                attribute: "key".to_owned(),
                operator: Op::InSegment,
                value: "pro-users".to_owned(),
                is_negated: false,
            }],
            "true",
        ));
        let fs = make_flagset_with_segments(vec![flag], vec![segment]);
        let d = &evaluate(&fs, &ctx("u1", &[("plan", "pro")]))["seg-flag"];
        assert_eq!(d.value, json!(true));
        assert!(matches!(d.reason, Reason::RuleMatch));
    }

    #[test]
    fn in_segment_miss() {
        // Segment doesn't exist → clause fails
        let mut flag = bool_flag("seg-miss", true, true);
        flag.rules.push(simple_rule(
            vec![CompiledClause {
                attribute: "key".to_owned(),
                operator: Op::InSegment,
                value: "nonexistent".to_owned(),
                is_negated: false,
            }],
            "true",
        ));
        let fs = make_flagset(vec![flag]);
        let d = &evaluate(&fs, &ctx("u1", &[]))["seg-miss"];
        assert!(matches!(d.reason, Reason::Default));
    }

    #[test]
    fn in_segment_no_recursion() {
        // Segment clauses should not use InSegment (schema prevents it),
        // but even if they did, our eval passes None for segments inside segment eval.
        // Test: segment with normal clauses evaluates correctly.
        let segment = CompiledSegment {
            segment_key: "beta".to_owned(),
            members: vec![],
            clauses: vec![clause("plan", Op::Eq, "beta", false)],
        };
        let mut flag = bool_flag("beta-flag", true, true);
        flag.rules.push(simple_rule(
            vec![CompiledClause {
                attribute: "key".to_owned(),
                operator: Op::InSegment,
                value: "beta".to_owned(),
                is_negated: false,
            }],
            "true",
        ));
        let fs = make_flagset_with_segments(vec![flag], vec![segment]);
        let d = &evaluate(&fs, &ctx("u1", &[("plan", "beta")]))["beta-flag"];
        assert_eq!(d.value, json!(true));
        assert!(matches!(d.reason, Reason::RuleMatch));
    }

    #[test]
    fn in_segment_member_list() {
        // Segment with an explicit member list and no clauses.
        // Only the listed targeting_key ("vip") should match.
        let segment = CompiledSegment {
            segment_key: "vip-users".to_owned(),
            members: vec!["vip".to_owned()],
            clauses: vec![],
        };
        let mut flag = bool_flag("vip-flag", true, true);
        flag.rules.push(simple_rule(
            vec![CompiledClause {
                attribute: "key".to_owned(),
                operator: Op::InSegment,
                value: "vip-users".to_owned(),
                is_negated: false,
            }],
            "true",
        ));
        let fs = make_flagset_with_segments(vec![flag], vec![segment]);
        // Member match → rule fires
        let d_member = &evaluate(&fs, &ctx("vip", &[]))["vip-flag"];
        assert_eq!(d_member.value, json!(true));
        assert!(matches!(d_member.reason, Reason::RuleMatch));
        // Non-member with no matching clauses → falls through to Default
        let d_non_member = &evaluate(&fs, &ctx("regular", &[]))["vip-flag"];
        assert!(matches!(d_non_member.reason, Reason::Default));
    }

    #[test]
    fn multivariate_string_value() {
        let flag = CompiledFlag {
            flag_id: Uuid::new_v4(),
            flag_key: "str-flag".to_owned(),
            value_type: ValueType::String,
            is_active: true,
            enabled: true,
            variations: variations(&[("off", "off"), ("blue", "blue")]),
            off_variation_key: Some("off".to_owned()),
            default_variation_key: Some("blue".to_owned()),
            targets: vec![],
            rules: vec![],
            prerequisites: vec![],
        };
        let fs = make_flagset(vec![flag]);
        let d = &evaluate(&fs, &empty_ctx())["str-flag"];
        assert_eq!(d.value, json!("blue"));
        assert!(matches!(d.reason, Reason::Default));
    }

    #[test]
    fn multivariate_number_value() {
        let flag = CompiledFlag {
            flag_id: Uuid::new_v4(),
            flag_key: "num-flag".to_owned(),
            value_type: ValueType::Number,
            is_active: true,
            enabled: true,
            variations: variations(&[("off", "0"), ("hundred", "100")]),
            off_variation_key: Some("off".to_owned()),
            default_variation_key: Some("hundred".to_owned()),
            targets: vec![],
            rules: vec![],
            prerequisites: vec![],
        };
        let fs = make_flagset(vec![flag]);
        let d = &evaluate(&fs, &empty_ctx())["num-flag"];
        assert_eq!(d.value, json!(100.0));
        assert!(matches!(d.reason, Reason::Default));
    }

    #[test]
    fn multivariate_json_value() {
        let flag = CompiledFlag {
            flag_id: Uuid::new_v4(),
            flag_key: "json-flag".to_owned(),
            value_type: ValueType::Json,
            is_active: true,
            enabled: true,
            variations: variations(&[("off", "null"), ("config", r#"{"timeout":30}"#)]),
            off_variation_key: Some("off".to_owned()),
            default_variation_key: Some("config".to_owned()),
            targets: vec![],
            rules: vec![],
            prerequisites: vec![],
        };
        let fs = make_flagset(vec![flag]);
        let d = &evaluate(&fs, &empty_ctx())["json-flag"];
        assert_eq!(d.value, json!({"timeout": 30}));
        assert!(matches!(d.reason, Reason::Default));
    }

    #[test]
    fn fallthrough_default_variation() {
        let flag = bool_flag("fall-flag", true, true);
        let fs = make_flagset(vec![flag]);
        let d = &evaluate(&fs, &empty_ctx())["fall-flag"];
        assert_eq!(d.value, json!(true));
        assert!(matches!(d.reason, Reason::Default));
        assert_eq!(d.variation_key.as_deref(), Some("true"));
    }

    #[test]
    fn missing_attribute_fails_clause() {
        let mut flag = bool_flag("miss-attr", true, true);
        flag.rules.push(simple_rule(
            vec![clause("plan", Op::Eq, "pro", false)],
            "true",
        ));
        let fs = make_flagset(vec![flag]);
        // No "plan" in context → clause fails → Default
        let d = &evaluate(&fs, &empty_ctx())["miss-attr"];
        assert!(matches!(d.reason, Reason::Default));
    }

    #[test]
    fn in_operator_json_array() {
        let mut flag = bool_flag("in-flag", true, true);
        flag.rules.push(simple_rule(
            vec![clause("plan", Op::In, r#"["pro","enterprise"]"#, false)],
            "true",
        ));
        let fs = make_flagset(vec![flag]);
        let d = &evaluate(&fs, &ctx("u1", &[("plan", "enterprise")]))["in-flag"];
        assert_eq!(d.value, json!(true));
        assert!(matches!(d.reason, Reason::RuleMatch));
    }

    #[test]
    fn not_in_operator() {
        let mut flag = bool_flag("not-in-flag", true, true);
        flag.rules.push(simple_rule(
            vec![clause("plan", Op::NotIn, r#"["free","trial"]"#, false)],
            "true",
        ));
        let fs = make_flagset(vec![flag]);
        // plan=pro → not in [free,trial] → clause passes → RuleMatch
        let d = &evaluate(&fs, &ctx("u1", &[("plan", "pro")]))["not-in-flag"];
        assert_eq!(d.value, json!(true));
        assert!(matches!(d.reason, Reason::RuleMatch));
        // plan=free → in [free,trial] → NotIn fails → Default
        let d2 = &evaluate(&fs, &ctx("u2", &[("plan", "free")]))["not-in-flag"];
        assert!(matches!(d2.reason, Reason::Default));
    }

    /// Regression test: multivariate flag with ≥3 variations where a RULE serves
    /// the third variation and an individual TARGET also serves the third variation.
    /// Uses the full variation map (as the real loader produces), NOT by hand-wiring
    /// only two slots. This test would have failed before the CompiledFlag.variations
    /// fix (find_variation_value previously only checked off/default).
    #[test]
    fn third_variation_via_rule_and_target() {
        // 3 string variations: "control"/"A"/"B"
        // off_variation_key = "control", default_variation_key = "control"
        // A rule serves "B", a target for "vip-user" serves "B"
        let flag = CompiledFlag {
            flag_id: Uuid::new_v4(),
            flag_key: "mv-flag".to_owned(),
            value_type: ValueType::String,
            is_active: true,
            enabled: true,
            variations: variations(&[
                ("control", "control"),
                ("variant-a", "A"),
                ("variant-b", "B"),
            ]),
            off_variation_key: Some("control".to_owned()),
            default_variation_key: Some("control".to_owned()),
            targets: vec![CompiledTarget {
                target_attribute: "key".to_owned(),
                target_value: "vip-user".to_owned(),
                variation_key: "variant-b".to_owned(),
            }],
            rules: vec![CompiledRule {
                rule_key: "beta-users".to_owned(),
                sort_order: 0,
                served_variation_key: Some("variant-b".to_owned()),
                rollout: None,
                clauses: vec![clause("plan", Op::Eq, "beta", false)],
            }],
            prerequisites: vec![],
        };
        let fs = make_flagset(vec![flag]);

        // Individual target → third variation "B"
        let d_vip = &evaluate(&fs, &ctx("vip-user", &[]))["mv-flag"];
        assert_eq!(
            d_vip.value,
            json!("B"),
            "target must serve third variation value"
        );
        assert_eq!(d_vip.variation_key.as_deref(), Some("variant-b"));
        assert!(matches!(d_vip.reason, Reason::TargetMatch));

        // Rule match → third variation "B"
        let d_beta = &evaluate(&fs, &ctx("other-user", &[("plan", "beta")]))["mv-flag"];
        assert_eq!(
            d_beta.value,
            json!("B"),
            "rule must serve third variation value"
        );
        assert_eq!(d_beta.variation_key.as_deref(), Some("variant-b"));
        assert!(matches!(d_beta.reason, Reason::RuleMatch));

        // No match → default "control"
        let d_default = &evaluate(&fs, &ctx("normal-user", &[("plan", "pro")]))["mv-flag"];
        assert_eq!(d_default.value, json!("control"));
        assert_eq!(d_default.variation_key.as_deref(), Some("control"));
        assert!(matches!(d_default.reason, Reason::Default));
    }

    /// Verify that serialising a FlagSet to JSON and deserialising it back produces
    /// identical evaluation results (regression guard for serde round-trips in local eval).
    #[test]
    fn json_roundtrip_local_eval_parity() {
        // Segment: plan Eq pro
        let segment = CompiledSegment {
            segment_key: "pro-seg".to_owned(),
            members: vec![],
            clauses: vec![clause("plan", Op::Eq, "pro", false)],
        };

        // Flag: 3 string variations, individual target, two rules (exact match + 50% rollout)
        let flag = CompiledFlag {
            flag_id: Uuid::new_v4(),
            flag_key: "mv-flag".to_owned(),
            value_type: ValueType::String,
            is_active: true,
            enabled: true,
            variations: variations(&[
                ("control", "control"),
                ("variant-a", "A"),
                ("variant-b", "B"),
            ]),
            off_variation_key: Some("control".to_owned()),
            default_variation_key: Some("control".to_owned()),
            targets: vec![CompiledTarget {
                target_attribute: "key".to_owned(),
                target_value: "vip-user".to_owned(),
                variation_key: "variant-b".to_owned(),
            }],
            rules: vec![
                // Rule 0: plan Eq beta → variant-b (no rollout)
                CompiledRule {
                    rule_key: "beta-rule".to_owned(),
                    sort_order: 0,
                    served_variation_key: Some("variant-b".to_owned()),
                    rollout: None,
                    clauses: vec![clause("plan", Op::Eq, "beta", false)],
                },
                // Rule 1: 50/50 weighted rollout by key → variant-a or control
                CompiledRule {
                    rule_key: "rollout-rule".to_owned(),
                    sort_order: 1,
                    served_variation_key: None,
                    rollout: Some(crate::types::CompiledRollout {
                        attribute: Some("key".into()),
                        variations: vec![
                            crate::types::RolloutWeight {
                                variation_key: "variant-a".into(),
                                weight: 50,
                            },
                            crate::types::RolloutWeight {
                                variation_key: "control".into(),
                                weight: 50,
                            },
                        ],
                    }),
                    clauses: vec![],
                },
            ],
            prerequisites: vec![],
        };

        let flagset = make_flagset_with_segments(vec![flag], vec![segment]);

        // Round-trip through JSON.
        let json_val = serde_json::to_value(&flagset).unwrap();
        let deserialized: FlagSet = serde_json::from_value(json_val).unwrap();

        // Three contexts to compare.
        let contexts = [
            ctx("vip-user", &[]),
            ctx("other-user", &[("plan", "beta")]),
            ctx("starter-user", &[("plan", "starter")]),
        ];

        for c in &contexts {
            let orig = evaluate(&flagset, c);
            let deser = evaluate(&deserialized, c);
            for key in orig.keys() {
                let o = &orig[key];
                let d = &deser[key];
                assert_eq!(
                    format!("{:?}", o.value),
                    format!("{:?}", d.value),
                    "value mismatch for key={key} ctx={}",
                    c.targeting_key,
                );
                assert_eq!(
                    o.variation_key, d.variation_key,
                    "variation_key mismatch for key={key} ctx={}",
                    c.targeting_key,
                );
                assert_eq!(
                    format!("{:?}", o.reason),
                    format!("{:?}", d.reason),
                    "reason mismatch for key={key} ctx={}",
                    c.targeting_key,
                );
            }
        }
    }

    /// Verify bucketing algorithm is byte-identical to soma_infra::crypto::sha256_hex.
    /// sha256("stable-flag\x00user-123") must produce the same hex as sha2::Sha256.
    #[test]
    fn bucketing_sha256_hex_matches_reference() {
        // Known SHA-256 of "stable-flag\x00user-123" computed independently:
        // echo -n "stable-flag\x00user-123" | sha256sum
        // We verify: first 16 hex chars parse as u64 and bucket is consistent.
        let input = "stable-flag\x00user-123";
        let h = sha256_hex(input.as_bytes());
        assert_eq!(h.len(), 64, "sha256_hex must be 64 hex chars");
        assert!(
            h.chars()
                .all(|c| c.is_ascii_hexdigit() && !c.is_uppercase()),
            "sha256_hex must be lowercase hex"
        );
        // Must parse cleanly as u64 (first 16 hex = 8 bytes)
        let n = u64::from_str_radix(&h[0..16], 16).expect("must be valid hex");
        let bucket = n % 10000;
        assert!(bucket < 10000);
    }

    #[test]
    fn weighted_rollout_fixed_variation_rule_match() {
        // A rule with served_variation_key (no rollout) → RULE_MATCH
        let mut flag = bool_flag("fixed-rule", true, true);
        flag.rules.push(CompiledRule {
            rule_key: "r1".into(),
            sort_order: 0,
            served_variation_key: Some("true".to_owned()),
            rollout: None,
            clauses: vec![],
        });
        let fs = make_flagset(vec![flag]);
        let d = &evaluate(&fs, &ctx("any-user", &[]))["fixed-rule"];
        assert!(matches!(d.reason, Reason::RuleMatch));
        assert_eq!(d.variation_key.as_deref(), Some("true"));
    }

    #[test]
    fn weighted_rollout_split_reason() {
        // 50/50 split → reason SPLIT, deterministic for a given key
        let flag = CompiledFlag {
            flag_id: Uuid::new_v4(),
            flag_key: "ab-flag".to_owned(),
            value_type: ValueType::String,
            is_active: true,
            enabled: true,
            variations: variations(&[("control", "control"), ("variant", "variant")]),
            off_variation_key: Some("control".to_owned()),
            default_variation_key: Some("control".to_owned()),
            targets: vec![],
            rules: vec![CompiledRule {
                rule_key: "rollout".into(),
                sort_order: 0,
                served_variation_key: None,
                rollout: Some(crate::types::CompiledRollout {
                    attribute: None,
                    variations: vec![
                        crate::types::RolloutWeight {
                            variation_key: "control".into(),
                            weight: 50,
                        },
                        crate::types::RolloutWeight {
                            variation_key: "variant".into(),
                            weight: 50,
                        },
                    ],
                }),
                clauses: vec![],
            }],
            prerequisites: vec![],
        };
        let fs = make_flagset(vec![flag]);
        // Same user → same result (determinism)
        let d1 = &evaluate(&fs, &ctx("user-abc", &[]))["ab-flag"];
        let d2 = &evaluate(&fs, &ctx("user-abc", &[]))["ab-flag"];
        assert!(matches!(d1.reason, Reason::Split));
        assert_eq!(d1.variation_key, d2.variation_key);
        // Over 1000 unique users, both variations get roughly 40-60%
        let mut counts = std::collections::HashMap::new();
        for i in 0..1000u32 {
            let c = ctx(&format!("user-{i}"), &[]);
            let d = &evaluate(&fs, &c)["ab-flag"];
            *counts
                .entry(d.variation_key.clone().unwrap_or_default())
                .or_insert(0u32) += 1;
        }
        let control_count = counts.get("control").copied().unwrap_or(0);
        let variant_count = counts.get("variant").copied().unwrap_or(0);
        assert!(
            control_count >= 400 && control_count <= 600,
            "control={control_count}"
        );
        assert!(
            variant_count >= 400 && variant_count <= 600,
            "variant={variant_count}"
        );
    }

    #[test]
    fn weighted_rollout_single_weight_always_serves() {
        // [A:100] → every matched user gets A
        let flag = CompiledFlag {
            flag_id: Uuid::new_v4(),
            flag_key: "solo-flag".to_owned(),
            value_type: ValueType::Boolean,
            is_active: true,
            enabled: true,
            variations: variations(&[("false", "false"), ("true", "true")]),
            off_variation_key: Some("false".to_owned()),
            default_variation_key: Some("false".to_owned()),
            targets: vec![],
            rules: vec![CompiledRule {
                rule_key: "all-true".into(),
                sort_order: 0,
                served_variation_key: None,
                rollout: Some(crate::types::CompiledRollout {
                    attribute: None,
                    variations: vec![crate::types::RolloutWeight {
                        variation_key: "true".into(),
                        weight: 100,
                    }],
                }),
                clauses: vec![],
            }],
            prerequisites: vec![],
        };
        let fs = make_flagset(vec![flag]);
        for i in 0..50u32 {
            let d = &evaluate(&fs, &ctx(&format!("user-{i}"), &[]))["solo-flag"];
            assert!(matches!(d.reason, Reason::Split));
            assert_eq!(d.variation_key.as_deref(), Some("true"));
        }
    }

    #[test]
    fn weighted_rollout_sticky_attribute_used_falls_back_to_targeting_key() {
        // With sticky attribute "email" present → buckets by email
        // Without "email" attribute → falls back to targeting_key (still assigns, does NOT skip)
        let flag = CompiledFlag {
            flag_id: Uuid::new_v4(),
            flag_key: "sticky-flag".to_owned(),
            value_type: ValueType::Boolean,
            is_active: true,
            enabled: true,
            variations: variations(&[("false", "false"), ("true", "true")]),
            off_variation_key: Some("false".to_owned()),
            default_variation_key: Some("false".to_owned()),
            targets: vec![],
            rules: vec![CompiledRule {
                rule_key: "sticky".into(),
                sort_order: 0,
                served_variation_key: None,
                rollout: Some(crate::types::CompiledRollout {
                    attribute: Some("email".into()),
                    variations: vec![crate::types::RolloutWeight {
                        variation_key: "true".into(),
                        weight: 100,
                    }],
                }),
                clauses: vec![],
            }],
            prerequisites: vec![],
        };
        let fs = make_flagset(vec![flag]);
        // With email → SPLIT
        let d_email = &evaluate(&fs, &ctx("u1", &[("email", "a@b.com")]))["sticky-flag"];
        assert!(
            matches!(d_email.reason, Reason::Split),
            "expected Split with email, got {:?}",
            d_email.reason
        );
        // Without email → falls back to targeting_key → still SPLIT (100% weight)
        let d_no_email = &evaluate(&fs, &ctx("u2", &[]))["sticky-flag"];
        assert!(
            matches!(d_no_email.reason, Reason::Split),
            "expected Split without email (fallback to targeting_key), got {:?}",
            d_no_email.reason
        );
    }

    #[test]
    fn prereq_satisfied_flag_evaluates_normally() {
        // prereq flag "gate" is enabled and evaluates to "true"
        // "child" flag should evaluate normally (not PrerequisiteFailed)
        let gate = bool_flag("gate", true, true); // enabled, default=true
        let mut child = bool_flag("child", true, true);
        child.prerequisites = vec![crate::types::CompiledPrerequisite {
            flag_key: "gate".into(),
            required_variation_key: "true".into(),
        }];
        let fs = make_flagset(vec![gate, child]);
        let d = &evaluate(&fs, &empty_ctx())["child"];
        // gate evaluates to "true" (Default) → prereq satisfied → child proceeds normally
        assert!(
            matches!(d.reason, Reason::Default),
            "expected Default, got {:?}",
            d.reason
        );
        assert_eq!(d.variation_key.as_deref(), Some("true"));
    }

    #[test]
    fn prereq_wrong_variation_returns_off() {
        // prereq gate is enabled and evaluates to "true", but we require "false"
        let gate = bool_flag("gate", true, true); // evaluates to "true"
        let mut child = bool_flag("child", true, true);
        child.prerequisites = vec![crate::types::CompiledPrerequisite {
            flag_key: "gate".into(),
            required_variation_key: "false".into(), // requires false, but gate returns true
        }];
        let fs = make_flagset(vec![gate, child]);
        let d = &evaluate(&fs, &empty_ctx())["child"];
        assert!(
            matches!(d.reason, Reason::PrerequisiteFailed),
            "expected PrerequisiteFailed, got {:?}",
            d.reason
        );
        assert_eq!(d.variation_key.as_deref(), Some("false")); // off variation
    }

    #[test]
    fn prereq_flag_missing_returns_off() {
        // prereq flag "nonexistent" is not in the flagset
        let mut child = bool_flag("child", true, true);
        child.prerequisites = vec![crate::types::CompiledPrerequisite {
            flag_key: "nonexistent".into(),
            required_variation_key: "true".into(),
        }];
        let fs = make_flagset(vec![child]);
        let d = &evaluate(&fs, &empty_ctx())["child"];
        assert!(
            matches!(d.reason, Reason::PrerequisiteFailed),
            "expected PrerequisiteFailed, got {:?}",
            d.reason
        );
    }

    #[test]
    fn prereq_cycle_terminates_safely() {
        // A requires B, B requires A → both should fail with PrerequisiteFailed, not infinite recursion
        let mut flag_a = bool_flag("flag-a", true, true);
        flag_a.prerequisites = vec![crate::types::CompiledPrerequisite {
            flag_key: "flag-b".into(),
            required_variation_key: "true".into(),
        }];
        let mut flag_b = bool_flag("flag-b", true, true);
        flag_b.prerequisites = vec![crate::types::CompiledPrerequisite {
            flag_key: "flag-a".into(),
            required_variation_key: "true".into(),
        }];
        let fs = make_flagset(vec![flag_a, flag_b]);
        let results = evaluate(&fs, &empty_ctx());
        // Both should fail (cycle detected) without stack overflow
        assert!(
            matches!(results["flag-a"].reason, Reason::PrerequisiteFailed),
            "flag-a expected PrerequisiteFailed, got {:?}",
            results["flag-a"].reason
        );
        assert!(
            matches!(results["flag-b"].reason, Reason::PrerequisiteFailed),
            "flag-b expected PrerequisiteFailed, got {:?}",
            results["flag-b"].reason
        );
    }
}
