//! Endian-aware numeric byte conversion aligned with Hutool's `ByteUtil` family.

use std::{
    str::FromStr,
    sync::atomic::{AtomicI32, AtomicI64, Ordering},
};

use num_bigint::BigInt;
use parking_lot::Mutex;
use rust_decimal::Decimal;

/// Byte order used by numeric conversions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ByteOrder {
    /// Least-significant byte first.
    LittleEndian,
    /// Most-significant byte first.
    BigEndian,
}

/// Errors produced by checked byte conversions.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum ByteUtilError {
    /// The requested fixed-width number did not fit in the remaining input.
    #[error("insufficient bytes at offset {start}: required {required}, available {available}")]
    InsufficientBytes {
        /// Requested starting offset.
        start: usize,
        /// Required number of bytes.
        required: usize,
        /// Bytes available after `start`.
        available: usize,
    },

    /// An IEEE-754 value could not be represented as a decimal number.
    #[error("floating-point value cannot be converted to Decimal: {0}")]
    Decimal(String),
}

/// Concurrent integer adder equivalent to Java's `LongAdder` result branch.
#[derive(Debug, Default)]
pub struct LongAdder {
    value: AtomicI64,
}

impl LongAdder {
    /// Creates an adder with `value` as its initial sum.
    #[must_use]
    pub const fn new(value: i64) -> Self {
        Self {
            value: AtomicI64::new(value),
        }
    }

    /// Atomically adds `value`.
    pub fn add(&self, value: i64) {
        self.value.fetch_add(value, Ordering::Relaxed);
    }

    /// Returns the current sum.
    #[must_use]
    pub fn sum(&self) -> i64 {
        self.value.load(Ordering::Relaxed)
    }
}

/// Concurrent floating-point adder equivalent to Java's `DoubleAdder` result branch.
#[derive(Debug, Default)]
pub struct DoubleAdder {
    value: Mutex<f64>,
}

impl DoubleAdder {
    /// Creates an adder with `value` as its initial sum.
    #[must_use]
    pub const fn new(value: f64) -> Self {
        Self {
            value: Mutex::new(value),
        }
    }

    /// Adds `value` under a non-poisoning lock.
    pub fn add(&self, value: f64) {
        *self.value.lock() += value;
    }

    /// Returns the current sum.
    #[must_use]
    pub fn sum(&self) -> f64 {
        *self.value.lock()
    }
}

/// Rust-native input contract for Hutool's `numberToBytes` overloads.
pub trait NumberToBytes {
    /// Serializes this numeric value in `order`.
    fn number_to_bytes(self, order: ByteOrder) -> Vec<u8>;
}

/// Rust-native target contract replacing Hutool's runtime `Class<T>` argument.
pub trait BytesToNumber: Sized {
    /// Reads this numeric type from `bytes` in `order`.
    fn bytes_to_number(bytes: &[u8], order: ByteOrder) -> Result<Self, ByteUtilError>;
}

/// Zero-sized facade for endian-aware numeric byte conversion.
#[derive(Debug, Clone, Copy, Default)]
pub struct ByteUtil;

impl ByteUtil {
    /// Hutool's default conversion order.
    pub const DEFAULT_ORDER: ByteOrder = ByteOrder::LittleEndian;

    /// Native CPU byte order determined at compile time.
    #[cfg(target_endian = "little")]
    pub const CPU_ENDIAN: ByteOrder = ByteOrder::LittleEndian;

    /// Native CPU byte order determined at compile time.
    #[cfg(target_endian = "big")]
    pub const CPU_ENDIAN: ByteOrder = ByteOrder::BigEndian;

    /// Narrows an `i32` to the low eight bits with Java cast semantics.
    #[must_use]
    pub const fn int_to_byte(value: i32) -> i8 {
        i8::from_ne_bytes([value.to_le_bytes()[0]])
    }

    /// Reinterprets a signed byte as the unsigned integer range `0..=255`.
    #[must_use]
    pub fn byte_to_unsigned_int(value: i8) -> u32 {
        u32::from(value.to_ne_bytes()[0])
    }

    /// Reads a little-endian `i16` from offset zero.
    pub fn bytes_to_i16(bytes: &[u8]) -> Result<i16, ByteUtilError> {
        Self::bytes_to_i16_with_order(bytes, Self::DEFAULT_ORDER)
    }

