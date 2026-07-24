//! Mutable value containers corresponding to Hutool's `core.lang.mutable` package.
//!
//! Rust normally prefers ordinary mutable bindings. These small wrappers are
//! useful when mutation itself must be passed around as a value, while keeping
//! ownership and borrowing explicit.

#![allow(

mod mutable;
mod mutable_obj;
mod mutable_bool;
mod mutable_pair;

pub use mutable::Mutable;
pub use mutable_obj::MutableObj;
pub use mutable_bool::MutableBool;
pub use mutable_pair::MutablePair;
