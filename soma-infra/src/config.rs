//! Tiny environment-variable helpers. No framework, no global state.

use std::str::FromStr;

/// Errors from reading or parsing environment configuration.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// A required variable was not set.
    #[error("required env var {0} is not set")]
    Missing(&'static str),
    /// A variable was set but could not be parsed into the requested type.
    #[error("env var {0} could not be parsed")]
    Parse(&'static str),
}

/// Read a required variable, erroring if unset.
pub fn require_env(key: &'static str) -> Result<String, ConfigError> {
    std::env::var(key).map_err(|_| ConfigError::Missing(key))
}

/// Read a variable or fall back to `default`.
pub fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

/// Parse an optional variable: `Ok(None)` if unset, `Err` if set-but-unparseable.
pub fn env_parse<T: FromStr>(key: &'static str) -> Result<Option<T>, ConfigError> {
    match std::env::var(key) {
        Ok(v) => v
            .parse::<T>()
            .map(Some)
            .map_err(|_| ConfigError::Parse(key)),
        Err(_) => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_or_uses_default_when_unset() {
        assert_eq!(env_or("SOMA_COMMON_TEST_UNSET_KEY", "fallback"), "fallback");
    }

    #[test]
    fn env_parse_reads_and_parses() {
        std::env::set_var("SOMA_COMMON_TEST_PORT", "5432");
        let parsed: Option<u16> = env_parse("SOMA_COMMON_TEST_PORT").unwrap();
        assert_eq!(parsed, Some(5432));
        std::env::remove_var("SOMA_COMMON_TEST_PORT");
        let missing: Option<u16> = env_parse("SOMA_COMMON_TEST_PORT").unwrap();
        assert_eq!(missing, None);
    }

    #[test]
    fn env_parse_rejects_garbage() {
        std::env::set_var("SOMA_COMMON_TEST_BAD", "notanumber");
        let r: Result<Option<u16>, _> = env_parse("SOMA_COMMON_TEST_BAD");
        assert!(r.is_err());
        std::env::remove_var("SOMA_COMMON_TEST_BAD");
    }
}