    /// Reads an `i16` from offset zero in `order`.
    pub fn bytes_to_i16_with_order(bytes: &[u8], order: ByteOrder) -> Result<i16, ByteUtilError> {
        Self::bytes_to_i16_at(bytes, 0, order)
    }

    /// Reads an `i16` from `start` in `order`.
    pub fn bytes_to_i16_at(
        bytes: &[u8],
        start: usize,
        order: ByteOrder,
    ) -> Result<i16, ByteUtilError> {
        let bytes = read_array::<2>(bytes, start)?;
        Ok(match order {
            ByteOrder::LittleEndian => i16::from_le_bytes(bytes),
            ByteOrder::BigEndian => i16::from_be_bytes(bytes),
        })
    }

    /// Writes an `i16` in Hutool's default little-endian order.
    #[must_use]
    pub const fn i16_to_bytes(value: i16) -> [u8; 2] {
        Self::i16_to_bytes_with_order(value, Self::DEFAULT_ORDER)
    }

    /// Writes an `i16` in `order`.
    #[must_use]
    pub const fn i16_to_bytes_with_order(value: i16, order: ByteOrder) -> [u8; 2] {
        match order {
            ByteOrder::LittleEndian => value.to_le_bytes(),
            ByteOrder::BigEndian => value.to_be_bytes(),
        }
    }

    /// Reads a little-endian `i32` from offset zero.
    pub fn bytes_to_i32(bytes: &[u8]) -> Result<i32, ByteUtilError> {
        Self::bytes_to_i32_with_order(bytes, Self::DEFAULT_ORDER)
    }

    /// Reads an `i32` from offset zero in `order`.
    pub fn bytes_to_i32_with_order(bytes: &[u8], order: ByteOrder) -> Result<i32, ByteUtilError> {
        Self::bytes_to_i32_at(bytes, 0, order)
    }

    /// Reads an `i32` from `start` in `order`.
    pub fn bytes_to_i32_at(
        bytes: &[u8],
        start: usize,
        order: ByteOrder,
    ) -> Result<i32, ByteUtilError> {
        let bytes = read_array::<4>(bytes, start)?;
        Ok(match order {
            ByteOrder::LittleEndian => i32::from_le_bytes(bytes),
            ByteOrder::BigEndian => i32::from_be_bytes(bytes),
        })
    }

    /// Writes an `i32` in Hutool's default little-endian order.
    #[must_use]
    pub const fn i32_to_bytes(value: i32) -> [u8; 4] {
        Self::i32_to_bytes_with_order(value, Self::DEFAULT_ORDER)
    }

    /// Writes an `i32` in `order`.
    #[must_use]
    pub const fn i32_to_bytes_with_order(value: i32, order: ByteOrder) -> [u8; 4] {
        match order {
            ByteOrder::LittleEndian => value.to_le_bytes(),
            ByteOrder::BigEndian => value.to_be_bytes(),
        }
    }

    /// Reads a little-endian `i64` from offset zero.
    pub fn bytes_to_i64(bytes: &[u8]) -> Result<i64, ByteUtilError> {
        Self::bytes_to_i64_with_order(bytes, Self::DEFAULT_ORDER)
    }

    /// Reads an `i64` from offset zero in `order`.
    pub fn bytes_to_i64_with_order(bytes: &[u8], order: ByteOrder) -> Result<i64, ByteUtilError> {
        Self::bytes_to_i64_at(bytes, 0, order)
    }

    /// Reads an `i64` from `start` in `order`.
    pub fn bytes_to_i64_at(
        bytes: &[u8],
        start: usize,
        order: ByteOrder,
    ) -> Result<i64, ByteUtilError> {
        let bytes = read_array::<8>(bytes, start)?;
        Ok(match order {
            ByteOrder::LittleEndian => i64::from_le_bytes(bytes),
            ByteOrder::BigEndian => i64::from_be_bytes(bytes),
        })
    }

    /// Writes an `i64` in Hutool's default little-endian order.
    #[must_use]
    pub const fn i64_to_bytes(value: i64) -> [u8; 8] {
        Self::i64_to_bytes_with_order(value, Self::DEFAULT_ORDER)
    }

    /// Writes an `i64` in `order`.
    #[must_use]
    pub const fn i64_to_bytes_with_order(value: i64, order: ByteOrder) -> [u8; 8] {
        match order {
            ByteOrder::LittleEndian => value.to_le_bytes(),
            ByteOrder::BigEndian => value.to_be_bytes(),
        }
    }

