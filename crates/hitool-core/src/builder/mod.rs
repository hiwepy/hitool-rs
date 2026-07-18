//! Deterministic builders corresponding to Hutool's `core.builder` package.
//!
//! Rust traits and closures replace Java reflection for normal use. The
//! reflection-style helpers serialize values through Serde and then walk the
//! resulting structure, making the inspected data explicit and testable.

use serde::Serialize;
use serde_json::{Map, Number, Value};
use std::{cmp::Ordering, collections::HashSet, marker::PhantomData};
use thiserror::Error;

/// Errors produced by structural builder operations.
#[derive(Debug, Error)]
pub enum BuilderError {
    /// Serde could not expose the requested value structure.
    #[error("unable to inspect value: {0}")]
    Serialization(#[from] serde_json::Error),
    /// Hutool's hash algorithm requires non-zero odd constants.
    #[error("hash builder initial value and multiplier must both be odd")]
    EvenHashConstant,
}

/// Common contract implemented by stateful builders.
pub trait Builder<T> {
    /// Produces the current result.
    fn build(&mut self) -> T;
}

type Supplier<'a, T> = Box<dyn FnMut() -> T + 'a>;
type Modifier<'a, T> = Box<dyn Fn(&mut T) + 'a>;

/// A closure-backed object builder.
///
/// As in Hutool, modifiers are consumed by the first build while the supplier
/// remains reusable.
pub struct GenericBuilder<'a, T> {
    supplier: Supplier<'a, T>,
    modifiers: Vec<Modifier<'a, T>>,
}

impl<T> std::fmt::Debug for GenericBuilder<'_, T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("GenericBuilder")
            .field("pending_modifiers", &self.modifiers.len())
            .finish_non_exhaustive()
    }
}

impl<'a, T: 'a> GenericBuilder<'a, T> {
    /// Creates a builder from a zero-argument supplier.
    pub fn of(supplier: impl FnMut() -> T + 'a) -> Self {
        Self {
            supplier: Box::new(supplier),
            modifiers: Vec::new(),
        }
    }

    /// Creates a builder from a one-argument supplier.
    pub fn of1<P1>(mut supplier: impl FnMut(P1) -> T + 'a, p1: P1) -> Self
    where
        P1: Clone + 'a,
    {
        Self::of(move || supplier(p1.clone()))
    }

    /// Creates a builder from a two-argument supplier.
    pub fn of2<P1, P2>(mut supplier: impl FnMut(P1, P2) -> T + 'a, p1: P1, p2: P2) -> Self
    where
        P1: Clone + 'a,
        P2: Clone + 'a,
    {
        Self::of(move || supplier(p1.clone(), p2.clone()))
    }

    /// Creates a builder from a three-argument supplier.
    pub fn of3<P1, P2, P3>(
        mut supplier: impl FnMut(P1, P2, P3) -> T + 'a,
        p1: P1,
        p2: P2,
        p3: P3,
    ) -> Self
    where
        P1: Clone + 'a,
        P2: Clone + 'a,
        P3: Clone + 'a,
    {
        Self::of(move || supplier(p1.clone(), p2.clone(), p3.clone()))
    }

    /// Creates a builder from a four-argument supplier.
    pub fn of4<P1, P2, P3, P4>(
        mut supplier: impl FnMut(P1, P2, P3, P4) -> T + 'a,
        p1: P1,
        p2: P2,
        p3: P3,
        p4: P4,
    ) -> Self
    where
        P1: Clone + 'a,
        P2: Clone + 'a,
        P3: Clone + 'a,
        P4: Clone + 'a,
    {
        Self::of(move || supplier(p1.clone(), p2.clone(), p3.clone(), p4.clone()))
    }

    /// Creates a builder from a five-argument supplier.
    pub fn of5<P1, P2, P3, P4, P5>(
        mut supplier: impl FnMut(P1, P2, P3, P4, P5) -> T + 'a,
        p1: P1,
        p2: P2,
        p3: P3,
        p4: P4,
        p5: P5,
    ) -> Self
    where
        P1: Clone + 'a,
        P2: Clone + 'a,
        P3: Clone + 'a,
        P4: Clone + 'a,
        P5: Clone + 'a,
    {
        Self::of(move || supplier(p1.clone(), p2.clone(), p3.clone(), p4.clone(), p5.clone()))
    }

    /// Queues a modifier without additional arguments.
    #[must_use]
    pub fn with(mut self, modifier: impl Fn(&mut T) + 'a) -> Self {
        self.modifiers.push(Box::new(modifier));
        self
    }

    /// Queues a modifier with one owned argument.
    #[must_use]
    pub fn with1<P1: 'a>(mut self, modifier: impl Fn(&mut T, &P1) + 'a, p1: P1) -> Self {
        self.modifiers
            .push(Box::new(move |value| modifier(value, &p1)));
        self
    }

    /// Queues a modifier with two owned arguments.
    #[must_use]
    pub fn with2<P1: 'a, P2: 'a>(
        mut self,
        modifier: impl Fn(&mut T, &P1, &P2) + 'a,
        p1: P1,
        p2: P2,
    ) -> Self {
        self.modifiers
            .push(Box::new(move |value| modifier(value, &p1, &p2)));
        self
    }

    /// Returns the number of modifiers waiting for the next build.
    #[must_use]
    pub fn pending_modifiers(&self) -> usize {
        self.modifiers.len()
    }

    /// Produces a value and consumes the pending modifiers.
    pub fn build(&mut self) -> T {
        <Self as Builder<T>>::build(self)
    }
}

