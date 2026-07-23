use std::cmp::Ordering;

use thiserror::Error;

const DEFAULT_DELIMITER: &str = ";";

/// Errors returned by Hutool-compatible version expression matching.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum VersionError {
    /// The expression separator is blank, a range marker, or a comparison operator.
    #[error("invalid version delimiter: {0:?}")]
    InvalidDelimiter(String),
}

/// Hutool-compatible matching for loose Java module-style version strings.
pub struct VersionUtil;

impl VersionUtil {
    /// Returns whether the current version equals any supplied version expression.
    pub fn any_match<I, S>(current_version: &str, compare_versions: I) -> bool
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let expression = compare_versions
            .into_iter()
            .map(|version| version.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(DEFAULT_DELIMITER);
        Self::match_el(current_version, &expression).unwrap_or(false)
    }

    /// Returns whether the current version is greater than the comparison version.
    ///
    /// `None` aligns with Java `null` and sorts as the smallest version.
    #[must_use]
    pub fn is_greater_than(
        current_version: Option<&str>,
        compare_version: Option<&str>,
    ) -> bool {
        compare_nullable(current_version, compare_version) == Ordering::Greater
    }

    /// Returns whether the current version is greater than or equal to the comparison version.
    ///
    /// `None` aligns with Java `null` and sorts as the smallest version.
    #[must_use]
    pub fn is_greater_than_or_equal(
        current_version: Option<&str>,
        compare_version: Option<&str>,
    ) -> bool {
        compare_nullable(current_version, compare_version) != Ordering::Less
    }

    /// Returns whether the current version is less than the comparison version.
    ///
    /// `None` aligns with Java `null` and sorts as the smallest version.
    #[must_use]
    pub fn is_less_than(current_version: Option<&str>, compare_version: Option<&str>) -> bool {
        compare_nullable(current_version, compare_version) == Ordering::Less
    }

    /// Returns whether the current version is less than or equal to the comparison version.
    ///
    /// `None` aligns with Java `null` and sorts as the smallest version.
    #[must_use]
    pub fn is_less_than_or_equal(
        current_version: Option<&str>,
        compare_version: Option<&str>,
    ) -> bool {
        compare_nullable(current_version, compare_version) != Ordering::Greater
    }

    /// Convenience wrapper for non-null `&str` arguments.
    ///
    /// Treats the literal strings `"null"` / `"NULL"` like Java `null` on the compare side.
    #[must_use]
    #[inline]
    pub fn is_greater_than_str(current_version: &str, compare_version: &str) -> bool {
        Self::is_greater_than(
            Some(current_version),
            comparison_target(compare_version),
        )
    }

    /// Convenience wrapper for non-null `&str` arguments.
    ///
    /// Treats the literal strings `"null"` / `"NULL"` like Java `null` on the compare side.
    #[must_use]
    #[inline]
    pub fn is_greater_than_or_equal_str(current_version: &str, compare_version: &str) -> bool {
        Self::is_greater_than_or_equal(
            Some(current_version),
            comparison_target(compare_version),
        )
    }

    /// Convenience wrapper for non-null `&str` arguments.
    ///
    /// Treats the literal strings `"null"` / `"NULL"` like Java `null` on the compare side.
    #[must_use]
    #[inline]
    pub fn is_less_than_str(current_version: &str, compare_version: &str) -> bool {
        Self::is_less_than(
            Some(current_version),
            comparison_target(compare_version),
        )
    }

    /// Convenience wrapper for non-null `&str` arguments.
    ///
    /// Treats the literal strings `"null"` / `"NULL"` like Java `null` on the compare side.
    #[must_use]
    #[inline]
    pub fn is_less_than_or_equal_str(current_version: &str, compare_version: &str) -> bool {
        Self::is_less_than_or_equal(
            Some(current_version),
            comparison_target(compare_version),
        )
    }

    /// Matches one or more `;`-separated exact, comparison, or inclusive range expressions.
    pub fn match_el(current_version: &str, version_el: &str) -> Result<bool, VersionError> {
        Self::match_el_with_delimiter(current_version, version_el, DEFAULT_DELIMITER)
    }

