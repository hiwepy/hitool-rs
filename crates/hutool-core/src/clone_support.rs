//! Rust-native equivalents of Hutool's `core.clone` package.

use std::{
    any::type_name,
    error::Error,
    fmt::{self, Debug, Display},
    ops::{Deref, DerefMut},
};

use crate::format_template;

/// Hutool's clone contract maps directly to Rust's ownership-safe [`Clone`].
pub use std::clone::Clone as Cloneable;

/// A transparent clone-support wrapper for values that implement [`Clone`].
#[repr(transparent)]
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct CloneSupport<T>(T);

impl<T> CloneSupport<T> {
    /// Wraps a value with an explicit clone-support type.
    pub const fn new(value: T) -> Self {
        Self(value)
    }

    /// Unwraps the owned value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for CloneSupport<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for CloneSupport<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Provides Hutool's generic `clone0` behavior without reflection.
pub trait DefaultCloneable: Clone {
    /// Clones `self` with the concrete return type preserved.
    #[must_use]
    fn clone0(&self) -> Self {
        self.clone()
    }
}

impl<T: Clone> DefaultCloneable for T {}

/// Clone failure with an optional standard Rust error source chain.
#[derive(Debug)]
pub struct CloneRuntimeException {
    message: String,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl CloneRuntimeException {
    /// Creates an exception from a message.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }

    /// Creates an exception whose message identifies and describes its source.
    pub fn from_error<E>(source: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        let kind = type_name::<E>()
            .rsplit("::")
            .next()
            .unwrap_or(type_name::<E>());
        Self {
            message: format!("{kind}: {source}"),
            source: Some(Box::new(source)),
        }
    }

    /// Creates an exception from a Hutool-style `{}` message template.
    #[must_use]
    pub fn formatted(template: &str, params: &[&dyn Display]) -> Self {
        Self::new(format_template(template, params))
    }

    /// Creates an exception from a message and error source.
    pub fn with_source<E>(message: impl Into<String>, source: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self {
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates a templated exception while preserving its error source.
    pub fn formatted_with_source<E>(source: E, template: &str, params: &[&dyn Display]) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self::with_source(format_template(template, params), source)
    }
}

impl Display for CloneRuntimeException {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for CloneRuntimeException {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &(dyn Error + 'static))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;

    #[derive(Debug)]
    struct SampleError;

    impl Display for SampleError {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("copy failed")
        }
    }

    impl Error for SampleError {}

    fn clone_through_contract<T: Cloneable>(value: &T) -> T {
        value.clone()
    }

    #[test]
    fn clone_contract_supports_owned_and_shared_rust_values() {
        let mut owned = CloneSupport::new(vec![1, 2]);
        owned.push(3);
        let cloned = clone_through_contract(&owned);
        owned.push(4);
        assert_eq!(&*cloned, &[1, 2, 3]);
        assert_eq!(owned.into_inner(), [1, 2, 3, 4]);

        let shared = Arc::new(Mutex::new(1));
        let shallow = shared.clone0();
        *shallow.lock().expect("mutex must remain healthy") = 2;
        assert_eq!(*shared.lock().expect("mutex must remain healthy"), 2);
    }

    #[test]
    fn clone_runtime_exception_preserves_messages_templates_and_sources() {
        let plain = CloneRuntimeException::new("plain failure");
        assert_eq!(plain.to_string(), "plain failure");
        assert!(plain.source().is_none());

        let from_error = CloneRuntimeException::from_error(SampleError);
        assert_eq!(from_error.to_string(), "SampleError: copy failed");
        assert_eq!(
            from_error.source().map(ToString::to_string).as_deref(),
            Some("copy failed")
        );

        let formatted = CloneRuntimeException::formatted("clone {} failed", &[&3]);
        assert_eq!(formatted.to_string(), "clone 3 failed");

        let sourced = CloneRuntimeException::with_source("explicit", SampleError);
        assert_eq!(sourced.to_string(), "explicit");
        assert!(sourced.source().is_some());

        let formatted_source = CloneRuntimeException::formatted_with_source(
            SampleError,
            "clone {} failed",
            &[&"record"],
        );
        assert_eq!(formatted_source.to_string(), "clone record failed");
        assert_eq!(
            formatted_source
                .source()
                .map(ToString::to_string)
                .as_deref(),
            Some("copy failed")
        );
    }
}
