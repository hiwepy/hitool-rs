//! Mutable value containers corresponding to Hutool's `core.lang.mutable` package.
//!
//! Rust normally prefers ordinary mutable bindings. These small wrappers are
//! useful when mutation itself must be passed around as a value, while keeping
//! ownership and borrowing explicit.

#![allow(

use super::mutable::Mutable;

/// Mutable key/value pair.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MutablePair<K, V> {
    pair: (K, V),
}

impl<K, V> MutablePair<K, V> {
    /// Creates a mutable pair.
    pub const fn new(key: K, value: V) -> Self {
        Self { pair: (key, value) }
    }

    /// Borrows the key.
    pub const fn key(&self) -> &K {
        &self.pair.0
    }

    /// Borrows the value.
    pub const fn value(&self) -> &V {
        &self.pair.1
    }

    /// Replaces the key and returns this pair.
    pub fn set_key(&mut self, key: K) -> &mut Self {
        self.pair.0 = key;
        self
    }

    /// Replaces the value and returns this pair.
    pub fn set_value(&mut self, value: V) -> &mut Self {
        self.pair.1 = value;
        self
    }

    /// Borrows the complete pair.
    pub const fn get(&self) -> &(K, V) {
        &self.pair
    }

    /// Mutably borrows the complete pair.
    pub fn get_mut(&mut self) -> &mut (K, V) {
        &mut self.pair
    }

    /// Replaces both pair elements.
    pub fn set(&mut self, pair: (K, V)) {
        self.pair = pair;
    }

    /// Consumes the wrapper and returns the pair.
    pub fn into_inner(self) -> (K, V) {
        self.pair
    }
}

impl<K, V> Mutable<(K, V)> for MutablePair<K, V> {
    fn get(&self) -> &(K, V) {
        self.get()
    }

    fn get_mut(&mut self) -> &mut (K, V) {
        self.get_mut()
    }

    fn set(&mut self, pair: (K, V)) {
        self.set(pair);
    }
}

impl<K, V> From<(K, V)> for MutablePair<K, V> {
    fn from((key, value): (K, V)) -> Self {
        Self::new(key, value)
    }
}

    macro_rules! assert_integer_mutable {
        ($type:ident, $primitive:ty, $min:expr, $max:expr, $hash:expr) => {{
            let mut value = $type::default();
            assert_eq!(value.get(), 0);
            value.set(2);
            assert_eq!(value.increment().decrement().add(3).subtract(1).get(), 4);
            assert_eq!(value.byte_value(), 4_i8);
            assert_eq!(value.short_value(), 4_i16);
            assert_eq!(value.int_value(), 4_i32);
            assert_eq!(value.long_value(), 4_i64);
            assert_eq!(value.float_value(), 4_f32);
            assert_eq!(value.double_value(), 4_f64);
            assert_eq!(value.java_hash_code(), $hash);
            assert_eq!(value.to_string(), "4");
            assert_eq!($type::from(4 as $primitive), value);
            assert_eq!("4".parse::<$type>().unwrap(), value);
            assert!("bad".parse::<$type>().is_err());
            assert_eq!(value.into_inner(), 4);

            let mut wrapping = $type::new($max);
            assert_eq!(wrapping.increment().get(), $min);
            assert_eq!(wrapping.decrement().get(), $max);
            assert_eq!(wrapping.add(1).get(), $min);
            assert_eq!(wrapping.subtract(1).get(), $max);
            trait_round_trip(&mut wrapping, &(7 as $primitive));
            assert_eq!(*Mutable::get(&wrapping), 7);
        }};
    }

    macro_rules! assert_float_mutable {
        ($type:ident, $primitive:ty) => {{
            let mut value = $type::default();
            value.set(2.0);
            assert_eq!(
                value.increment().decrement().add(3.0).subtract(1.0).get(),
                4.0
            );
            assert_eq!(value.int_value(), 4);
            assert_eq!(value.long_value(), 4);
            assert_eq!(value.float_value(), 4.0);
            assert_eq!(value.double_value(), 4.0);
            assert_eq!(value.to_string(), "4");
            assert_eq!($type::from(4.0), value);
            assert_eq!("4".parse::<$type>().unwrap(), value);
            assert!("bad".parse::<$type>().is_err());
            assert_eq!(value.into_inner(), 4.0);
            trait_round_trip(&mut value, &(7.0 as $primitive));
            assert_eq!(*Mutable::get(&value), 7.0);

            let nan = $type::new(<$primitive>::NAN);
            let other_nan = $type::new(-<$primitive>::NAN);
            assert_eq!(nan, other_nan);
            assert_eq!(nan.cmp(&other_nan), Ordering::Equal);
            assert_eq!(nan.partial_cmp(&other_nan), Some(Ordering::Equal));
            assert_ne!($type::new(-0.0), $type::new(0.0));
            assert_eq!($type::new(-0.0).cmp(&$type::new(0.0)), Ordering::Less);
            assert_eq!($type::new(1.0).cmp(&$type::new(2.0)), Ordering::Less);
            assert_eq!(nan.cmp(&$type::new(2.0)), Ordering::Greater);

            let mut lhs = DefaultHasher::new();
            let mut rhs = DefaultHasher::new();
            nan.hash(&mut lhs);
            other_nan.hash(&mut rhs);
            assert_eq!(lhs.finish(), rhs.finish());
            assert_ne!(nan.java_hash_code(), 0);
        }};
    }