    /// Matches expressions using an explicit non-operator delimiter.
    pub fn match_el_with_delimiter(
        current_version: &str,
        version_el: &str,
        versions_delimiter: &str,
    ) -> Result<bool, VersionError> {
        validate_delimiter(versions_delimiter)?;
        if version_el.trim().is_empty() {
            return Ok(false);
        }
        let current = current_version.trim();
        for expression in version_el
            .split(versions_delimiter)
            .map(str::trim)
            .filter(|expression| !expression.is_empty())
        {
            if let Some((operator, version)) = split_operator(expression) {
                let version = if version.eq_ignore_ascii_case("null") {
                    None
                } else {
                    Some(version)
                };
                let ordering = compare_nullable(Some(current), version);
                let matches = match operator {
                    ">=" | "≥" => ordering != Ordering::Less,
                    "<=" | "≤" => ordering != Ordering::Greater,
                    ">" => ordering == Ordering::Greater,
                    "<" => ordering == Ordering::Less,
                    _ => false,
                };
                if matches {
                    return Ok(true);
                }
            } else if let Some(index) = expression.find('-') {
                let left = expression[..index].trim();
                let right = expression[index + 1..].trim();
                let left_matches =
                    left.is_empty() || compare_versions(left, current) != Ordering::Greater;
                let right_matches =
                    right.is_empty() || compare_versions(right, current) != Ordering::Less;
                if left_matches && right_matches {
                    return Ok(true);
                }
            } else if current == expression {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

fn validate_delimiter(delimiter: &str) -> Result<(), VersionError> {
    if delimiter.trim().is_empty()
        || delimiter == "-"
        || matches!(delimiter.chars().next(), Some('>' | '<' | '≥' | '≤'))
    {
        return Err(VersionError::InvalidDelimiter(delimiter.to_owned()));
    }
    Ok(())
}

fn split_operator(expression: &str) -> Option<(&str, &str)> {
    for operator in [">=", "<=", "≥=", "≤=", ">", "<", "≥", "≤"] {
        if let Some(version) = expression.strip_prefix(operator) {
            return Some((operator, version));
        }
    }
    None
}

/// Compares two nullable version strings; `None` sorts as smallest (Java `null`).
fn compare_nullable(current: Option<&str>, compare: Option<&str>) -> Ordering {
    let current = current.map(str::trim);
    match (current, compare) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(a), Some(b)) => compare_versions(a, b),
    }
}

fn comparison_target(version: &str) -> Option<&str> {
    (!version.eq_ignore_ascii_case("null")).then_some(version)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Number(i32),
    Text(String),
}

#[derive(Debug, Default)]
struct LooseVersion {
    sequence: Vec<Token>,
    pre: Vec<Token>,
    build: Vec<Token>,
}

fn compare_versions(left: &str, right: &str) -> Ordering {
    let left = parse_version(left);
    let right = parse_version(right);
    compare_tokens(&left.sequence, &right.sequence)
        .then_with(|| match (left.pre.is_empty(), right.pre.is_empty()) {
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            _ => Ordering::Equal,
        })
        .then_with(|| compare_tokens(&left.pre, &right.pre))
        .then_with(|| compare_tokens(&left.build, &right.build))
}

fn parse_version(value: &str) -> LooseVersion {
    let chars: Vec<char> = value.chars().collect();
    if chars.is_empty() {
        return LooseVersion::default();
    }
    let mut version = LooseVersion::default();
    let mut index = take_number(&chars, 0, &mut version.sequence);
    let mut separator = chars[0];
    while index < chars.len() {
        separator = chars[index];
        if separator == '.' {
            index += 1;
        } else if separator == '-' || separator == '+' {
            index += 1;
            break;
        } else if separator.is_ascii_digit() {
            index = take_number(&chars, index, &mut version.sequence);
        } else {
            index = take_text(&chars, index, &mut version.sequence);
        }
    }
    if separator == '-' && index >= chars.len() {
        return version;
    }
    while index < chars.len() {
        if chars[index].is_ascii_digit() {
            index = take_number(&chars, index, &mut version.pre);
        } else {
            index = take_text(&chars, index, &mut version.pre);
        }
        if index >= chars.len() {
            break;
        }
        separator = chars[index];
        if separator == '.' || separator == '-' {
            index += 1;
        } else if separator == '+' {
            index += 1;
            break;
        }
    }
    if separator == '+' && index >= chars.len() {
        return version;
    }
    while index < chars.len() {
        if chars[index].is_ascii_digit() {
            index = take_number(&chars, index, &mut version.build);
        } else {
            index = take_text(&chars, index, &mut version.build);
        }
        // Both token readers stop only at a version separator or at end-of-input.
        if index < chars.len() {
            index += 1;
        }
    }
    version
}

fn take_number(chars: &[char], mut index: usize, output: &mut Vec<Token>) -> usize {
    let mut number = (chars[index] as i32).wrapping_sub('0' as i32);
    index += 1;
    while index < chars.len() && chars[index].is_ascii_digit() {
        number = number
            .wrapping_mul(10)
            .wrapping_add((chars[index] as i32).wrapping_sub('0' as i32));
        index += 1;
    }
    output.push(Token::Number(number));
    index
}

fn take_text(chars: &[char], mut index: usize, output: &mut Vec<Token>) -> usize {
    let start = index;
    index += 1;
    while index < chars.len()
        && !matches!(chars[index], '.' | '-' | '+')
        && !chars[index].is_ascii_digit()
    {
        index += 1;
    }
    output.push(Token::Text(chars[start..index].iter().collect()));
    index
}

fn compare_tokens(left: &[Token], right: &[Token]) -> Ordering {
    for (left, right) in left.iter().zip(right) {
        let ordering = match (left, right) {
            (Token::Number(left), Token::Number(right)) => left.cmp(right),
            (Token::Text(left), Token::Text(right)) => java_string_cmp(left, right),
            (Token::Number(left), Token::Text(right)) => java_string_cmp(&left.to_string(), right),
            (Token::Text(left), Token::Number(right)) => java_string_cmp(left, &right.to_string()),
        };
        if ordering != Ordering::Equal {
            return ordering;
        }
    }
    let rest = if left.len() > right.len() {
        left
    } else {
        right
    };
    if rest
        .iter()
        .skip(left.len().min(right.len()))
        .any(|token| !matches!(token, Token::Number(0)))
    {
        left.len().cmp(&right.len())
    } else {
        Ordering::Equal
    }
}

fn java_string_cmp(left: &str, right: &str) -> Ordering {
    left.encode_utf16().cmp(right.encode_utf16())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comparisons_support_hutool_loose_versions_and_null_expression() {
        assert!(VersionUtil::is_greater_than_str(" 1.0.2", "1.0.1"));
        assert!(VersionUtil::is_greater_than_str("1.0.2", "1"));
        assert!(!VersionUtil::is_greater_than_str("1.0.2", "1.1"));
        assert!(VersionUtil::is_greater_than_or_equal_str("1.0.2 ", "1.0.2"));
        assert!(VersionUtil::is_less_than_str("1.0.2", "1.0.3"));
        assert!(VersionUtil::is_less_than_or_equal_str("1.0.2", "1.1"));
        assert_eq!(compare_versions("8.5a", "8.5c"), Ordering::Less);
        assert_eq!(compare_versions("1.0", "1.0.0"), Ordering::Equal);
        assert_eq!(compare_versions("1.0.2", "1.0.2a"), Ordering::Less);
        assert_eq!(compare_versions("1.13.0", "1.12.1c"), Ordering::Greater);
        assert_eq!(
            compare_versions("V0.0.20170102", "V0.0.20170101"),
            Ordering::Greater
        );
        assert!(VersionUtil::match_el("1.0.2", ">null").unwrap());
        assert!(!VersionUtil::match_el("1.0.2", "<null").unwrap());
        assert!(VersionUtil::is_greater_than_str("1.0.2", "NULL"));
        assert!(!VersionUtil::is_less_than_or_equal_str("1.0.2", "null"));
        assert!(VersionUtil::is_greater_than(Some("1.0"), None));
        assert!(!VersionUtil::is_greater_than(None, Some("1.0")));
        assert_eq!(compare_nullable(Some("1.0"), None), Ordering::Greater);
        assert_eq!(compare_nullable(None, Some("1.0")), Ordering::Less);
        assert_eq!(compare_nullable(None, None), Ordering::Equal);
    }

    #[test]
    fn exact_comparison_range_and_multi_expression_matching_are_complete() {
        assert!(VersionUtil::match_el("1.0.2", "1.0.1;1.0.2").unwrap());
        assert!(!VersionUtil::match_el("1.0.2", "1.0.1;1.0.3").unwrap());
        assert!(VersionUtil::match_el("1.0.2", "1.0.9;1.0.1-1.0.2").unwrap());
        assert!(VersionUtil::match_el_with_delimiter("1.0.2", "1.0.9,1.0.1-1.0.3", ",").unwrap());
        assert!(VersionUtil::match_el("1.0.2", "-1.0.3").unwrap());
        assert!(VersionUtil::match_el("1.0.2", "1.0.0-").unwrap());
        assert!(VersionUtil::match_el("1.0.2", "-").unwrap());
        assert!(VersionUtil::match_el("1.0.2", "≥1.0.2").unwrap());
        assert!(VersionUtil::match_el("1.0.2", "≤1.0.2").unwrap());
        assert!(VersionUtil::match_el("1.0.2", ">=1.0.2").unwrap());
        assert!(VersionUtil::match_el("1.0.2", "<=1.0.2").unwrap());
        assert!(!VersionUtil::match_el("1.0.2", ">1.0.2").unwrap());
        assert!(!VersionUtil::match_el("1.0.2", "<1.0.2").unwrap());
        assert!(!VersionUtil::match_el("1.0.2", "≥=1.0.2").unwrap());
        assert!(!VersionUtil::match_el("1.0.2", "≤=1.0.2").unwrap());
        assert!(!VersionUtil::match_el("1.0.2", "2.0.0-3.0.0").unwrap());
        assert!(!VersionUtil::match_el("1.0.2", " ").unwrap());
        assert!(VersionUtil::any_match("1.0.2", ["1.0.1", "1.0.2"]));
        assert!(!VersionUtil::any_match("1.0.2", std::iter::empty::<&str>()));
    }

    #[test]
    fn illegal_delimiters_and_parser_edge_paths_are_explicit() {
        for delimiter in ["-", ">", ">=", "<", "<=", "≥", "≤", "", " "] {
            assert_eq!(
                VersionUtil::match_el_with_delimiter("1.0.2", "1.0.2", delimiter).unwrap_err(),
                VersionError::InvalidDelimiter(delimiter.to_owned())
            );
        }
        assert_eq!(compare_versions("", ""), Ordering::Equal);
        assert_eq!(compare_versions("1-", "1"), Ordering::Equal);
        assert_eq!(compare_versions("1+", "1"), Ordering::Equal);
        assert_eq!(
            compare_versions("1.0-alpha+7", "1.0-alpha+8"),
            Ordering::Less
        );
        assert_eq!(
            compare_versions("1.0-alpha.beta", "1.0-alpha.beta"),
            Ordering::Equal
        );
        assert_eq!(compare_versions("1.0-a1", "1.0-a2"), Ordering::Less);
        assert_eq!(compare_versions("a1", "b1"), Ordering::Less);
        assert_eq!(compare_versions("1.0", "1.0-alpha"), Ordering::Greater);
        assert_eq!(compare_versions("1.0-alpha", "1.0"), Ordering::Less);
        assert_eq!(compare_versions("1+a", "1+b"), Ordering::Less);
        assert_eq!(compare_versions("1+a-b+c", "1+a-b+d"), Ordering::Less);
        assert_eq!(
            compare_versions("1-a+b.c-d+e", "1-a+b.c-d+f"),
            Ordering::Less
        );
        assert_eq!(compare_versions("1.a", "1.2"), Ordering::Greater);
        assert_eq!(compare_versions("1.2", "1.a"), Ordering::Less);
        assert_eq!(compare_versions("1.a", "1"), Ordering::Greater);
    }
}
