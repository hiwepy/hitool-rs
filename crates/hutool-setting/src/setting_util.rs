use crate::{Setting, SettingError};
use std::path::{Path, PathBuf};

/// Path-based convenience operations.
pub struct SettingUtil;
impl SettingUtil {
    /// Loads a setting, appending `.setting` when absent.
    pub fn get(name: impl AsRef<Path>) -> Result<Setting, SettingError> {
        Setting::from_path(super::setting::fix_extension(name.as_ref(), "setting"))
    }
    /// Loads the first existing name.
    pub fn get_first_found<I, P>(names: I) -> Result<Option<Setting>, SettingError>
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
    fn get_first_found_paths(names: &[PathBuf]) -> Result<Option<Setting>, SettingError> {
        for name in names {
            let path = super::setting::fix_extension(name, "setting");
            if path.is_file() {
                return Setting::from_path(path).map(Some);
            }
        }
        Ok(None)
    }
}
