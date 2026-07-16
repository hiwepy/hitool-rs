//! Multi-pattern matching aligned with Hutool's DFA module.

#![forbid(unsafe_code)]

use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};
use thiserror::Error;

/// Errors returned while building a matcher.
#[derive(Debug, Error)]
pub enum DfaError {
    /// No patterns were provided.
    #[error("at least one pattern is required")]
    EmptyPatterns,
    /// The underlying automaton could not be constructed.
    #[error("failed to build matcher: {0}")]
    Build(#[from] aho_corasick::BuildError),
}

/// One sensitive-word match in UTF-8 byte offsets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Match {
    /// Index of the matching pattern supplied at construction time.
    pub pattern_index: usize,
    /// Inclusive UTF-8 byte start offset.
    pub start: usize,
    /// Exclusive UTF-8 byte end offset.
    pub end: usize,
    /// Matching pattern text.
    pub pattern: String,
}

/// An immutable, thread-safe multi-pattern matcher.
#[derive(Debug, Clone)]
pub struct WordTree {
    automaton: AhoCorasick,
    patterns: Vec<String>,
}

impl WordTree {
    /// Builds a leftmost-longest matcher from non-empty patterns.
    pub fn new<I, S>(patterns: I) -> Result<Self, DfaError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let patterns: Vec<String> = patterns.into_iter().map(Into::into).collect();
        if patterns.is_empty() {
            return Err(DfaError::EmptyPatterns);
        }
        let automaton = AhoCorasickBuilder::new()
            .match_kind(MatchKind::LeftmostLongest)
            .build(&patterns)?;
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

    /// Finds all non-overlapping leftmost-longest matches.
    #[must_use]
    pub fn find_all(&self, input: &str) -> Vec<Match> {
        self.automaton
            .find_iter(input)
            .map(|found| {
                let pattern_index = found.pattern().as_usize();
                Match {
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
    fn selects_leftmost_longest_and_replaces_matches() {
        let tree = WordTree::new(["敏感", "敏感词", "bad"]).unwrap();
        assert!(tree.is_match("包含敏感词和bad"));
        assert_eq!(tree.find_all("包含敏感词")[0].pattern, "敏感词");
        assert_eq!(tree.replace_all("敏感词 bad", "***"), "*** ***");
    }
}