impl<T> Builder<T> for GenericBuilder<'_, T> {
    fn build(&mut self) -> T {
        let mut value = (self.supplier)();
        for modifier in std::mem::take(&mut self.modifiers) {
            modifier(&mut value);
        }
        value
    }
}

/// Incremental lexicographic comparison builder.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CompareToBuilder {
    comparison: i32,
}

impl CompareToBuilder {
    /// Creates an equal comparison state.
    #[must_use]
    pub const fn new() -> Self {
        Self { comparison: 0 }
    }

    fn append_ordering(&mut self, ordering: Ordering) -> &mut Self {
        if self.comparison == 0 {
            self.comparison = ordering_to_i32(ordering);
        }
        self
    }

    /// Appends a superclass comparison result.
    pub fn append_super(&mut self, comparison: i32) -> &mut Self {
        if self.comparison == 0 {
            self.comparison = comparison.signum();
        }
        self
    }

    /// Appends any ordered pair.
    pub fn append<T: Ord + ?Sized>(&mut self, lhs: &T, rhs: &T) -> &mut Self {
        self.append_ordering(lhs.cmp(rhs))
    }

    /// Appends optional values, ordering `None` before `Some`.
    pub fn append_option<T: Ord>(&mut self, lhs: Option<&T>, rhs: Option<&T>) -> &mut Self {
        self.append_ordering(lhs.cmp(&rhs))
    }

    /// Appends a pair using an explicit comparator.
    pub fn append_by<T: ?Sized>(
        &mut self,
        lhs: &T,
        rhs: &T,
        comparator: impl FnOnce(&T, &T) -> Ordering,
    ) -> &mut Self {
        if self.comparison == 0 {
            self.comparison = ordering_to_i32(comparator(lhs, rhs));
        }
        self
    }

    /// Appends slices using lexicographic ordering.
    pub fn append_slice<T: Ord>(&mut self, lhs: &[T], rhs: &[T]) -> &mut Self {
        self.append_ordering(lhs.cmp(rhs))
    }

    /// Appends `f32` values with Java `Float.compare` semantics.
    pub fn append_f32(&mut self, lhs: f32, rhs: f32) -> &mut Self {
        self.append_ordering(java_f32_cmp(lhs, rhs))
    }

    /// Appends `f64` values with Java `Double.compare` semantics.
    pub fn append_f64(&mut self, lhs: f64, rhs: f64) -> &mut Self {
        self.append_ordering(java_f64_cmp(lhs, rhs))
    }

    /// Returns the normalized comparison result.
    #[must_use]
    pub const fn to_comparison(self) -> i32 {
        self.comparison
    }

    /// Returns the current comparison through the shared builder contract.
    pub fn build(&mut self) -> i32 {
        <Self as Builder<i32>>::build(self)
    }

    /// Structurally compares two serializable Rust values.
    pub fn reflection_compare<T: Serialize>(lhs: &T, rhs: &T) -> Result<i32, BuilderError> {
        Self::reflection_compare_excluding(lhs, rhs, std::iter::empty::<&str>())
    }

