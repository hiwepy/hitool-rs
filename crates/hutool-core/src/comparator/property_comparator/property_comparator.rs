//! 对齐: `cn.hutool.core.comparator.PropertyComparator` / `FieldsComparator`
//! 来源: hutool-core PropertyComparator / FieldsComparator / FuncComparator

use std::cmp::Ordering;

use super::reversed_property_comparator::ReversedPropertyComparator;

/// 属性比较器 —— 对齐 Java `PropertyComparator`（Rust 用提取闭包代替 Bean 反射）。
pub struct PropertyComparator<T, F>

impl<T, F> PropertyComparator<T, F>
where
    F: Fn(&T) -> Option<String>,
{
    /// 对齐 Java: `PropertyComparator(property)` —— null 在末尾。
    pub fn new(extractor: F) -> Self {
        Self::with_null_greater(extractor, true)
    }

    /// 对齐 Java: `PropertyComparator(property, isNullGreater)`
    pub fn with_null_greater(extractor: F, null_greater: bool) -> Self {
        Self {
            extractor,
            null_greater,
            _marker: std::marker::PhantomData,
        }
    }

    /// 对齐 Java: `reversed()`
    pub fn reversed(self) -> ReversedPropertyComparator<T, F> {
        ReversedPropertyComparator { inner: self }
    }

    /// 对齐 Java: `compare(T, T)`
    pub fn compare(&self, a: &T, b: &T) -> i32 {
        let v1 = (self.extractor)(a);
        let v2 = (self.extractor)(b);
        compare_nullable_str(v1.as_deref(), v2.as_deref(), self.null_greater)
    }
}

fn compare_nullable_str(a: Option<&str>, b: Option<&str>, null_greater: bool) -> i32 {
    match (a, b) {
        (None, None) => 0,
        (None, Some(_)) => {
            if null_greater {
                1
            } else {
                -1
            }
        }
        (Some(_), None) => {
            if null_greater {
                -1
            } else {
                1
            }
        }
        (Some(x), Some(y)) => match x.cmp(y) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        },
    }
}
