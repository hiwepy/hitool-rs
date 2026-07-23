//! 对齐: `cn.hutool.core.comparator.PropertyComparator` / `FieldsComparator`
//! 来源: hutool-core PropertyComparator / FieldsComparator / FuncComparator

use std::cmp::Ordering;

/// 属性比较器 —— 对齐 Java `PropertyComparator`（Rust 用提取闭包代替 Bean 反射）。
pub struct PropertyComparator<T, F>
where
    F: Fn(&T) -> Option<String>,
{
    extractor: F,
    null_greater: bool,
    _marker: std::marker::PhantomData<T>,
}

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

/// 反序属性比较器。
pub struct ReversedPropertyComparator<T, F>
where
    F: Fn(&T) -> Option<String>,
{
    inner: PropertyComparator<T, F>,
}

impl<T, F> ReversedPropertyComparator<T, F>
where
    F: Fn(&T) -> Option<String>,
{
    /// 反序比较。
    pub fn compare(&self, a: &T, b: &T) -> i32 {
        -self.inner.compare(a, b)
    }
}

/// 多字段比较器 —— 对齐 Java `FieldsComparator`。
pub struct FieldsComparator<T> {
    extractors: Vec<Box<dyn Fn(&T) -> Option<i32> + Send + Sync>>,
}

impl<T> FieldsComparator<T> {
    /// 用多个 i32 字段提取器构造（对齐 Java 字段名列表的测试向量）。
    pub fn new(extractors: Vec<Box<dyn Fn(&T) -> Option<i32> + Send + Sync>>) -> Self {
        Self { extractors }
    }

    /// 对齐 Java: `compare(T, T)`
    pub fn compare(&self, a: &T, b: &T) -> i32 {
        for ex in &self.extractors {
            let v1 = ex(a);
            let v2 = ex(b);
            let c = compare_nullable_i32(v1, v2, true);
            if c != 0 {
                return c;
            }
        }
        0
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

fn compare_nullable_i32(a: Option<i32>, b: Option<i32>, null_greater: bool) -> i32 {
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
        (Some(x), Some(y)) => x.cmp(&y) as i32,
    }
}
