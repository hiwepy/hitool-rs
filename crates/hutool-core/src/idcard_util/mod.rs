use std::fmt;

use chrono::{Datelike, Local, NaiveDate};
use thiserror::Error;

mod idcard_error;
mod card10_info;
mod idcard_util;
mod idcard;

pub use idcard_error::IdcardError;
pub use card10_info::Card10Info;
pub use idcard_util::IdcardUtil;
pub use idcard::Idcard;
