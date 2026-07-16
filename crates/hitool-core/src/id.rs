//! Identifier generation utilities.

use uuid::Uuid;

/// UUID-based identifier helpers.
pub struct IdUtil;

impl IdUtil {
    /// Creates a hyphenated random UUID v4.
    #[must_use]
    pub fn uuid() -> String {
        Uuid::new_v4().to_string()
    }

    /// Creates a compact random UUID v4 without hyphens.
    #[must_use]
    pub fn simple_uuid() -> String {
        Uuid::new_v4().simple().to_string()
    }

    /// Parses and validates a UUID string.
    #[must_use]
    pub fn is_valid(value: &str) -> bool {
        Uuid::parse_str(value).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_identifiers_are_valid_and_unique() {
        let first = IdUtil::uuid();
        let second = IdUtil::uuid();
        assert!(IdUtil::is_valid(&first));
        assert_ne!(first, second);
        assert_eq!(IdUtil::simple_uuid().len(), 32);
    }
}
