//! Immutable high-throughput matching backed by `aho-corasick`.

use crate::DfaError;
use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};

const MAX_PATTERN_COUNT: usize = 100_000;
const MAX_PATTERN_BYTES: usize = 16 * 1024 * 1024;

/// One immutable-engine match in UTF-8 byte offsets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternMatch {
    /// Index of the matching pattern supplied at construction time.
    pub pattern_index: usize,
    /// Inclusive UTF-8 byte start offset.
    pub start: usize,
    /// Exclusive UTF-8 byte end offset.
    pub end: usize,
    /// Matching pattern text.
    pub pattern: String,
}

/// Immutable, thread-safe leftmost-longest multi-pattern matcher.
#[derive(Debug, Clone)]
pub struct DfaMatcher {
    automaton: AhoCorasick,
    patterns: Vec<String>,
}

impl DfaMatcher {
    /// Builds a matcher from non-empty, defensively bounded patterns.
    ///
    /// # Panics
    ///
    /// Panics only if a future `aho-corasick` release rejects a pattern set
    /// that is within `HiTool`'s count and byte bounds; the pinned engine accepts
    /// every input that passes these checks.
    pub fn new<I, S>(patterns: I) -> Result<Self, DfaError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let patterns: Vec<String> = patterns.into_iter().map(Into::into).collect();
        if patterns.is_empty() {
            return Err(DfaError::EmptyPatterns);
        }
        if patterns.iter().any(String::is_empty) {
            return Err(DfaError::EmptyPattern);
        }
        if patterns.len() > MAX_PATTERN_COUNT {
            return Err(DfaError::PatternSetTooLarge {
                actual: patterns.len(),
                maximum: MAX_PATTERN_COUNT,
            });
        }
        let pattern_bytes = patterns.iter().map(String::len).sum::<usize>();
        if pattern_bytes > MAX_PATTERN_BYTES {
            return Err(DfaError::PatternSetTooLarge {
                actual: pattern_bytes,
                maximum: MAX_PATTERN_BYTES,
            });
        }
        let automaton = AhoCorasickBuilder::new()
            .match_kind(MatchKind::LeftmostLongest)
            .build(&patterns)
            .expect("bounded non-empty patterns fit Aho-Corasick identifiers");
        Ok(Self {
            automaton,
            patterns,
        })
    }

    /// Returns whether any pattern occurs in the input.
    #[must_use]
    pub fn is_match(&self, input: &str) -> bool {
        self.automaton.is_match(input)
    }

    /// Finds non-overlapping leftmost-longest matches.
    #[must_use]
    pub fn find_all(&self, input: &str) -> Vec<PatternMatch> {
        self.automaton
            .find_iter(input)
            .map(|found| {
                let pattern_index = found.pattern().as_usize();
                PatternMatch {
                    pattern_index,
                    start: found.start(),
                    end: found.end(),
                    pattern: self.patterns[pattern_index].clone(),
                }
            })
            .collect()
    }

    /// Replaces each match with the supplied marker.
    #[must_use]
    pub fn replace_all(&self, input: &str, replacement: &str) -> String {
        let replacements = vec![replacement; self.patterns.len()];
        self.automaton.replace_all(input, &replacements)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_patterns_and_uses_leftmost_longest_engine() {
        let error = DfaMatcher::new(Vec::<String>::new()).unwrap_err();
        assert!(error.to_string().contains("at least one"));
        let error = DfaMatcher::new(vec!["valid".to_owned(), String::new()]).unwrap_err();
        assert!(error.to_string().contains("must not be empty"));
        let error = DfaMatcher::new(vec!["a".to_owned(); MAX_PATTERN_COUNT + 1]).unwrap_err();
        assert!(error.to_string().contains("too large"));
        let error = DfaMatcher::new(vec!["a".repeat(MAX_PATTERN_BYTES + 1)]).unwrap_err();
        assert!(error.to_string().contains("too large"));

        let matcher = DfaMatcher::new(vec![
            "敏感".to_owned(),
            "敏感词".to_owned(),
            "bad".to_owned(),
        ])
        .unwrap();
        assert!(matcher.is_match("包含敏感词和bad"));
        assert!(!matcher.is_match("clean"));
        assert_eq!(
            matcher.find_all("包含敏感词 bad"),
            [
                PatternMatch {
                    pattern_index: 1,
                    start: 6,
                    end: 15,
                    pattern: "敏感词".to_owned(),
                },
                PatternMatch {
                    pattern_index: 2,
                    start: 16,
                    end: 19,
                    pattern: "bad".to_owned(),
                }
            ]
        );
        assert_eq!(matcher.replace_all("敏感词 bad", "***"), "*** ***");
    }
}
