//! Unicode-aware string utilities.
//!
//! Portions of the behavior and tests were adapted from yimi-rutool 0.2.5
//! (Apache-2.0) and substantially revised for borrowed strings, Unicode, and
//! Rust extension traits.

use std::fmt::{Display, Write};

/// Returns `true` when a string is empty or contains only Unicode whitespace.
#[inline]
#[must_use]
pub fn is_blank(value: &str) -> bool {
    value.trim().is_empty()
}

/// Removes every non-overlapping occurrence of `needle` from `value`.
#[must_use]
pub fn remove_all(value: &str, needle: &str) -> String {
    if needle.is_empty() {
        return value.to_owned();
    }
    value.replace(needle, "")
}

/// Removes all characters listed in `characters` from `value`.
#[must_use]
pub fn remove_chars(value: &str, characters: &[char]) -> String {
    value
        .chars()
        .filter(|character| !characters.contains(character))
        .collect()
}

/// Uppercases the first Unicode scalar value without changing the remainder.
#[must_use]
pub fn upper_first(value: &str) -> String {
    change_first(value, char::to_uppercase)
}

/// Lowercases the first Unicode scalar value without changing the remainder.
#[must_use]
pub fn lower_first(value: &str) -> String {
    change_first(value, char::to_lowercase)
}

fn change_first<I>(value: &str, transform: impl FnOnce(char) -> I) -> String
where
    I: Iterator<Item = char>,
{
    let Some(first) = value.chars().next() else {
        return String::new();
    };
    let remainder = &value[first.len_utf8()..];
    let mut result = String::with_capacity(value.len());
    result.extend(transform(first));
    result.push_str(remainder);
    result
}

/// Splits a string with optional trimming and empty-item removal.
#[must_use]
pub fn split(value: &str, separator: char, trim_items: bool, ignore_empty: bool) -> Vec<&str> {
    value
        .split(separator)
        .map(|item| if trim_items { item.trim() } else { item })
        .filter(|item| !ignore_empty || !item.is_empty())
        .collect()
}

/// Formats sequential `{}` placeholders.
///
/// `{{` and `}}` produce literal braces. Missing values leave their `{}`
/// placeholder intact, while extra values are ignored.
#[must_use]
pub fn format_template(template: &str, values: &[&dyn Display]) -> String {
    let mut result = String::with_capacity(template.len());
    let mut chars = template.chars().peekable();
    let mut value_index = 0;

    while let Some(character) = chars.next() {
        match (character, chars.peek().copied()) {
            ('{', Some('{')) => {
                chars.next();
                result.push('{');
            }
            ('}', Some('}')) => {
                chars.next();
                result.push('}');
            }
            ('{', Some('}')) => {
                chars.next();
                if let Some(value) = values.get(value_index) {
                    write!(&mut result, "{value}").expect("writing to String cannot fail");
                    value_index += 1;
                } else {
                    result.push_str("{}");
                }
            }
            _ => result.push(character),
        }
    }

    result
}

/// Extension methods for string slices.
pub trait StrExt {
    /// Returns `true` when the string is empty or only Unicode whitespace.
    fn is_blank(&self) -> bool;

    /// Returns `true` when the string contains a non-whitespace character.
    fn is_not_blank(&self) -> bool;

    /// Returns a borrowed string with surrounding Unicode whitespace removed.
    fn trimmed(&self) -> &str;

    /// Returns an owned string with all occurrences of `needle` removed.
    fn without(&self, needle: &str) -> String;

    /// Returns an owned string with the first character uppercased.
    fn upper_first(&self) -> String;

    /// Returns an owned string with the first character lowercased.
    fn lower_first(&self) -> String;
}

impl StrExt for str {
    #[inline]
    fn is_blank(&self) -> bool {
        is_blank(self)
    }

    #[inline]
    fn is_not_blank(&self) -> bool {
        !is_blank(self)
    }

    #[inline]
    fn trimmed(&self) -> &str {
        self.trim()
    }

    fn without(&self, needle: &str) -> String {
        remove_all(self, needle)
    }

    fn upper_first(&self) -> String {
        upper_first(self)
    }

    fn lower_first(&self) -> String {
        lower_first(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blank_uses_unicode_whitespace() {
        assert!(is_blank(" \t\n\u{3000}"));
        assert!(!is_blank(" value "));
    }

    #[test]
    fn first_character_conversion_is_unicode_safe() {
        assert_eq!(upper_first("élan"), "Élan");
        assert_eq!(lower_first("Über"), "über");
        assert_eq!(upper_first(""), "");
    }

    #[test]
    fn split_can_trim_and_drop_empty_items() {
        assert_eq!(split(" a, ,b,", ',', true, true), ["a", "b"]);
        assert_eq!(split("a,,b", ',', false, false), ["a", "", "b"]);
    }

    #[test]
    fn template_formatting_handles_escapes_and_missing_values() {
        let count = 2;
        let name = "files";
        assert_eq!(
            format_template("{{copied}} {} {} {}", &[&count, &name]),
            "{copied} 2 files {}"
        );
    }

    #[test]
    fn extension_trait_keeps_borrowed_operations_borrowed() {
        let value = "  hello  ";
        assert_eq!(value.trimmed(), "hello");
        assert!(" \n".is_blank());
        assert_eq!("banana".without("na"), "ba");
    }
}
