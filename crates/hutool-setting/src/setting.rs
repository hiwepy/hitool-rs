use crate::{DEFAULT_ENCODING, GroupedMap, Props, SettingError, SettingLoader, checked_path};
use encoding_rs::Encoding;
use indexmap::IndexMap;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use serde::de::DeserializeOwned;
use std::{
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

/// Owns a filesystem watcher. Dropping it stops automatic reload.
pub struct AutoLoadHandle {
    _watcher: Box<dyn WatchOwner>,
}
impl std::fmt::Debug for AutoLoadHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AutoLoadHandle").finish_non_exhaustive()
    }
}

/// Hutool-compatible grouped setting document.
#[derive(Debug, Clone)]
pub struct Setting {
    data: Arc<RwLock<GroupedMap>>,
    path: Option<PathBuf>,
    charset: &'static Encoding,
    use_variable: bool,
    variable_prefix: String,
    variable_suffix: String,
}

impl Default for Setting {
    fn default() -> Self {
        Self::new()
    }
}

impl Setting {
    /// Creates an empty setting.
    #[must_use]
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(GroupedMap::new())),
            path: None,
            charset: DEFAULT_ENCODING,
            use_variable: false,
            variable_prefix: "${".into(),
            variable_suffix: "}".into(),
        }
    }
    /// Alias of `new`, matching Hutool.
    #[must_use]
    pub fn create() -> Self {
        Self::new()
    }
    /// Loads a UTF-8 path.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, SettingError> {
        Self::from_path_with_options(path, DEFAULT_ENCODING, false)
    }
    /// Loads a path with explicit options.
    pub fn from_path_with_options(
        path: impl AsRef<Path>,
        charset: &'static Encoding,
        use_variable: bool,
    ) -> Result<Self, SettingError> {
        Self::from_path_ref(path.as_ref(), charset, use_variable)
    }
    fn from_path_ref(
        path: &Path,
        charset: &'static Encoding,
        use_variable: bool,
    ) -> Result<Self, SettingError> {
        let path = checked_path(path)?;
        let mut value = Self {
            path: Some(path),
            charset,
            use_variable,
            ..Self::new()
        };
        value.load()?;
        Ok(value)
    }
    fn loader(&self) -> SettingLoader {
        let mut loader = SettingLoader::new(self.charset, self.use_variable);
        loader
            .set_var_regex(self.variable_prefix.clone(), self.variable_suffix.clone())
            .expect("stored delimiters are valid");
        loader
    }
    /// Reloads the configured path atomically.
    pub fn load(&mut self) -> Result<bool, SettingError> {
        let path = self
            .path
            .as_deref()
            .ok_or_else(|| SettingError::Invalid("setting has no path".into()))?;
        let bytes = std::fs::read(path)?;
        let mut parsed = GroupedMap::new();
        let mut reader = bytes.as_slice();
        self.loader().load(&mut reader, &mut parsed)?;
        *self.data.write().expect("setting poisoned") = parsed;
        Ok(true)
    }
    /// Starts explicit automatic reload and returns its owner.
    pub fn auto_load<F>(&self, callback: F) -> Result<AutoLoadHandle, SettingError>
    where
        F: Fn(bool) + Send + Sync + 'static,
    {
        self.auto_load_arc(Arc::new(callback))
    }

    fn auto_load_arc(
        &self,
        callback: Arc<dyn Fn(bool) + Send + Sync>,
    ) -> Result<AutoLoadHandle, SettingError> {
        self.auto_load_arc_with(callback, create_recommended_watcher)
    }

    fn auto_load_arc_with(
        &self,
        callback: Arc<dyn Fn(bool) + Send + Sync>,
        factory: fn(ReloadHandler) -> Result<Box<dyn WatchOwner>, notify::Error>,
    ) -> Result<AutoLoadHandle, SettingError> {
        let path = self
            .path
            .clone()
            .ok_or_else(|| SettingError::Invalid("setting has no path".into()))?;
        let data = Arc::clone(&self.data);
        let loader = self.loader();
        let handler = ReloadHandler {
            path: path.clone(),
            loader,
            data,
            callback,
        };
        let mut watcher = factory(handler)?;
        watcher.watch_path(&path)?;
        Ok(AutoLoadHandle { _watcher: watcher })
    }
    /// Returns the source path.
    #[must_use]
    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }
    /// Counts entries.
    #[must_use]
    pub fn size(&self) -> usize {
        self.data.read().expect("setting poisoned").size()
    }
    /// Gets a value by group.
    #[must_use]
    pub fn get_by_group(&self, key: &str, group: &str) -> Option<String> {
        self.data
            .read()
            .expect("setting poisoned")
            .get(group, key)
            .map(str::to_owned)
    }
    /// Gets a default-group value.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<String> {
        self.get_by_group(key, "")
    }
    /// Gets a value or default.
    #[must_use]
    pub fn get_or(&self, key: &str, group: &str, default: impl Into<String>) -> String {
        self.get_by_group(key, group)
            .unwrap_or_else(|| default.into())
    }
    /// Gets a non-empty value or default.
    #[must_use]
    pub fn get_not_empty_or(&self, key: &str, group: &str, default: impl Into<String>) -> String {
        self.get_by_group(key, group)
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| default.into())
    }
    /// Parses a typed value.
    pub fn get_parse<T: std::str::FromStr>(&self, key: &str, group: &str) -> Option<T> {
        self.get_by_group(key, group)?.parse().ok()
    }
    /// Splits a value.
    #[must_use]
    pub fn get_strings(&self, key: &str, group: &str, delimiter: &str) -> Option<Vec<String>> {
        let value = self.get_by_group(key, group)?;
        if value.trim().is_empty() {
            None
        } else {
            Some(
                value
                    .split(delimiter)
                    .map(|v| v.trim().to_owned())
                    .collect(),
            )
        }
    }
    /// Gets and removes the first matching key from the default group.
    pub fn get_and_remove(&self, keys: &[&str]) -> Option<String> {
        let mut data = self.data.write().expect("setting poisoned");
        for key in keys {
            if let Some(value) = data.remove("", key) {
                return Some(value);
            }
        }
        None
    }
    /// Returns a group snapshot.
    #[must_use]
    pub fn get_map(&self, group: &str) -> IndexMap<String, String> {
        self.data
            .read()
            .expect("setting poisoned")
            .group(group)
            .cloned()
            .unwrap_or_default()
    }
    /// Returns a subgroup as a setting.
    #[must_use]
    pub fn get_setting(&self, group: &str) -> Self {
        let result = Self::new();
        *result.data.write().expect("setting poisoned") = GroupedMap::new();
        result
            .data
            .write()
            .expect("setting poisoned")
            .put_all("", self.get_map(group));
        result
    }
    /// Returns a group as properties.
    #[must_use]
    pub fn get_props(&self, group: &str) -> Props {
        Props::from_map(self.get_map(group))
    }
    /// Serializes a group into a map.
    #[must_use]
    pub fn to_properties(&self, group: &str) -> IndexMap<String, String> {
        self.get_map(group)
    }
    /// Stores the whole document.
    pub fn store(&self, path: impl AsRef<Path>) -> Result<(), SettingError> {
        self.store_path(path.as_ref())
    }
    fn store_path(&self, path: &Path) -> Result<(), SettingError> {
        let mut file = std::fs::File::create(path)?;
        self.loader()
            .store(&self.data.read().expect("setting poisoned"), &mut file)
    }
    /// Returns a data snapshot.
    #[must_use]
    pub fn grouped_map(&self) -> GroupedMap {
        self.data.read().expect("setting poisoned").clone()
    }
    /// Returns group names.
    #[must_use]
    pub fn groups(&self) -> Vec<String> {
        self.data
            .read()
            .expect("setting poisoned")
            .groups()
            .map(str::to_owned)
            .collect()
    }
    /// Changes variable delimiters.
    pub fn set_var_regex(
        &mut self,
        prefix: impl Into<String>,
        suffix: impl Into<String>,
    ) -> Result<&mut Self, SettingError> {
        let mut loader = self.loader();
        loader.set_var_regex(prefix, suffix)?;
        self.variable_prefix = loader.variable_prefix;
        self.variable_suffix = loader.variable_suffix;
        Ok(self)
    }
    /// Changes decoding for subsequent loads.
    pub fn set_charset(&mut self, charset: &'static Encoding) -> &mut Self {
        self.charset = charset;
        self
    }
    /// Tests emptiness.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.read().expect("setting poisoned").is_empty()
    }
    /// Tests a group key.
    #[must_use]
    pub fn contains_key(&self, group: &str, key: &str) -> bool {
        self.data
            .read()
            .expect("setting poisoned")
            .contains_key(group, key)
    }
    /// Tests a group value.
    #[must_use]
    pub fn contains_value(&self, group: &str, value: &str) -> bool {
        self.data
            .read()
            .expect("setting poisoned")
            .contains_value(group, value)
    }
    /// Inserts a group value.
    pub fn put_by_group(
        &self,
        key: impl Into<String>,
        group: impl Into<String>,
        value: impl Into<String>,
    ) -> Option<String> {
        self.data
            .write()
            .expect("setting poisoned")
            .put(group, key, value)
    }
    /// Inserts a default-group value.
    pub fn set(&self, key: impl Into<String>, value: impl Into<String>) -> &Self {
        self.put_by_group(key, "", value);
        self
    }
    /// Extends a group.
    pub fn put_all<I, K, V>(&self, group: impl Into<String>, values: I) -> &Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        self.data
            .write()
            .expect("setting poisoned")
            .put_all(group, values);
        self
    }
    /// Merges another setting.
    pub fn add_setting(&self, other: &Self) -> &Self {
        for group in other.groups() {
            self.put_all(group.clone(), other.get_map(&group));
        }
        self
    }
    /// Removes a group key.
    pub fn remove(&self, group: &str, key: &str) -> Option<String> {
        self.data
            .write()
            .expect("setting poisoned")
            .remove(group, key)
    }
    /// Clears all values.
    pub fn clear(&self) {
        self.data.write().expect("setting poisoned").clear();
    }
    /// Deserializes a group into a typed object.
    pub fn to_bean<T: DeserializeOwned>(&self, group: &str) -> Result<T, SettingError> {
        self.to_bean_with(group, crate::config_from_string_map)
    }
    fn to_bean_with<T: DeserializeOwned>(
        &self,
        group: &str,
        converter: fn(
            &std::collections::HashMap<String, String>,
        ) -> Result<config::Config, config::ConfigError>,
    ) -> Result<T, SettingError> {
        let values: std::collections::HashMap<String, String> =
            self.get_map(group).into_iter().collect();
        Ok(converter(&values)?.try_deserialize()?)
    }
}

