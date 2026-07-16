//! Token counting helper.
//!
//! When the `llm-tiktoken` feature is active, uses tiktoken encodings for known
//! model prefixes. Otherwise (and for unknown models), falls back to a
//! `chars / 4` heuristic — cheap and good enough for budget estimation.

/// Approximate token count for `text` given a `model_id`.
///
/// # Accuracy
///
/// - With `llm-tiktoken` feature: exact for GPT-4 and GPT-3.5 (cl100k_base
///   encoding); approximate for Claude models (Anthropic's tokenizer is not
///   public — cl100k_base is used as a proxy and may diverge by several percent
///   on English text and more on code/Unicode). Unknown models fall back to
///   the heuristic.
/// - Without `llm-tiktoken` feature: always uses `chars / 4` heuristic.
///
/// # Examples
///
/// ```rust,ignore
/// let n = token_count("gpt-4o", "Hello, world!");
/// ```
pub fn token_count(model_id: &str, text: &str) -> usize {
    #[cfg(feature = "llm-tiktoken")]
    {
        use_tiktoken(model_id, text)
    }
    #[cfg(not(feature = "llm-tiktoken"))]
    {
        let _ = model_id;
        heuristic(text)
    }
}

/// `chars / 4` heuristic — fast, no dependency.
pub(crate) fn heuristic(text: &str) -> usize {
    let chars = text.chars().count();
    if chars == 0 {
        0
    } else {
        (chars / 4).max(1)
    }
}

#[cfg(feature = "llm-tiktoken")]
fn use_tiktoken(model_id: &str, text: &str) -> usize {
    // Map known model prefixes to tiktoken encodings.
    let enc = if model_id.starts_with("gpt-4")
        || model_id.starts_with("gpt-3.5")
        || model_id.starts_with("claude-")
    {
        tiktoken_rs::cl100k_base()
    } else {
        return heuristic(text);
    };

    match enc {
        Ok(e) => e.encode_with_special_tokens(text).len(),
        Err(_) => heuristic(text),
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_count_heuristic_fallback() {
        // Unknown model — must use chars/4 fallback regardless of feature flags.
        // We verify the heuristic function directly since the public fn may call tiktoken.
        let text = "Hello world"; // 11 chars → 11/4 = 2
        assert_eq!(heuristic(text), 2);
    }

    #[test]
    fn token_count_empty_string() {
        assert_eq!(heuristic(""), 0);
    }

    #[test]
    fn token_count_single_char() {
        // 1 char → max(1/4, 1) = 1
        assert_eq!(heuristic("a"), 1);
    }

    #[test]
    fn token_count_unknown_model_uses_heuristic() {
        // Public function with unknown model — should not panic.
        let n = token_count(
            "unknown-model-xyz",
            "The quick brown fox jumps over the lazy dog",
        );
        assert!(n > 0, "should return a positive count");
    }

    #[test]
    fn token_count_known_model_does_not_panic() {
        let n = token_count("gpt-4o", "Hello, world!");
        assert!(n > 0, "should return a positive count for gpt-4o");
    }
}