    /// Structurally compares two values after excluding top-level field names.
    pub fn reflection_compare_excluding<'a, T: Serialize>(
        lhs: &T,
        rhs: &T,
        excluded: impl IntoIterator<Item = &'a str>,
    ) -> Result<i32, BuilderError> {
        let excluded = excluded.into_iter().collect::<HashSet<_>>();
        compare_serialized(
            serde_json::to_value(lhs),
            serde_json::to_value(rhs),
            &excluded,
        )
    }
}

impl Builder<i32> for CompareToBuilder {
    fn build(&mut self) -> i32 {
        self.comparison
    }
}

/// Incremental equality builder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EqualsBuilder {
    equal: bool,
}

impl Default for EqualsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl EqualsBuilder {
    /// Creates an equal state.
    #[must_use]
    pub const fn new() -> Self {
        Self { equal: true }
    }

    /// Appends a superclass equality result.
    pub fn append_super(&mut self, equal: bool) -> &mut Self {
        if self.equal {
            self.equal = equal;
        }
        self
    }

    /// Appends any pair supporting equality.
    pub fn append<T: PartialEq + ?Sized>(&mut self, lhs: &T, rhs: &T) -> &mut Self {
        if self.equal {
            self.equal = lhs == rhs;
        }
        self
    }

    /// Appends `f32` values using canonical Java bit equality.
    pub fn append_f32(&mut self, lhs: f32, rhs: f32) -> &mut Self {
        if self.equal {
            self.equal = java_f32_bits(lhs) == java_f32_bits(rhs);
        }
        self
    }

    /// Appends `f64` values using canonical Java bit equality.
    pub fn append_f64(&mut self, lhs: f64, rhs: f64) -> &mut Self {
        if self.equal {
            self.equal = java_f64_bits(lhs) == java_f64_bits(rhs);
        }
        self
    }

    /// Returns whether all appended pairs are equal.
    #[must_use]
    pub const fn is_equals(self) -> bool {
        self.equal
    }

    /// Resets the builder for reuse.
    pub fn reset(&mut self) {
        self.equal = true;
    }

    /// Returns the current equality through the shared builder contract.
    pub fn build(&mut self) -> bool {
        <Self as Builder<bool>>::build(self)
    }

    /// Structurally compares two serializable values.
    pub fn reflection_equals<T: Serialize>(lhs: &T, rhs: &T) -> Result<bool, BuilderError> {
        Self::reflection_equals_excluding(lhs, rhs, std::iter::empty::<&str>())
    }

    /// Structurally compares values after excluding top-level field names.
    pub fn reflection_equals_excluding<'a, T: Serialize>(
        lhs: &T,
        rhs: &T,
        excluded: impl IntoIterator<Item = &'a str>,
    ) -> Result<bool, BuilderError> {
        comparison_is_equal(CompareToBuilder::reflection_compare_excluding(
            lhs, rhs, excluded,
        ))
    }
}

impl Builder<bool> for EqualsBuilder {
    fn build(&mut self) -> bool {
        self.equal
    }
}

/// Hutool-compatible wrapping 32-bit hash builder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HashCodeBuilder {
    multiplier: i32,
    total: i32,
}

