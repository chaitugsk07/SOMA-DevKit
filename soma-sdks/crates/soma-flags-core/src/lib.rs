#![forbid(unsafe_code)]

mod eval;
mod types;

pub use eval::evaluate;
pub use types::{
    CompiledClause, CompiledFlag, CompiledPrerequisite, CompiledRollout, CompiledRule,
    CompiledSegment, CompiledTarget, CompiledVariation, Decision, EvalContext, FlagSet, Op, Reason,
    RolloutWeight, ValueType,
};