pub(crate) fn fix_extension(path: &Path, extension: &str) -> PathBuf {
    if path.extension().is_none() {
        path.with_extension(extension)
    } else {
        path.to_path_buf()
    }
}

fn reload_event(
    event: Result<notify::Event, notify::Error>,
    path: &Path,
    loader: &SettingLoader,
    data: &RwLock<GroupedMap>,
    callback: &dyn Fn(bool),
) {
    let success = event.is_ok_and(|event| event.paths.iter().any(|candidate| candidate == path));
    if success {
        if let Ok(bytes) = std::fs::read(path) {
            let mut parsed = GroupedMap::new();
            let mut reader = bytes.as_slice();
            if loader.load(&mut reader, &mut parsed).is_ok() {
                *data.write().expect("setting poisoned") = parsed;
                callback(true);
                return;
            }
        }
    }
    callback(false);
}

struct ReloadHandler {
    path: PathBuf,
    loader: SettingLoader,
    data: Arc<RwLock<GroupedMap>>,
    callback: Arc<dyn Fn(bool) + Send + Sync>,
}

impl notify::EventHandler for ReloadHandler {
    fn handle_event(&mut self, event: Result<notify::Event, notify::Error>) {
        reload_event(
            event,
            &self.path,
            &self.loader,
            &self.data,
            self.callback.as_ref(),
        );
    }
}

