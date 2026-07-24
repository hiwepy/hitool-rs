//! Mutable value containers corresponding to Hutool's `core.lang.mutable` package.
//!
//! Rust normally prefers ordinary mutable bindings. These small wrappers are
//! useful when mutation itself must be passed around as a value, while keeping
//! ownership and borrowing explicit.

#![allow(

use super::mutable::Mutable;

/// Generic owned mutable value.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MutableObj<T> {
    value: T,
}

impl<T> MutableObj<T> {
    /// Creates a mutable object around `value`.
    pub const fn new(value: T) -> Self {
        Self { value }
    }

    /// Hutool-style factory alias.
    pub const fn of(value: T) -> Self {
        Self::new(value)
    }

    /// Borrows the wrapped value.
    pub const fn get(&self) -> &T {
        &self.value
    }

    /// Mutably borrows the wrapped value.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }

    /// Replaces the wrapped value.
    pub fn set(&mut self, value: T) {
        self.value = value;
    }

    /// Consumes the wrapper and returns its value.
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T> Mutable<T> for MutableObj<T> {
    fn get(&self) -> &T {
        self.get()
    }

    fn get_mut(&mut self) -> &mut T {
        self.get_mut()
    }

    fn set(&mut self, value: T) {
        self.set(value);
    }
}

impl<T> From<T> for MutableObj<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: fmt::Display> fmt::Display for MutableObj<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(formatter)
    }
}

impl MutableFloat {
    /// Returns the exact Java `Float.hashCode` value.
    pub fn java_hash_code(&self) -> i32 {
        i32::from_ne_bytes(self.canonical_bits().to_ne_bytes())
    }
}

impl MutableDouble {
    /// Returns the exact Java `Double.hashCode` value.
    pub fn java_hash_code(&self) -> i32 {
        java_i64_hash(i64::from_ne_bytes(self.canonical_bits().to_ne_bytes()))
    }
}

macro_rules! mutable_integer {
    ($name:ident, $primitive:ty, $java_hash:expr) => {
        #[doc = concat!("Mutable `", stringify!($primitive), "` value.")]
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name {
            value: $primitive,
        }

        impl $name {
            /// Creates a mutable numeric value.
            pub const fn new(value: $primitive) -> Self {
                Self { value }
            }

            /// Returns the current primitive value.
            pub const fn get(&self) -> $primitive {
                self.value
            }

            /// Replaces the current value.
            pub fn set(&mut self, value: $primitive) {
                self.value = value;
            }

            /// Adds one using Java-compatible wrapping integer arithmetic.
            pub fn increment(&mut self) -> &mut Self {
                self.value = self.value.wrapping_add(1);
                self
            }

            /// Subtracts one using Java-compatible wrapping integer arithmetic.
            pub fn decrement(&mut self) -> &mut Self {
                self.value = self.value.wrapping_sub(1);
                self
            }

            /// Adds an operand using Java-compatible wrapping arithmetic.
            pub fn add(&mut self, operand: $primitive) -> &mut Self {
                self.value = self.value.wrapping_add(operand);
                self
            }

            /// Subtracts an operand using Java-compatible wrapping arithmetic.
            pub fn subtract(&mut self, operand: $primitive) -> &mut Self {
                self.value = self.value.wrapping_sub(operand);
                self
            }

            /// Returns the Java `byteValue` conversion.
            pub const fn byte_value(&self) -> i8 {
                self.value as i8
            }

            /// Returns the Java `shortValue` conversion.
            pub const fn short_value(&self) -> i16 {
                self.value as i16
            }

            /// Returns the Java `intValue` conversion.
            pub const fn int_value(&self) -> i32 {
                self.value as i32
            }

            /// Returns the Java `longValue` conversion.
            pub const fn long_value(&self) -> i64 {
                self.value as i64
            }

            /// Returns the Java `floatValue` conversion.
            pub const fn float_value(&self) -> f32 {
                self.value as f32
            }

            /// Returns the Java `doubleValue` conversion.
            pub const fn double_value(&self) -> f64 {
                self.value as f64
            }

            /// Returns the exact Java wrapper hash code used by Hutool.
            pub fn java_hash_code(&self) -> i32 {
                ($java_hash)(self.value)
            }

            /// Consumes the wrapper and returns its value.
            pub const fn into_inner(self) -> $primitive {
                self.value
            }
        }

        impl Mutable<$primitive> for $name {
            fn get(&self) -> &$primitive {
                &self.value
            }

            fn get_mut(&mut self) -> &mut $primitive {
                &mut self.value
            }

            fn set(&mut self, value: $primitive) {
                self.value = value;
            }
        }

        impl From<$primitive> for $name {
            fn from(value: $primitive) -> Self {
                Self::new(value)
            }
        }

        impl FromStr for $name {
            type Err = <$primitive as FromStr>::Err;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                value.parse().map(Self::new)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.value.fmt(formatter)
            }
        }
    };
}