    /// Writes an `f32` in Hutool's default little-endian order.
    #[must_use]
    pub fn f32_to_bytes(value: f32) -> [u8; 4] {
        Self::f32_to_bytes_with_order(value, Self::DEFAULT_ORDER)
    }

    /// Writes Java-canonicalized `f32` bits in `order`.
    #[must_use]
    pub fn f32_to_bytes_with_order(value: f32, order: ByteOrder) -> [u8; 4] {
        let bits = java_f32_bits(value);
        match order {
            ByteOrder::LittleEndian => bits.to_le_bytes(),
            ByteOrder::BigEndian => bits.to_be_bytes(),
        }
    }

    /// Reads a little-endian `f32` from offset zero.
    pub fn bytes_to_f32(bytes: &[u8]) -> Result<f32, ByteUtilError> {
        Self::bytes_to_f32_with_order(bytes, Self::DEFAULT_ORDER)
    }

    /// Reads an `f32` from offset zero in `order`.
    pub fn bytes_to_f32_with_order(bytes: &[u8], order: ByteOrder) -> Result<f32, ByteUtilError> {
        let bits = read_array::<4>(bytes, 0)?;
        Ok(f32::from_bits(match order {
            ByteOrder::LittleEndian => u32::from_le_bytes(bits),
            ByteOrder::BigEndian => u32::from_be_bytes(bits),
        }))
    }

    /// Writes an `f64` in Hutool's default little-endian order.
    #[must_use]
    pub fn f64_to_bytes(value: f64) -> [u8; 8] {
        Self::f64_to_bytes_with_order(value, Self::DEFAULT_ORDER)
    }

    /// Writes Java-canonicalized `f64` bits in `order`.
    #[must_use]
    pub fn f64_to_bytes_with_order(value: f64, order: ByteOrder) -> [u8; 8] {
        let bits = java_f64_bits(value);
        match order {
            ByteOrder::LittleEndian => bits.to_le_bytes(),
            ByteOrder::BigEndian => bits.to_be_bytes(),
        }
    }

    /// Reads a little-endian `f64` from offset zero.
    pub fn bytes_to_f64(bytes: &[u8]) -> Result<f64, ByteUtilError> {
        Self::bytes_to_f64_with_order(bytes, Self::DEFAULT_ORDER)
    }

    /// Reads an `f64` from offset zero in `order`.
    pub fn bytes_to_f64_with_order(bytes: &[u8], order: ByteOrder) -> Result<f64, ByteUtilError> {
        let bits = read_array::<8>(bytes, 0)?;
        Ok(f64::from_bits(match order {
            ByteOrder::LittleEndian => u64::from_le_bytes(bits),
            ByteOrder::BigEndian => u64::from_be_bytes(bits),
        }))
    }

    /// Serializes a supported numeric type in the default little-endian order.
    #[must_use]
    pub fn number_to_bytes<T: NumberToBytes>(number: T) -> Vec<u8> {
        Self::number_to_bytes_with_order(number, Self::DEFAULT_ORDER)
    }

    /// Serializes a supported numeric type in `order`.
    #[must_use]
    pub fn number_to_bytes_with_order<T: NumberToBytes>(number: T, order: ByteOrder) -> Vec<u8> {
        number.number_to_bytes(order)
    }

    /// Converts bytes to a statically selected numeric target type.
    pub fn bytes_to_number<T: BytesToNumber>(
        bytes: &[u8],
        order: ByteOrder,
    ) -> Result<T, ByteUtilError> {
        T::bytes_to_number(bytes, order)
    }
}

macro_rules! impl_number_to_bytes {
    ($type:ty, $method:ident) => {
        impl NumberToBytes for $type {
            fn number_to_bytes(self, order: ByteOrder) -> Vec<u8> {
                ByteUtil::$method(self, order).to_vec()
            }
        }
    };
}

impl NumberToBytes for i8 {
    fn number_to_bytes(self, _order: ByteOrder) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}

impl_number_to_bytes!(i16, i16_to_bytes_with_order);
impl_number_to_bytes!(i32, i32_to_bytes_with_order);
impl_number_to_bytes!(i64, i64_to_bytes_with_order);
impl_number_to_bytes!(f32, f32_to_bytes_with_order);
impl_number_to_bytes!(f64, f64_to_bytes_with_order);