trait WatchOwner: Send {
    fn watch_path(&mut self, path: &Path) -> Result<(), notify::Error>;
}

impl WatchOwner for RecommendedWatcher {
    fn watch_path(&mut self, path: &Path) -> Result<(), notify::Error> {
        self.watch(path, RecursiveMode::NonRecursive)
    }
}

fn create_recommended_watcher(
    handler: ReloadHandler,
) -> Result<Box<dyn WatchOwner>, notify::Error> {
    notify::recommended_watcher(handler).map(box_watcher)
}
fn box_watcher(watcher: RecommendedWatcher) -> Box<dyn WatchOwner> {
    Box::new(watcher)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{SettingLoader, SettingUtil};
    use serde::Deserialize;
    use std::io::{Read, Write};

    fn ignore_reload(_: bool) {}
    fn failing_config(
        _: &std::collections::HashMap<String, String>,
    ) -> Result<config::Config, config::ConfigError> {
        Err(config::ConfigError::Message("intentional".into()))
    }
    fn load_slice(
        loader: &SettingLoader,
        bytes: &[u8],
        destination: &mut GroupedMap,
    ) -> Result<(), SettingError> {
        let mut reader = bytes;
        loader.load(&mut reader, destination)
    }
    struct FailingIo;
    impl Read for FailingIo {
        fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
            Err(std::io::Error::other("intentional"))
        }
    }
    impl Write for FailingIo {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::other("intentional"))
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    struct FailingWatch;
    impl WatchOwner for FailingWatch {
        fn watch_path(&mut self, _: &Path) -> Result<(), notify::Error> {
            Err(notify::Error::generic("intentional"))
        }
    }
    fn failing_factory(_: ReloadHandler) -> Result<Box<dyn WatchOwner>, notify::Error> {
        Err(notify::Error::generic("intentional"))
    }
    #[allow(clippy::unnecessary_wraps)]
    fn failing_watch_factory(_: ReloadHandler) -> Result<Box<dyn WatchOwner>, notify::Error> {
        Ok(Box::new(FailingWatch))
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Db {
        host: String,
        port: u16,
    }

    #[test]
    fn setting_loads_variables_groups_mutations_and_storage() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("app.setting");
        std::fs::write(&path,"root = /srv\n[db]\nhost = localhost\nport = 5432\nurl = postgres://${host}:${port}\nshared = ${root}\nempty =\n").unwrap();
        let mut setting = Setting::from_path_with_options(&path, DEFAULT_ENCODING, true).unwrap();
        assert_eq!(setting.path(), Some(path.as_path()));
        assert_eq!(setting.size(), 6);
        assert_eq!(setting.get("root"), Some("/srv".into()));
        assert_eq!(
            setting.get_by_group("url", "db"),
            Some("postgres://localhost:5432".into())
        );
        assert_eq!(setting.get_or("missing", "db", "fallback"), "fallback");
        assert_eq!(
            setting.get_not_empty_or("empty", "db", "fallback"),
            "fallback"
        );
        assert_eq!(setting.get_parse::<u16>("port", "db"), Some(5432));
        assert_eq!(setting.get_parse::<u16>("missing", "db"), None);
        assert_eq!(setting.get_strings("url", "db", ":").unwrap().len(), 3);
        assert_eq!(setting.get_strings("empty", "db", ","), None);
        assert_eq!(setting.get_strings("missing", "db", ","), None);
        let bean: Db = setting.to_bean("db").unwrap();
        assert!(setting.to_bean::<Db>("missing").is_err());
        assert!(setting.to_bean_with::<Db>("db", failing_config).is_err());
        assert_eq!(
            bean,
            Db {
                host: "localhost".into(),
                port: 5432
            }
        );
        assert_eq!(setting.groups(), [String::new(), "db".to_owned()]);
        assert!(setting.contains_key("db", "host"));
        assert!(setting.contains_value("db", "5432"));
        assert_eq!(setting.get_map("db").len(), 5);
        assert_eq!(
            setting.get_setting("db").get("host"),
            Some("localhost".into())
        );
        assert_eq!(setting.get_props("db").get_int("port"), Some(5432));
        assert_eq!(setting.to_properties("db").len(), 5);
        setting.put_by_group("user", "db", "sa");
        setting.set("mode", "prod");
        setting.put_all("extra", [("a", "1"), ("b", "2")]);
        let other = Setting::new();
        other.set("merged", "yes");
        setting.add_setting(&other);
        assert_eq!(setting.remove("extra", "a"), Some("1".into()));
        assert_eq!(
            setting.get_and_remove(&["none", "mode"]),
            Some("prod".into())
        );
        assert_eq!(setting.get_and_remove(&["none"]), None);
        assert!(setting.set_var_regex("$(", ")").is_ok());
        assert!(setting.set_var_regex("", ")").is_err());
        setting.set_charset(DEFAULT_ENCODING);
        let saved = directory.path().join("saved.setting");
        setting.store(&saved).unwrap();
        assert!(setting.store(directory.path()).is_err());
        assert!(std::fs::read_to_string(saved).unwrap().contains("[db]"));
        setting.clear();
        assert!(setting.is_empty());
        assert!(setting.grouped_map().is_empty());
        assert_eq!(Setting::create().size(), 0);
        assert_eq!(Setting::default().size(), 0);
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn loader_and_util_reject_invalid_syntax_and_find_files() {
        let mut destination = GroupedMap::new();
        let mut loader = SettingLoader::default();
        assert!(loader.set_var_regex("", "}").is_err());
        loader.set_assign_flag(':');
        let utf8_loader = SettingLoader::new(DEFAULT_ENCODING, false);
        let decode_error =
            load_slice(&utf8_loader, [0xff].as_slice(), &mut destination).unwrap_err();
        assert!(decode_error.to_string().contains("UTF-8"));
        assert!(load_slice(&loader, "bad line".as_bytes(), &mut destination).is_err());
        assert!(load_slice(&loader, ": value".as_bytes(), &mut destination).is_err());
        load_slice(
            &loader,
            "\n# comment\n; other\n[g]\nk: v\n".as_bytes(),
            &mut destination,
        )
        .unwrap();
        let mut output = Vec::new();
        loader.store(&destination, &mut output).unwrap();
        assert!(loader.load(&mut FailingIo, &mut destination).is_err());
        assert!(loader.store(&destination, &mut FailingIo).is_err());
        let mut default_only = GroupedMap::new();
        default_only.put("", "x", "1");
        assert!(loader.store(&default_only, &mut FailingIo).is_err());
        assert!(FailingIo.flush().is_ok());
        assert_eq!(String::from_utf8(output).unwrap(), "[g]\nk : v\n");
        assert!(Setting::new().load().is_err());
        assert!(Setting::new().auto_load(ignore_reload).is_err());
        assert!(Setting::from_path("").is_err());
        let directory = tempfile::tempdir().unwrap();
        let base = directory.path().join("first");
        std::fs::write(base.with_extension("setting"), "x=1\n").unwrap();
        assert_eq!(SettingUtil::get(&base).unwrap().get("x"), Some("1".into()));
        assert!(
            SettingUtil::get_first_found([directory.path().join("none"), base.clone()])
                .unwrap()
                .is_some()
        );
        assert!(
            SettingUtil::get_first_found([directory.path().join("none")])
                .unwrap()
                .is_none()
        );
        assert_eq!(
            fix_extension(Path::new("has.toml"), "setting"),
            PathBuf::from("has.toml")
        );
        let invalid_path = directory.path().join("invalid.setting");
        std::fs::write(&invalid_path, "not valid\n").unwrap();
        let mut invalid_setting = Setting::from_path(base.with_extension("setting")).unwrap();
        invalid_setting.path = Some(invalid_path);
        assert!(invalid_setting.load().is_err());
        let variables = SettingLoader::new(DEFAULT_ENCODING, true);
        load_slice(
            &variables,
            "x=${missing}\ny=${unterminated\n".as_bytes(),
            &mut destination,
        )
        .unwrap();
        assert_eq!(destination.get("", "x"), Some("${missing}"));
        assert_eq!(destination.get("", "y"), Some("${unterminated"));

        let watched = directory.path().join("watched.setting");
        std::fs::write(&watched, "x=1\n").unwrap();
        let watched_setting = Setting::from_path(&watched).unwrap();
        let handle = watched_setting.auto_load(ignore_reload).unwrap();
        assert!(format!("{handle:?}").starts_with("AutoLoadHandle"));
        assert!(
            watched_setting
                .auto_load_arc_with(Arc::new(ignore_reload), failing_factory)
                .is_err()
        );
        assert!(
            watched_setting
                .auto_load_arc_with(Arc::new(ignore_reload), failing_watch_factory)
                .is_err()
        );

        let data = RwLock::new(GroupedMap::new());
        let outcomes = std::sync::Mutex::new(Vec::new());
        let callback = |value| outcomes.lock().unwrap().push(value);
        reload_event(
            Ok(notify::Event::new(notify::EventKind::Any)),
            &watched,
            &variables,
            &data,
            &callback,
        );
        let matching = notify::Event::new(notify::EventKind::Any).add_path(watched.clone());
        reload_event(Ok(matching), &watched, &variables, &data, &callback);
        std::fs::write(&watched, "invalid\n").unwrap();
        let invalid = notify::Event::new(notify::EventKind::Any).add_path(watched.clone());
        reload_event(Ok(invalid), &watched, &variables, &data, &callback);
        std::fs::remove_file(&watched).unwrap();
        let missing = notify::Event::new(notify::EventKind::Any).add_path(watched.clone());
        reload_event(Ok(missing), &watched, &variables, &data, &callback);
        assert_eq!(*outcomes.lock().unwrap(), [false, true, false, false]);
        std::fs::write(&watched, "x=3\n").unwrap();
        let mut handler = ReloadHandler {
            path: watched.clone(),
            loader: variables,
            data: Arc::new(RwLock::new(GroupedMap::new())),
            callback: Arc::new(ignore_reload),
        };
        notify::EventHandler::handle_event(
            &mut handler,
            Ok(notify::Event::new(notify::EventKind::Any).add_path(watched)),
        );
    }
}
