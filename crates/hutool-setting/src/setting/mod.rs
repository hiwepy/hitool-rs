use crate::{DEFAULT_ENCODING, GroupedMap, Props, SettingError, SettingLoader, checked_path};
use encoding_rs::Encoding;
use indexmap::IndexMap;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use serde::de::DeserializeOwned;
use std::{
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

mod auto_load_handle;
mod setting;

pub use auto_load_handle::AutoLoadHandle;
pub use setting::Setting;