macro_rules! mutable_float {
    ($name:ident, $primitive:ty, $bits:ty, $canonical:ident, $compare:ident) => {
        #[doc = concat!("Mutable `", stringify!($primitive), "` value with Java equality semantics.")]
        #[derive(Debug, Clone, Copy, Default)]
        pub struct $name {
            value: $primitive,
        }

        impl $name {
            /// Creates a mutable floating-point value.
            pub const fn new(value: $primitive) -> Self {
                Self { value }
            }

            /// Returns the current primitive value.
            pub const fn get(&self) -> $primitive {
                self.value
            }

            /// Replaces the current value.
            pub fn set(&mut self, value: $primitive) {
                self.value = value;
            }

            /// Adds one.
            pub fn increment(&mut self) -> &mut Self {
                self.value += 1.0;
                self
            }

            /// Subtracts one.
            pub fn decrement(&mut self) -> &mut Self {
                self.value -= 1.0;
                self
            }

            /// Adds an operand.
            pub fn add(&mut self, operand: $primitive) -> &mut Self {
                self.value += operand;
                self
            }

            /// Subtracts an operand.
            pub fn subtract(&mut self, operand: $primitive) -> &mut Self {
                self.value -= operand;
                self
            }

            /// Returns the Java `intValue` conversion.
            pub fn int_value(&self) -> i32 {
                self.value as i32
            }

            /// Returns the Java `longValue` conversion.
            pub fn long_value(&self) -> i64 {
                self.value as i64
            }

            /// Returns the Java `floatValue` conversion.
            pub fn float_value(&self) -> f32 {
                self.value as f32
            }

            /// Returns the Java `doubleValue` conversion.
            pub fn double_value(&self) -> f64 {
                self.value as f64
            }

            /// Returns canonical Java floating-point bits.
            pub fn canonical_bits(&self) -> $bits {
                $canonical(self.value)
            }

            /// Consumes the wrapper and returns its value.
            pub const fn into_inner(self) -> $primitive {
                self.value
            }
        }

        impl Mutable<$primitive> for $name {
            fn get(&self) -> &$primitive {
                &self.value
            }

            fn get_mut(&mut self) -> &mut $primitive {
                &mut self.value
            }

            fn set(&mut self, value: $primitive) {
                self.value = value;
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.canonical_bits() == other.canonical_bits()
            }
        }

        impl Eq for $name {}

        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $name {
            fn cmp(&self, other: &Self) -> Ordering {
                $compare(self.value, other.value)
            }
        }

        impl Hash for $name {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.canonical_bits().hash(state);
            }
        }

        impl From<$primitive> for $name {
            fn from(value: $primitive) -> Self {
                Self::new(value)
            }
        }

        impl FromStr for $name {
            type Err = <$primitive as FromStr>::Err;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                value.parse().map(Self::new)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.value.fmt(formatter)
            }
        }
    };
}

fn java_i64_hash(value: i64) -> i32 {
    let bits = u64::from_ne_bytes(value.to_ne_bytes());
    let folded = bits ^ (bits >> 32);
    let bytes = folded.to_le_bytes();
    i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}