impl Default for HashCodeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl HashCodeBuilder {
    /// Creates the Hutool default `17 × 37` builder.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            multiplier: 37,
            total: 17,
        }
    }

    /// Creates a builder with caller-selected odd constants.
    pub fn with_constants(initial: i32, multiplier: i32) -> Result<Self, BuilderError> {
        if initial % 2 == 0 || multiplier % 2 == 0 {
            return Err(BuilderError::EvenHashConstant);
        }
        Ok(Self {
            multiplier,
            total: initial,
        })
    }

    fn append_i32_value(&mut self, value: i32) -> &mut Self {
        self.total = self.total.wrapping_mul(self.multiplier).wrapping_add(value);
        self
    }

    /// Appends a boolean using Hutool's true=0, false=1 rule.
    pub fn append_bool(&mut self, value: bool) -> &mut Self {
        self.append_i32_value(i32::from(!value))
    }

    /// Appends an 8-bit integer.
    pub fn append_i8(&mut self, value: i8) -> &mut Self {
        self.append_i32_value(i32::from(value))
    }

    /// Appends a 16-bit integer.
    pub fn append_i16(&mut self, value: i16) -> &mut Self {
        self.append_i32_value(i32::from(value))
    }

    /// Appends a Java-compatible UTF-16 character unit.
    pub fn append_char(&mut self, value: u16) -> &mut Self {
        self.append_i32_value(i32::from(value))
    }

    /// Appends a 32-bit integer.
    pub fn append_i32(&mut self, value: i32) -> &mut Self {
        self.append_i32_value(value)
    }

    /// Appends a 64-bit integer using Hutool's signed right-shift formula.
    pub fn append_i64(&mut self, value: i64) -> &mut Self {
        self.append_i32_value(low_i32_from_i64(value ^ (value >> 32)))
    }

    /// Appends an `f32` using canonical Java bits.
    pub fn append_f32(&mut self, value: f32) -> &mut Self {
        self.append_i32_value(u32_as_i32(java_f32_bits(value)))
    }

    /// Appends an `f64` using canonical Java bits.
    pub fn append_f64(&mut self, value: f64) -> &mut Self {
        self.append_i64(u64_as_i64(java_f64_bits(value)))
    }

    /// Appends a caller-provided object hash code.
    pub fn append_hash_code(&mut self, value: i32) -> &mut Self {
        self.append_i32_value(value)
    }

    /// Appends a UTF-8 string using Java's UTF-16 `String.hashCode` algorithm.
    pub fn append_str(&mut self, value: &str) -> &mut Self {
        self.append_hash_code(java_string_hash(value))
    }

    /// Appends all slice elements with a typed projection.
    pub fn append_slice_by<T>(
        &mut self,
        values: Option<&[T]>,
        mut append: impl FnMut(&mut Self, &T),
    ) -> &mut Self {
        if let Some(values) = values {
            for value in values {
                append(self, value);
            }
        } else {
            self.append_i32_value(0);
        }
        self
    }

    /// Appends the superclass hash code.
    pub fn append_super(&mut self, value: i32) -> &mut Self {
        self.append_i32_value(value)
    }

    /// Returns the current wrapping 32-bit hash code.
    #[must_use]
    pub const fn to_hash_code(self) -> i32 {
        self.total
    }

    /// Returns the current hash through the shared builder contract.
    pub fn build(&mut self) -> i32 {
        <Self as Builder<i32>>::build(self)
    }

    /// Hashes a serializable structure using the same primitive append rules.
    pub fn reflection_hash_code<T: Serialize>(value: &T) -> Result<i32, BuilderError> {
        Self::reflection_hash_code_excluding(value, std::iter::empty::<&str>())
    }

    /// Hashes a structure after excluding top-level field names.
    pub fn reflection_hash_code_excluding<'a, T: Serialize>(
        value: &T,
        excluded: impl IntoIterator<Item = &'a str>,
    ) -> Result<i32, BuilderError> {
        let excluded = excluded.into_iter().collect::<HashSet<_>>();
        hash_serialized(serde_json::to_value(value), &excluded)
    }

    fn append_json(&mut self, value: &Value) {
        match value {
            Value::Null => {
                self.append_i32_value(0);
            }
            Value::Bool(value) => {
                self.append_bool(*value);
            }
            Value::Number(value) => self.append_number(value),
            Value::String(value) => {
                self.append_str(value);
            }
            Value::Array(values) => {
                for value in values {
                    self.append_json(value);
                }
            }
            Value::Object(values) => {
                for value in values.values() {
                    self.append_json(value);
                }
            }
        }
    }

    fn append_number(&mut self, value: &Number) {
        if let Some(value) = value.as_i64() {
            self.append_i64(value);
            return;
        }
        if let Some(value) = value.as_u64() {
            self.append_i64(u64_as_i64(value));
            return;
        }
        self.append_f64(value.as_f64().expect("JSON numbers are finite"));
    }
}

impl Builder<i32> for HashCodeBuilder {
    fn build(&mut self) -> i32 {
        self.total
    }
}

/// A lifetime-bound key that compares references by identity rather than value.
#[derive(Debug)]
pub struct IdKey<'a, T> {
    address: usize,
    marker: PhantomData<&'a T>,
}

impl<'a, T> IdKey<'a, T> {
    /// Captures the identity of a live reference.
    #[must_use]
    pub fn new(value: &'a T) -> Self {
        Self {
            address: std::ptr::from_ref(value) as usize,
            marker: PhantomData,
        }
    }