impl BytesToNumber for i8 {
    fn bytes_to_number(bytes: &[u8], _order: ByteOrder) -> Result<Self, ByteUtilError> {
        Ok(i8::from_ne_bytes(read_array::<1>(bytes, 0)?))
    }
}

macro_rules! impl_bytes_to_number {
    ($type:ty, $method:ident) => {
        impl BytesToNumber for $type {
            fn bytes_to_number(bytes: &[u8], order: ByteOrder) -> Result<Self, ByteUtilError> {
                ByteUtil::$method(bytes, order)
            }
        }
    };
}

impl_bytes_to_number!(i16, bytes_to_i16_with_order);
impl_bytes_to_number!(i32, bytes_to_i32_with_order);
impl_bytes_to_number!(i64, bytes_to_i64_with_order);
impl_bytes_to_number!(f32, bytes_to_f32_with_order);
impl_bytes_to_number!(f64, bytes_to_f64_with_order);

impl BytesToNumber for AtomicI32 {
    fn bytes_to_number(bytes: &[u8], order: ByteOrder) -> Result<Self, ByteUtilError> {
        Ok(Self::new(ByteUtil::bytes_to_i32_with_order(bytes, order)?))
    }
}

impl BytesToNumber for AtomicI64 {
    fn bytes_to_number(bytes: &[u8], order: ByteOrder) -> Result<Self, ByteUtilError> {
        Ok(Self::new(ByteUtil::bytes_to_i64_with_order(bytes, order)?))
    }
}

impl BytesToNumber for LongAdder {
    fn bytes_to_number(bytes: &[u8], order: ByteOrder) -> Result<Self, ByteUtilError> {
        Ok(Self::new(ByteUtil::bytes_to_i64_with_order(bytes, order)?))
    }
}

impl BytesToNumber for DoubleAdder {
    fn bytes_to_number(bytes: &[u8], order: ByteOrder) -> Result<Self, ByteUtilError> {
        Ok(Self::new(ByteUtil::bytes_to_f64_with_order(bytes, order)?))
    }
}

impl BytesToNumber for BigInt {
    fn bytes_to_number(bytes: &[u8], order: ByteOrder) -> Result<Self, ByteUtilError> {
        Ok(Self::from(ByteUtil::bytes_to_i64_with_order(bytes, order)?))
    }
}

impl BytesToNumber for Decimal {
    fn bytes_to_number(bytes: &[u8], order: ByteOrder) -> Result<Self, ByteUtilError> {
        let value = ByteUtil::bytes_to_f64_with_order(bytes, order)?;
        Self::from_str(&value.to_string()).map_err(|_| ByteUtilError::Decimal(value.to_string()))
    }
}

