//! Mutable value containers corresponding to Hutool's `core.lang.mutable` package.
//!
//! Rust normally prefers ordinary mutable bindings. These small wrappers are
//! useful when mutation itself must be passed around as a value, while keeping
//! ownership and borrowing explicit.

#![allow(

/// Shared contract for owned mutable value wrappers.
pub trait Mutable<T> {
    /// Borrows the current value.
    fn get(&self) -> &T;

    /// Mutably borrows the current value.
    fn get_mut(&mut self) -> &mut T;

    /// Replaces the current value.
    fn set(&mut self, value: T);
}