    /// Returns a folded address hash analogous to `System.identityHashCode`.
    #[must_use]
    pub fn hash_code(&self) -> i32 {
        let folded = self.address ^ self.address.rotate_right(usize::BITS / 2);
        let bytes = folded.to_le_bytes();
        i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
}

impl<T> Copy for IdKey<'_, T> {}

impl<T> Clone for IdKey<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> PartialEq for IdKey<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}

impl<T> Eq for IdKey<'_, T> {}

impl<T> std::hash::Hash for IdKey<'_, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&self.address, state);
    }
}

fn ordering_to_i32(ordering: Ordering) -> i32 {
    match ordering {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

fn java_f32_bits(value: f32) -> u32 {
    if value.is_nan() {
        0x7fc0_0000
    } else {
        value.to_bits()
    }
}

fn java_f64_bits(value: f64) -> u64 {
    if value.is_nan() {
        0x7ff8_0000_0000_0000
    } else {
        value.to_bits()
    }
}

fn java_f32_cmp(lhs: f32, rhs: f32) -> Ordering {
    lhs.partial_cmp(&rhs)
        .unwrap_or_else(|| u32_as_i32(java_f32_bits(lhs)).cmp(&u32_as_i32(java_f32_bits(rhs))))
        .then_with(|| u32_as_i32(java_f32_bits(lhs)).cmp(&u32_as_i32(java_f32_bits(rhs))))
}

fn java_f64_cmp(lhs: f64, rhs: f64) -> Ordering {
    lhs.partial_cmp(&rhs)
        .unwrap_or_else(|| u64_as_i64(java_f64_bits(lhs)).cmp(&u64_as_i64(java_f64_bits(rhs))))
        .then_with(|| u64_as_i64(java_f64_bits(lhs)).cmp(&u64_as_i64(java_f64_bits(rhs))))
}

fn u32_as_i32(value: u32) -> i32 {
    i32::from_ne_bytes(value.to_ne_bytes())
}

fn u64_as_i64(value: u64) -> i64 {
    i64::from_ne_bytes(value.to_ne_bytes())
}

fn low_i32_from_i64(value: i64) -> i32 {
    let bytes = value.to_le_bytes();
    i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

fn java_string_hash(value: &str) -> i32 {
    value.encode_utf16().fold(0_i32, |hash, unit| {
        hash.wrapping_mul(31).wrapping_add(i32::from(unit))
    })
}

fn remove_fields(value: &mut Value, excluded: &HashSet<&str>) {
    if let Value::Object(fields) = value {
        fields.retain(|name, _| !excluded.contains(name.as_str()));
    }
}

fn compare_serialized(
    lhs: Result<Value, serde_json::Error>,
    rhs: Result<Value, serde_json::Error>,
    excluded: &HashSet<&str>,
) -> Result<i32, BuilderError> {
    let mut lhs = lhs?;
    let mut rhs = rhs?;
    remove_fields(&mut lhs, excluded);
    remove_fields(&mut rhs, excluded);
    Ok(ordering_to_i32(compare_values(&lhs, &rhs)))
}

fn comparison_is_equal(comparison: Result<i32, BuilderError>) -> Result<bool, BuilderError> {
    comparison.map(|comparison| comparison == 0)
}

fn hash_serialized(
    value: Result<Value, serde_json::Error>,
    excluded: &HashSet<&str>,
) -> Result<i32, BuilderError> {
    let mut value = value?;
    remove_fields(&mut value, excluded);
    let mut builder = HashCodeBuilder::new();
    builder.append_json(&value);
    Ok(builder.to_hash_code())
}

fn compare_values(lhs: &Value, rhs: &Value) -> Ordering {
    let rank_ordering = value_rank(lhs).cmp(&value_rank(rhs));
    if rank_ordering != Ordering::Equal {
        return rank_ordering;
    }
    match lhs {
        Value::Null => Ordering::Equal,
        Value::Bool(lhs) => lhs.cmp(&rhs.as_bool().expect("equal rank is boolean")),
        Value::Number(lhs) => compare_numbers(lhs, rhs.as_number().expect("equal rank is number")),
        Value::String(lhs) => lhs
            .as_str()
            .cmp(rhs.as_str().expect("equal rank is string")),
        Value::Array(lhs) => compare_slices(lhs, rhs.as_array().expect("equal rank is array")),
        Value::Object(lhs) => compare_maps(lhs, rhs.as_object().expect("equal rank is object")),
    }
}

fn value_rank(value: &Value) -> u8 {
    match value {
        Value::Null => 0,
        Value::Bool(_) => 1,
        Value::Number(_) => 2,
        Value::String(_) => 3,
        Value::Array(_) => 4,
        Value::Object(_) => 5,
    }
}

fn compare_numbers(lhs: &Number, rhs: &Number) -> Ordering {
    match (lhs.as_i64(), rhs.as_i64(), lhs.as_u64(), rhs.as_u64()) {
        (Some(lhs), Some(rhs), _, _) => lhs.cmp(&rhs),
        (_, _, Some(lhs), Some(rhs)) => lhs.cmp(&rhs),
        (Some(_), _, _, Some(_)) => Ordering::Less,
        (_, Some(_), Some(_), _) => Ordering::Greater,
        _ => lhs
            .as_f64()
            .expect("JSON numbers are finite")
            .total_cmp(&rhs.as_f64().expect("JSON numbers are finite")),
    }
}

fn compare_slices(lhs: &[Value], rhs: &[Value]) -> Ordering {
    for (lhs, rhs) in lhs.iter().zip(rhs) {
        let ordering = compare_values(lhs, rhs);
        if ordering != Ordering::Equal {
            return ordering;
        }
    }
    lhs.len().cmp(&rhs.len())
}

fn compare_maps(lhs: &Map<String, Value>, rhs: &Map<String, Value>) -> Ordering {
    for ((lhs_key, lhs_value), (rhs_key, rhs_value)) in lhs.iter().zip(rhs) {
        let ordering = lhs_key
            .cmp(rhs_key)
            .then_with(|| compare_values(lhs_value, rhs_value));
        if ordering != Ordering::Equal {
            return ordering;
        }
    }
    lhs.len().cmp(&rhs.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::ser::Error as _;
    use std::{collections::hash_map::DefaultHasher, hash::Hasher};

    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    struct Record {
        id: i64,
        name: String,
        ignored: u32,
    }

    #[test]
    fn generic_builder_supports_every_supplier_and_modifier_shape() {
        let mut zero = GenericBuilder::of(Vec::<i32>::new)
            .with(|values| values.push(1))
            .with1(|values, value| values.push(*value), 2)
            .with2(
                |values, first, second| values.extend([*first, *second]),
                3,
                4,
            );
        assert!(format!("{zero:?}").contains("pending_modifiers: 3"));
        assert_eq!(zero.pending_modifiers(), 3);
        assert_eq!(zero.build(), vec![1, 2, 3, 4]);
        assert_eq!(zero.pending_modifiers(), 0);
        assert!(zero.build().is_empty());

        assert_eq!(GenericBuilder::of1(|a| a, 1).build(), 1);
        assert_eq!(
            GenericBuilder::of2(|a, b| a + b, 1, 2)
                .with(|value| *value += 1)
                .build(),
            4
        );
        assert_eq!(GenericBuilder::of3(|a, b, c| a + b + c, 1, 2, 3).build(), 6);
        assert_eq!(
            GenericBuilder::of4(|a, b, c, d| a + b + c + d, 1, 2, 3, 4).build(),
            10
        );
        assert_eq!(
            GenericBuilder::of5(|a, b, c, d, e| a + b + c + d + e, 1, 2, 3, 4, 5).build(),
            15
        );
    }

    #[test]
    fn compare_builder_short_circuits_options_slices_custom_and_float_edges() {
        let mut builder = CompareToBuilder::new();
        assert_eq!(
            builder
                .append(&1, &1)
                .append_super(9)
                .append(&0, &99)
                .build(),
            1
        );
        assert_eq!(builder.to_comparison(), 1);
        assert_eq!(
            CompareToBuilder::default()
                .append_option(None, Some(&1))
                .build(),
            -1
        );
        assert_eq!(
            CompareToBuilder::new()
                .append_slice(&[1, 2], &[1, 3])
                .build(),
            -1
        );
        assert_eq!(
            CompareToBuilder::new()
                .append_by("a", "bbb", |lhs, rhs| lhs.len().cmp(&rhs.len()))
                .build(),
            -1
        );
        let mut skipped = CompareToBuilder::new();
        assert_eq!(
            CompareToBuilder::new().append_by(&1, &2, Ord::cmp).build(),
            -1
        );
        skipped
            .append(&2, &1)
            .append_by(&1, &2, Ord::cmp)
            .append_super(9);
        assert_eq!(skipped.build(), 1);
        assert_eq!(CompareToBuilder::new().append_super(-9).build(), -1);
        assert_eq!(CompareToBuilder::new().append_super(0).build(), 0);
        assert_eq!(
            CompareToBuilder::new()
                .append_f32(f32::NAN, f32::NAN)
                .build(),
            0
        );
        assert_eq!(CompareToBuilder::new().append_f32(-0.0, 0.0).build(), -1);
        assert_eq!(CompareToBuilder::new().append_f64(f64::NAN, 2.0).build(), 1);
        assert_eq!(CompareToBuilder::new().append_f64(-0.0, 0.0).build(), -1);
    }

    #[test]
    fn structural_compare_equals_and_exclusions_are_real_serde_walks() {
        let lhs = Record {
            id: 1,
            name: "a".into(),
            ignored: 1,
        };
        let rhs = Record {
            id: 1,
            name: "b".into(),
            ignored: 2,
        };
        assert_eq!(
            CompareToBuilder::reflection_compare(&lhs, &rhs).unwrap(),
            -1
        );
        assert!(EqualsBuilder::reflection_equals_excluding(&lhs, &lhs, ["ignored"]).unwrap());
        assert!(!EqualsBuilder::reflection_equals(&lhs, &rhs).unwrap());
        assert_eq!(
            CompareToBuilder::reflection_compare_excluding(&lhs, &rhs, ["name", "ignored"])
                .unwrap(),
            0
        );

        let values = serde_json::json!([null, false, 1, "a", [1], {"a": 1}]);
        for pair in values.as_array().unwrap().windows(2) {
            assert_eq!(compare_values(&pair[0], &pair[1]), Ordering::Less);
        }
        assert_eq!(
            compare_values(&serde_json::json!([1]), &serde_json::json!([1, 2])),
            Ordering::Less
        );
        assert_eq!(
            compare_values(&serde_json::json!([2]), &serde_json::json!([1])),
            Ordering::Greater
        );
        assert_eq!(compare_values(&Value::Null, &Value::Null), Ordering::Equal);
        assert_eq!(
            compare_values(&Value::Bool(false), &Value::Bool(true)),
            Ordering::Less
        );
        assert_eq!(
            compare_values(&serde_json::json!({"a": 1}), &serde_json::json!({"b": 1})),
            Ordering::Less
        );
        assert_eq!(
            compare_values(&serde_json::json!({"a": 1}), &serde_json::json!({"a": 2})),
            Ordering::Less
        );
        assert_eq!(
            compare_numbers(&Number::from(-1), &Number::from(u64::MAX)),
            Ordering::Less
        );
        assert_eq!(
            compare_numbers(&Number::from(u64::MAX), &Number::from(-1)),
            Ordering::Greater
        );
        assert_eq!(
            compare_numbers(&Number::from(5_u64), &Number::from(5_i64)),
            Ordering::Equal
        );
        assert_eq!(
            compare_numbers(&Number::from(u64::MAX - 1), &Number::from(u64::MAX)),
            Ordering::Less
        );
        assert_eq!(
            compare_numbers(&Number::from(i64::MAX), &Number::from(u64::MAX)),
            Ordering::Less
        );
        assert_eq!(
            compare_numbers(&Number::from(u64::MAX), &Number::from(i64::MAX)),
            Ordering::Greater
        );
        assert_eq!(
            compare_numbers(
                &Number::from_f64(1.5).unwrap(),
                &Number::from_f64(2.5).unwrap()
            ),
            Ordering::Less
        );
    }

    #[test]
    fn equals_builder_matches_java_float_bits_short_circuit_and_reset() {
        let mut builder = EqualsBuilder::new();
        builder
            .append(&1, &1)
            .append_f32(f32::NAN, f32::from_bits(0x7fc0_0001));
        assert!(builder.is_equals());
        builder
            .append_f64(-0.0, 0.0)
            .append_super(true)
            .append(&1, &1)
            .append_f32(1.0, 1.0)
            .append_f64(1.0, 1.0);
        assert!(!builder.build());
        builder.reset();
        assert!(
            builder
                .append_f64(f64::NAN, f64::NAN)
                .append(&[1, 2], &[1, 2])
                .build()
        );
        assert!(!EqualsBuilder::default().append_super(false).build());
    }

    #[test]
    fn hash_builder_matches_hutool_primitive_array_and_structural_rules() {
        fn append_record(builder: &mut HashCodeBuilder, value: &Record) {
            builder.append_i64(value.id);
        }

        assert!(HashCodeBuilder::with_constants(2, 37).is_err());
        assert!(HashCodeBuilder::with_constants(17, 2).is_err());
        let mut hash = HashCodeBuilder::with_constants(17, 37).unwrap();
        hash.append_bool(true)
            .append_bool(false)
            .append_i8(-1)
            .append_i16(-2)
            .append_char('好' as u16)
            .append_i32(3)
            .append_i64(-4)
            .append_f32(f32::NAN)
            .append_f64(f64::NAN)
            .append_hash_code(5)
            .append_str("A😀")
            .append_super(6);
        let before_slice = hash.to_hash_code();
        let records = [
            Record {
                id: 1,
                name: String::new(),
                ignored: 0,
            },
            Record {
                id: 2,
                name: String::new(),
                ignored: 0,
            },
        ];
        hash.append_slice_by(Some(&records), append_record);
        assert_ne!(hash.to_hash_code(), before_slice);
        hash.append_slice_by::<Record>(None, append_record);
        assert_eq!(hash.build(), hash.to_hash_code());

        let record = Record {
            id: 9,
            name: "x".into(),
            ignored: 3,
        };
        let full = HashCodeBuilder::reflection_hash_code(&record).unwrap();
        let selected =
            HashCodeBuilder::reflection_hash_code_excluding(&record, ["ignored"]).unwrap();
        assert_ne!(full, selected);
        assert_eq!(java_string_hash("😀"), 1_772_899);

        let all_shapes = serde_json::json!({"a": null, "b": false, "c": 1.5, "d": ["x"]});
        assert_ne!(
            HashCodeBuilder::reflection_hash_code(&all_shapes).unwrap(),
            17
        );
        assert_eq!(HashCodeBuilder::new().to_hash_code(), 17);
        assert_ne!(
            HashCodeBuilder::reflection_hash_code(&serde_json::json!(u64::MAX)).unwrap(),
            17
        );
        assert_eq!(HashCodeBuilder::default(), HashCodeBuilder::new());
    }

    #[test]
    fn identity_key_uses_reference_identity_and_standard_hashing() {
        let first = String::from("same");
        let second = first.clone();
        let one = IdKey::new(&first);
        let same = IdKey::new(&first);
        let other = IdKey::new(&second);
        assert_eq!(one, same);
        assert_eq!(one.clone(), same);
        assert_ne!(one, other);
        assert_eq!(one.hash_code(), same.hash_code());
        let mut lhs = DefaultHasher::new();
        let mut rhs = DefaultHasher::new();
        std::hash::Hash::hash(&one, &mut lhs);
        std::hash::Hash::hash(&same, &mut rhs);
        assert_eq!(lhs.finish(), rhs.finish());
    }

    struct FailingSerialize;

    struct FailSecond(std::cell::Cell<u8>);

    impl Serialize for FailSecond {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let count = self.0.get();
            self.0.set(count + 1);
            if count == 0 {
                serializer.serialize_u8(1)
            } else {
                Err(S::Error::custom("second"))
            }
        }
    }

    impl Serialize for FailingSerialize {
        fn serialize<S: serde::Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
            Err(S::Error::custom("injected"))
        }
    }

    #[test]
    fn structural_helpers_propagate_serialization_errors() {
        assert!(
            CompareToBuilder::reflection_compare(&FailingSerialize, &FailingSerialize)
                .unwrap_err()
                .to_string()
                .contains("injected")
        );
        assert!(EqualsBuilder::reflection_equals(&FailingSerialize, &FailingSerialize).is_err());
        assert!(HashCodeBuilder::reflection_hash_code(&FailingSerialize).is_err());
        let fail_second = FailSecond(std::cell::Cell::new(0));
        assert!(CompareToBuilder::reflection_compare(&fail_second, &fail_second).is_err());
        assert_eq!(
            CompareToBuilder::reflection_compare_excluding(&1, &1, ["unused"]).unwrap(),
            0
        );
    }
}
