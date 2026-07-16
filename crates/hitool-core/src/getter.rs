//! Strongly typed getter traits aligned with Hutool's `core.getter` package.

use std::{any::Any, collections::HashMap, hash::Hash, str::FromStr};

/// Retrieves scalar values and converts them through Rust's standard parser.
pub trait BasicTypeGetter<K: ?Sized> {
    /// Returns the raw string associated with `key`.
    fn raw(&self, key: &K) -> Option<&str>;

    /// Parses a typed scalar, returning `None` for absence or invalid input.
    fn get<T>(&self, key: &K) -> Option<T>
    where
        T: FromStr,
    {
        self.raw(key)?.parse().ok()
    }
}

/// Adds Hutool-style caller-provided defaults to [`BasicTypeGetter`].
pub trait OptBasicTypeGetter<K: ?Sized>: BasicTypeGetter<K> {
    /// Parses a value or returns `default` when absent/invalid.
    fn get_or<T>(&self, key: &K, default: T) -> T
    where
        T: FromStr,
    {
        self.get(key).unwrap_or(default)
    }
}

impl<K: ?Sized, G: BasicTypeGetter<K> + ?Sized> OptBasicTypeGetter<K> for G {}

/// Marker for nullable scalar getter behavior (`None` is Rust's null value).
pub trait OptNullBasicTypeGetter<K: ?Sized>: BasicTypeGetter<K> {}

impl<K: ?Sized, G: BasicTypeGetter<K> + ?Sized> OptNullBasicTypeGetter<K> for G {}

/// String-backed nullable scalar getter counterpart.
pub trait OptNullBasicTypeFromStringGetter<K: ?Sized>: OptNullBasicTypeGetter<K> {}

impl<K: ?Sized, G: OptNullBasicTypeGetter<K> + ?Sized> OptNullBasicTypeFromStringGetter<K> for G {}

/// Safely retrieves dynamically typed object values through `Any` downcasting.
pub trait OptNullBasicTypeFromObjectGetter<K> {
    /// Returns the stored value when it has the requested concrete type.
    fn get_object<T: Any>(&self, key: &K) -> Option<&T>;
}

impl<K, S> OptNullBasicTypeFromObjectGetter<K> for HashMap<K, Box<dyn Any + Send + Sync>, S>
where
    K: Eq + Hash,
    S: std::hash::BuildHasher,
{
    fn get_object<T: Any>(&self, key: &K) -> Option<&T> {
        self.get(key)?.downcast_ref()
    }
}

/// Retrieves delimiter-separated values as typed arrays.
pub trait ArrayTypeGetter {
    /// Returns the raw list source.
    fn raw_array(&self, key: &str) -> Option<&str>;

    /// Parses all items; one invalid item makes the result absent.
    fn get_array<T>(&self, key: &str) -> Option<Vec<T>>
    where
        T: FromStr,
    {
        self.raw_array(key)?
            .split(',')
            .map(str::trim)
            .map(str::parse)
            .collect::<std::result::Result<Vec<_>, _>>()
            .ok()
    }
}

/// Adds caller-provided defaults to [`ArrayTypeGetter`].
pub trait OptArrayTypeGetter: ArrayTypeGetter {
    /// Parses an array or returns `default` when absent/invalid.
    fn get_array_or<T>(&self, key: &str, default: Vec<T>) -> Vec<T>
    where
        T: FromStr,
    {
        self.get_array(key).unwrap_or(default)
    }
}

impl<G: ArrayTypeGetter + ?Sized> OptArrayTypeGetter for G {}

/// List getter counterpart; Rust uses `Vec<T>` for both array/list results.
pub trait ListTypeGetter: ArrayTypeGetter {
    /// Parses a typed list.
    fn get_list<T>(&self, key: &str) -> Option<Vec<T>>
    where
        T: FromStr,
    {
        self.get_array(key)
    }
}

impl<G: ArrayTypeGetter + ?Sized> ListTypeGetter for G {}

/// Retrieves values from a `(group, key)` namespace.
pub trait GroupedTypeGetter {
    /// Returns the raw grouped value.
    fn raw_by_group(&self, group: &str, key: &str) -> Option<&str>;

    /// Parses a typed grouped value.
    fn get_by_group<T>(&self, group: &str, key: &str) -> Option<T>
    where
        T: FromStr,
    {
        self.raw_by_group(group, key)?.parse().ok()
    }
}

/// Production-ready string/group map implementation of all string getter traits.
#[derive(Debug, Clone, Default)]
pub struct StringMapGetter {
    values: HashMap<String, String>,
    groups: HashMap<String, HashMap<String, String>>,
}

impl StringMapGetter {
    /// Creates a getter from flat values.
    #[must_use]
    pub fn new(values: impl IntoIterator<Item = (String, String)>) -> Self {
        Self {
            values: values.into_iter().collect(),
            groups: HashMap::new(),
        }
    }

    /// Inserts or replaces a grouped value.
    pub fn insert_grouped(
        &mut self,
        group: impl Into<String>,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Option<String> {
        self.groups
            .entry(group.into())
            .or_default()
            .insert(key.into(), value.into())
    }
}

impl BasicTypeGetter<str> for StringMapGetter {
    fn raw(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }
}

impl ArrayTypeGetter for StringMapGetter {
    fn raw_array(&self, key: &str) -> Option<&str> {
        self.raw(key)
    }
}

impl GroupedTypeGetter for StringMapGetter {
    fn raw_by_group(&self, group: &str, key: &str) -> Option<&str> {
        self.groups.get(group)?.get(key).map(String::as_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getter_traits_cover_scalars_defaults_arrays_lists_groups_and_objects() {
        let mut getter = StringMapGetter::new([
            ("count".into(), "42".into()),
            ("enabled".into(), "true".into()),
            ("items".into(), "1, 2,3".into()),
            ("invalid".into(), "not-a-number".into()),
        ]);
        assert_eq!(getter.get::<i32>("count"), Some(42));
        assert_eq!(getter.get::<bool>("enabled"), Some(true));
        assert_eq!(getter.get::<i32>("missing"), None);
        assert_eq!(getter.get::<i32>("invalid"), None);
        assert_eq!(getter.get_or("missing", 7_i32), 7);
        assert_eq!(getter.get_array::<i32>("items"), Some(vec![1, 2, 3]));
        assert_eq!(getter.get_array::<i32>("invalid"), None);
        assert_eq!(getter.get_array_or("missing", vec![9_i32]), [9]);
        assert_eq!(getter.get_list::<i32>("items"), Some(vec![1, 2, 3]));

        assert_eq!(getter.insert_grouped("db", "port", "5432"), None);
        assert_eq!(
            getter.insert_grouped("db", "port", "6432"),
            Some("5432".into())
        );
        assert_eq!(getter.get_by_group::<u16>("db", "port"), Some(6432));
        assert_eq!(getter.get_by_group::<u16>("db", "missing"), None);
        assert_eq!(getter.get_by_group::<u16>("missing", "port"), None);

        let objects: HashMap<String, Box<dyn Any + Send + Sync>> = HashMap::from([(
            "count".into(),
            Box::new(3_i32) as Box<dyn Any + Send + Sync>,
        )]);
        assert_eq!(objects.get_object::<i32>(&"count".into()), Some(&3));
        assert_eq!(objects.get_object::<String>(&"count".into()), None);
        assert_eq!(objects.get_object::<i32>(&"missing".into()), None);
    }
}
