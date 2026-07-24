use crate::SettingError;
use std::path::{Path, PathBuf};

use super::props::Props;

/// Properties lookup helpers.
pub struct PropsUtil;
impl PropsUtil {
    /// Loads a properties file, appending `.properties` when absent.
    pub fn get(name: impl AsRef<Path>) -> Result<Props, SettingError> {
        Props::from_path(super::setting::fix_extension(name.as_ref(), "properties"))
    }
    /// Loads the first existing file.
    pub fn get_first_found<I, P>(names: I) -> Result<Option<Props>, SettingError>
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        let names: Vec<PathBuf> = names
            .into_iter()
            .map(|name| name.as_ref().to_path_buf())
            .collect();
        Self::get_first_found_paths(&names)
    }
    fn get_first_found_paths(names: &[PathBuf]) -> Result<Option<Props>, SettingError> {
        for name in names {
            let path = super::setting::fix_extension(name, "properties");
            if path.is_file() {
                return Props::from_path(path).map(Some);
            }
        }
        Ok(None)
    }
    /// Captures environment variables as explicit properties.
    #[must_use]
    pub fn get_system_props() -> Props {
        Props::from_map(std::env::vars().collect())
    }
}
