use std::{fmt, ops::Index};

use serde::Serialize;
use serde_json::{Map, Number, Value};

use crate::{JsonError, Result};

mod json_config;
mod path_error;
mod json_container;
mod json_object;
mod json_array;
mod json_null;

pub use json_config::JSONConfig;
pub use path_error::PathError;
pub use json_container::JsonContainer;
pub use json_object::JSONObject;
pub use json_array::JSONArray;
pub use json_null::JSONNull;
pub use json_object::get_by_path;
pub use json_object::put_by_path;
