use crate::{DEFAULT_ENCODING, SettingError, checked_path, read_text};
use encoding_rs::Encoding;
use indexmap::{IndexMap, IndexSet};
use std::path::{Path, PathBuf};

/// Insertion-ordered values partitioned by group.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GroupedMap(IndexMap<String, IndexMap<String, String>>);

impl GroupedMap {
    /// Creates an empty map.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Returns a value.
    #[must_use]
    pub fn get(&self, group: &str, key: &str) -> Option<&str> {
        self.0
            .get(group)
            .and_then(|m| m.get(key))
            .map(String::as_str)
    }
    /// Returns one group.
    #[must_use]
    pub fn group(&self, group: &str) -> Option<&IndexMap<String, String>> {
        self.0.get(group)
    }
    /// Counts values across groups.
    #[must_use]
    pub fn size(&self) -> usize {
        self.0.values().map(IndexMap::len).sum()
    }
    /// Inserts a value and returns the previous value.
    pub fn put(
        &mut self,
        group: impl Into<String>,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Option<String> {
        self.0
            .entry(group.into())
            .or_default()
            .insert(key.into(), value.into())
    }
    /// Extends one group.
    pub fn put_all<I, K, V>(&mut self, group: impl Into<String>, values: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        self.0
            .entry(group.into())
            .or_default()
            .extend(values.into_iter().map(|(k, v)| (k.into(), v.into())));
        self
    }
    /// Removes a value.
    pub fn remove(&mut self, group: &str, key: &str) -> Option<String> {
        self.0.get_mut(group).and_then(|m| m.shift_remove(key))
    }
    /// Tests whether a group is empty or absent.
    #[must_use]
    pub fn is_group_empty(&self, group: &str) -> bool {
        self.0.get(group).is_none_or(IndexMap::is_empty)
    }
    /// Tests whether all groups are empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.values().all(IndexMap::is_empty)
    }
    /// Tests a key.
    #[must_use]
    pub fn contains_key(&self, group: &str, key: &str) -> bool {
        self.0.get(group).is_some_and(|m| m.contains_key(key))
    }
    /// Tests a value.
    #[must_use]
    pub fn contains_value(&self, group: &str, value: &str) -> bool {
        self.0
            .get(group)
            .is_some_and(|m| m.values().any(|v| v == value))
    }
    /// Clears one group.
    pub fn clear_group(&mut self, group: &str) -> &mut Self {
        if let Some(values) = self.0.get_mut(group) {
            values.clear();
        }
        self
    }
    /// Clears all groups.
    pub fn clear(&mut self) {
        self.0.clear();
    }
    /// Returns group names.
    pub fn groups(&self) -> impl Iterator<Item = &str> {
        self.0.keys().map(String::as_str)
    }
    /// Returns a group's keys.
    pub fn keys(&self, group: &str) -> impl Iterator<Item = &str> {
        self.0
            .get(group)
            .into_iter()
            .flat_map(|m| m.keys().map(String::as_str))
    }
    /// Returns a group's values.
    pub fn values(&self, group: &str) -> impl Iterator<Item = &str> {
        self.0
            .get(group)
            .into_iter()
            .flat_map(|m| m.values().map(String::as_str))
    }
    /// Returns a group's entries.
    pub fn entries(&self, group: &str) -> impl Iterator<Item = (&str, &str)> {
        self.0
            .get(group)
            .into_iter()
            .flat_map(|m| m.iter().map(|(k, v)| (k.as_str(), v.as_str())))
    }
    /// Returns all groups.
    #[must_use]
    pub const fn as_map(&self) -> &IndexMap<String, IndexMap<String, String>> {
        &self.0
    }
}

/// Insertion-ordered grouped string sets loaded from Hutool's grouped-set syntax.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupedSet {
    groups: IndexMap<String, IndexSet<String>>,
    path: Option<PathBuf>,
    encoding: &'static Encoding,
}

impl Default for GroupedSet {
    fn default() -> Self {
        Self {
            groups: IndexMap::new(),
            path: None,
            encoding: DEFAULT_ENCODING,
        }
    }
}