fn read_array<const SIZE: usize>(bytes: &[u8], start: usize) -> Result<[u8; SIZE], ByteUtilError> {
    let available = bytes.len().saturating_sub(start);
    let source = bytes
        .get(start..)
        .and_then(|remaining| remaining.get(..SIZE))
        .ok_or(ByteUtilError::InsufficientBytes {
            start,
            required: SIZE,
            available,
        })?;
    let mut output = [0_u8; SIZE];
    output.copy_from_slice(source);
    Ok(output)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitive_integer_conversions_cover_defaults_endianness_offsets_and_bounds() {
        assert_eq!(ByteUtil::DEFAULT_ORDER, ByteOrder::LittleEndian);
        #[cfg(target_endian = "little")]
        assert_eq!(ByteUtil::CPU_ENDIAN, ByteOrder::LittleEndian);
        #[cfg(target_endian = "big")]
        assert_eq!(ByteUtil::CPU_ENDIAN, ByteOrder::BigEndian);
        assert_eq!(ByteUtil::int_to_byte(0x1ff), -1);
        assert_eq!(ByteUtil::byte_to_unsigned_int(-1), 255);

        let i16_value = -0x1234_i16;
        assert_eq!(ByteUtil::i16_to_bytes(i16_value), i16_value.to_le_bytes());
        assert_eq!(
            ByteUtil::bytes_to_i16(&i16_value.to_le_bytes()).unwrap(),
            i16_value
        );
        assert_eq!(
            ByteUtil::bytes_to_i16_with_order(&i16_value.to_be_bytes(), ByteOrder::BigEndian)
                .unwrap(),
            i16_value
        );
        assert_eq!(
            ByteUtil::bytes_to_i16_at(&[9, 9, 0x12, 0x34], 2, ByteOrder::BigEndian).unwrap(),
            0x1234
        );
        assert_eq!(
            ByteUtil::i16_to_bytes_with_order(i16_value, ByteOrder::BigEndian),
            i16_value.to_be_bytes()
        );

        let i32_value = -0x0123_4567_i32;
        assert_eq!(ByteUtil::i32_to_bytes(i32_value), i32_value.to_le_bytes());
        assert_eq!(
            ByteUtil::bytes_to_i32(&i32_value.to_le_bytes()).unwrap(),
            i32_value
        );
        assert_eq!(
            ByteUtil::bytes_to_i32_with_order(&i32_value.to_be_bytes(), ByteOrder::BigEndian)
                .unwrap(),
            i32_value
        );
        let mut prefixed_i32 = vec![1, 2];
        prefixed_i32.extend_from_slice(&i32_value.to_be_bytes());
        assert_eq!(
            ByteUtil::bytes_to_i32_at(&prefixed_i32, 2, ByteOrder::BigEndian).unwrap(),
            i32_value
        );
        assert_eq!(
            ByteUtil::i32_to_bytes_with_order(i32_value, ByteOrder::BigEndian),
            i32_value.to_be_bytes()
        );

        let i64_value = -0x0123_4567_8123_4567_i64;
        assert_eq!(ByteUtil::i64_to_bytes(i64_value), i64_value.to_le_bytes());
        assert_eq!(
            ByteUtil::bytes_to_i64(&i64_value.to_le_bytes()).unwrap(),
            i64_value
        );
        assert_eq!(
            ByteUtil::bytes_to_i64_with_order(&i64_value.to_be_bytes(), ByteOrder::BigEndian)
                .unwrap(),
            i64_value
        );
        let mut prefixed_i64 = vec![1, 2];
        prefixed_i64.extend_from_slice(&i64_value.to_be_bytes());
        assert_eq!(
            ByteUtil::bytes_to_i64_at(&prefixed_i64, 2, ByteOrder::BigEndian).unwrap(),
            i64_value
        );
        assert_eq!(
            ByteUtil::i64_to_bytes_with_order(i64_value, ByteOrder::BigEndian),
            i64_value.to_be_bytes()
        );

        let error = ByteUtil::bytes_to_i64_at(&[1, 2], 9, ByteOrder::LittleEndian).unwrap_err();
        assert_eq!(
            error,
            ByteUtilError::InsufficientBytes {
                start: 9,
                required: 8,
                available: 0,
            }
        );
        assert!(ByteUtil::bytes_to_i16(&[1]).is_err());
        assert!(ByteUtil::bytes_to_i32(&[1, 2, 3]).is_err());
    }

    #[test]
    fn floating_conversions_preserve_values_endianness_and_java_nan_canonicalization() {
        for value in [1.5_f32, -0.0, f32::INFINITY] {
            assert_eq!(
                ByteUtil::bytes_to_f32(&ByteUtil::f32_to_bytes(value))
                    .unwrap()
                    .to_bits(),
                value.to_bits()
            );
            assert_eq!(
                ByteUtil::bytes_to_f32_with_order(
                    &ByteUtil::f32_to_bytes_with_order(value, ByteOrder::BigEndian),
                    ByteOrder::BigEndian,
                )
                .unwrap()
                .to_bits(),
                value.to_bits()
            );
        }
        for value in [1.5_f64, -0.0, f64::NEG_INFINITY] {
            assert_eq!(
                ByteUtil::bytes_to_f64(&ByteUtil::f64_to_bytes(value))
                    .unwrap()
                    .to_bits(),
                value.to_bits()
            );
            assert_eq!(
                ByteUtil::bytes_to_f64_with_order(
                    &ByteUtil::f64_to_bytes_with_order(value, ByteOrder::BigEndian),
                    ByteOrder::BigEndian,
                )
                .unwrap()
                .to_bits(),
                value.to_bits()
            );
        }
        assert_eq!(
            ByteUtil::f32_to_bytes(f32::from_bits(0x7fa0_0001)),
            0x7fc0_0000_u32.to_le_bytes()
        );
        assert_eq!(
            ByteUtil::f64_to_bytes_with_order(
                f64::from_bits(0x7ff0_0000_0000_0001),
                ByteOrder::BigEndian,
            ),
            0x7ff8_0000_0000_0000_u64.to_be_bytes()
        );
        assert!(ByteUtil::bytes_to_f32(&[1]).is_err());
        assert!(ByteUtil::bytes_to_f64(&[1]).is_err());
    }

    #[test]
    fn number_to_bytes_trait_covers_every_java_wrapper_shape() {
        assert_eq!(ByteUtil::number_to_bytes(-1_i8), vec![0xff]);
        assert_eq!(ByteUtil::number_to_bytes(-2_i16), (-2_i16).to_le_bytes());
        assert_eq!(ByteUtil::number_to_bytes(-3_i32), (-3_i32).to_le_bytes());
        assert_eq!(ByteUtil::number_to_bytes(-4_i64), (-4_i64).to_le_bytes());
        assert_eq!(
            ByteUtil::number_to_bytes(1.5_f32),
            1.5_f32.to_bits().to_le_bytes()
        );
        assert_eq!(
            ByteUtil::number_to_bytes_with_order(1.5_f64, ByteOrder::BigEndian),
            1.5_f64.to_bits().to_be_bytes()
        );
    }

    #[test]
    fn bytes_to_number_trait_covers_primitives_atomics_adders_and_big_numbers() {
        let order = ByteOrder::LittleEndian;
        assert_eq!(ByteUtil::bytes_to_number::<i8>(&[0xff], order).unwrap(), -1);
        assert_eq!(
            ByteUtil::bytes_to_number::<i16>(&(-2_i16).to_le_bytes(), order).unwrap(),
            -2
        );
        assert_eq!(
            ByteUtil::bytes_to_number::<i32>(&(-3_i32).to_le_bytes(), order).unwrap(),
            -3
        );
        assert_eq!(
            ByteUtil::bytes_to_number::<i64>(&(-4_i64).to_le_bytes(), order).unwrap(),
            -4
        );
        assert_eq!(
            ByteUtil::bytes_to_number::<f32>(&1.5_f32.to_bits().to_le_bytes(), order)
                .unwrap()
                .to_bits(),
            1.5_f32.to_bits()
        );
        assert_eq!(
            ByteUtil::bytes_to_number::<f64>(&1.5_f64.to_bits().to_le_bytes(), order)
                .unwrap()
                .to_bits(),
            1.5_f64.to_bits()
        );

        let atomic_i32 =
            ByteUtil::bytes_to_number::<AtomicI32>(&42_i32.to_le_bytes(), order).unwrap();
        assert_eq!(atomic_i32.load(Ordering::Relaxed), 42);
        let atomic_i64 =
            ByteUtil::bytes_to_number::<AtomicI64>(&43_i64.to_le_bytes(), order).unwrap();
        assert_eq!(atomic_i64.load(Ordering::Relaxed), 43);

        let long_adder =
            ByteUtil::bytes_to_number::<LongAdder>(&44_i64.to_le_bytes(), order).unwrap();
        long_adder.add(1);
        assert_eq!(long_adder.sum(), 45);
        let double_adder =
            ByteUtil::bytes_to_number::<DoubleAdder>(&1.5_f64.to_bits().to_le_bytes(), order)
                .unwrap();
        double_adder.add(0.5);
        assert_eq!(double_adder.sum().to_bits(), 2.0_f64.to_bits());

        assert_eq!(
            ByteUtil::bytes_to_number::<BigInt>(&45_i64.to_le_bytes(), order).unwrap(),
            BigInt::from(45)
        );
        assert_eq!(
            ByteUtil::bytes_to_number::<Decimal>(&1.5_f64.to_bits().to_le_bytes(), order).unwrap(),
            Decimal::new(15, 1)
        );
        assert_eq!(
            ByteUtil::bytes_to_number::<Decimal>(&f64::NAN.to_bits().to_le_bytes(), order)
                .unwrap_err(),
            ByteUtilError::Decimal("NaN".to_owned())
        );
        assert!(ByteUtil::bytes_to_number::<i8>(&[], order).is_err());
        assert!(ByteUtil::bytes_to_number::<Decimal>(&[1], order).is_err());
        assert!(ByteUtil::bytes_to_number::<AtomicI32>(&[1], order).is_err());
        assert!(ByteUtil::bytes_to_number::<AtomicI64>(&[1], order).is_err());
        assert!(ByteUtil::bytes_to_number::<LongAdder>(&[1], order).is_err());
        assert!(ByteUtil::bytes_to_number::<DoubleAdder>(&[1], order).is_err());
        assert!(ByteUtil::bytes_to_number::<BigInt>(&[1], order).is_err());
    }
}
