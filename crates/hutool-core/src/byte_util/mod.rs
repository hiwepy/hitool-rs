//! Endian-aware numeric byte conversion aligned with Hutool's `ByteUtil` family.

use std::{
    str::FromStr,
    sync::atomic::{AtomicI32, AtomicI64, Ordering},
};

use num_bigint::BigInt;
use parking_lot::Mutex;
use rust_decimal::Decimal;

mod byte_order;
mod byte_util_error;
mod long_adder;
mod double_adder;
mod number_to_bytes;
mod bytes_to_number;
mod byte_util;

pub use byte_order::ByteOrder;
pub use byte_util_error::ByteUtilError;
pub use long_adder::LongAdder;
pub use double_adder::DoubleAdder;
pub use number_to_bytes::NumberToBytes;
pub use bytes_to_number::BytesToNumber;
pub use byte_util::ByteUtil;
