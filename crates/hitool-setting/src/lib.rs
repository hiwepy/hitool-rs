//! Layered settings aligned with Hutool's setting module.

#![forbid(unsafe_code)]

pub use config::{Config, ConfigError, Environment, File, FileFormat};
use serde::de::DeserializeOwned;
use std::path::PathBuf;

/// One configuration file and whether it must exist.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileSource {
    /// File path; its extension selects TOML, JSON, YAML, and other formats.
    pub path: PathBuf,
    /// Whether loading fails when the file does not exist.
    pub required: bool,
}

/// Declarative settings sources, applied in the order files then environment.
#[derive(Debug, Clone, Default)]
pub struct SettingsLoader {
    files: Vec<FileSource>,
    environment_prefix: Option<String>,
    environment_separator: String,
}

impl SettingsLoader {
    /// Creates an empty loader.
    #[must_use]
    pub fn new() -> Self {
        Self {
            environment_separator: "__".to_owned(),
            ..Self::default()
        }
    }

    /// Adds a required configuration file.
    #[must_use]
    pub fn required_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.files.push(FileSource {
            path: path.into(),
            required: true,
        });
        self
    }

    /// Adds an optional configuration file.
    #[must_use]
    pub fn optional_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.files.push(FileSource {
            path: path.into(),
            required: false,
        });
        self
    }

    /// Adds environment variables with the given prefix.
    #[must_use]
    pub fn environment(mut self, prefix: impl Into<String>) -> Self {
        self.environment_prefix = Some(prefix.into());
        self
    }

    /// Builds the merged dynamic configuration.
    pub fn build(&self) -> Result<Config, ConfigError> {
        let mut builder = Config::builder();
        for source in &self.files {
            builder = builder.add_source(File::from(source.path.clone()).required(source.required));
        }
        if let Some(prefix) = &self.environment_prefix {
            builder = builder.add_source(
                Environment::with_prefix(prefix)
                    .prefix_separator("_")
                    .separator(&self.environment_separator),
            );
        }
        builder.build()
    }

    /// Builds and deserializes the merged settings into a typed value.
    pub fn load<T: DeserializeOwned>(&self) -> Result<T, ConfigError> {
        self.build()?.try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct AppSettings {
        name: String,
        workers: u16,
    }

    #[test]
    fn loads_a_typed_json_file() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/app.json");
        let loaded: AppSettings = SettingsLoader::new().required_file(path).load().unwrap();
        assert_eq!(
            loaded,
            AppSettings {
                name: "hitool".into(),
                workers: 4
            }
        );
    }
}