impl GroupedSet {
    /// Creates an empty grouped set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            encoding: DEFAULT_ENCODING,
            ..Self::default()
        }
    }
    /// Loads a path using UTF-8.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, SettingError> {
        Self::from_path_with_encoding(path, DEFAULT_ENCODING)
    }
    /// Loads a path using an explicit encoding.
    pub fn from_path_with_encoding(
        path: impl AsRef<Path>,
        encoding: &'static Encoding,
    ) -> Result<Self, SettingError> {
        Self::from_path_ref(path.as_ref(), encoding)
    }
    fn from_path_ref(path: &Path, encoding: &'static Encoding) -> Result<Self, SettingError> {
        let path = checked_path(path)?;
        let mut value = Self {
            path: Some(path),
            encoding,
            ..Self::default()
        };
        value.reload()?;
        Ok(value)
    }
    /// Replaces content from text.
    pub fn load_text(&mut self, text: &str) {
        self.groups.clear();
        let mut group = String::new();
        for raw in text.lines() {
            let line = raw.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
                continue;
            }
            if line.starts_with('[') && line.ends_with(']') {
                group = line[1..line.len() - 1].trim().to_owned();
            } else {
                self.groups
                    .entry(group.clone())
                    .or_default()
                    .insert(line.to_owned());
            }
        }
    }
    /// Reloads the original path.
    pub fn reload(&mut self) -> Result<(), SettingError> {
        let path = self
            .path
            .as_deref()
            .ok_or_else(|| SettingError::Invalid("grouped set has no path".into()))?;
        let text = read_text(path, self.encoding)?;
        self.load_text(&text);
        Ok(())
    }
    /// Returns the original path.
    #[must_use]
    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }
    /// Returns group names.
    pub fn groups(&self) -> impl Iterator<Item = &str> {
        self.groups.keys().map(String::as_str)
    }
    /// Returns values in a group.
    #[must_use]
    pub fn values(&self, group: &str) -> Option<&IndexSet<String>> {
        self.groups.get(group)
    }
    /// Tests that every requested value exists.
    #[must_use]
    pub fn contains(&self, group: &str, values: &[&str]) -> bool {
        self.groups
            .get(group)
            .is_some_and(|set| values.iter().all(|v| set.contains(*v)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grouped_map_covers_ordered_map_semantics() {
        let mut map = GroupedMap::new();
        assert!(map.is_empty());
        assert!(map.is_group_empty("missing"));
        assert_eq!(map.put("db", "host", "localhost"), None);
        assert_eq!(map.put("db", "host", "db.local"), Some("localhost".into()));
        map.put_all("db", [("port", "5432"), ("user", "sa")]);
        map.put("", "root", "yes");
        assert_eq!(map.size(), 4);
        assert_eq!(map.get("db", "host"), Some("db.local"));
        assert_eq!(map.group("db").unwrap().len(), 3);
        assert!(map.contains_key("db", "port"));
        assert!(map.contains_value("db", "sa"));
        assert_eq!(map.groups().collect::<Vec<_>>(), ["db", ""]);
        assert_eq!(map.keys("db").collect::<Vec<_>>(), ["host", "port", "user"]);
        assert_eq!(map.values("db").count(), 3);
        assert_eq!(map.entries("db").count(), 3);
        assert_eq!(map.as_map().len(), 2);
        assert_eq!(map.remove("db", "port"), Some("5432".into()));
        assert_eq!(map.remove("missing", "x"), None);
        map.clear_group("missing").clear_group("db");
        assert!(map.is_group_empty("db"));
        map.clear();
        assert!(map.is_empty());
    }

    #[test]
    fn grouped_set_loads_comments_groups_and_reload() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("roles.grouped");
        std::fs::write(
            &path,
            "# comment\nroot\n[admin]\nread\nwrite\n;ignored\n[user]\nread\n",
        )
        .unwrap();
        let mut set = GroupedSet::from_path(&path).unwrap();
        assert_eq!(set.path(), Some(path.as_path()));
        assert_eq!(set.groups().collect::<Vec<_>>(), ["", "admin", "user"]);
        assert!(set.contains("admin", &["read", "write"]));
        assert!(!set.contains("admin", &["missing"]));
        assert!(!set.contains("missing", &["read"]));
        assert_eq!(set.values("").unwrap().len(), 1);
        std::fs::write(&path, "[new]\nvalue\n").unwrap();
        set.reload().unwrap();
        assert!(set.contains("new", &["value"]));
        std::fs::remove_file(&path).unwrap();
        assert!(set.reload().is_err());
        let mut detached = GroupedSet::new();
        detached.load_text("\n# only\n[group]\nx\n");
        assert!(detached.reload().is_err());
        assert!(GroupedSet::from_path("").is_err());
        assert!(GroupedSet::from_path(directory.path().join("missing")).is_err());
    }
}
