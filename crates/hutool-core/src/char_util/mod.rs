use std::any::{Any, TypeId};

use unicode_general_category::{GeneralCategory, get_general_category};

mod char_error;
mod char_util;

pub use char_error::CharError;
pub use char_util::CharUtil;
