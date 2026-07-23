//! 对齐: `cn.hutool.core.lang.Opt`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Opt.java
//!
//! Hutool `Opt` 是增强版 `Optional`；Rust 用泛型包装 + 失败态（`exception`）。

use std::fmt::{Debug, Display};

/// 对齐 Java: `cn.hutool.core.lang.Opt<T>`
#[derive(Clone, Debug)]
pub struct Opt<T> {
    value: Option<T>,
    /// 对齐 `exception` 字段：`ofTry` 失败时保留错误消息。
    exception: Option<String>,
}

impl<T> Default for Opt<T> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<T> Opt<T> {
    /// 对齐 Java: `Opt.empty()`
    pub fn empty() -> Self {
        Self {
            value: None,
            exception: None,
        }
    }

    /// 对齐 Java: `Opt.of(T)` —— 值不可为 None 语义（直接 Some）
    pub fn of(value: T) -> Self {
        Self {
            value: Some(value),
            exception: None,
        }
    }

    /// 对齐 Java: `Opt.ofNullable(T)`
    pub fn of_nullable(value: Option<T>) -> Self {
        Self {
            value,
            exception: None,
        }
    }

    /// 对齐 Java: `Opt.ofTry(Func0)` —— 闭包失败转为 fail 态
    pub fn of_try<F, E>(supplier: F) -> Self
    where
        F: FnOnce() -> Result<T, E>,
        E: Display,
    {
        match supplier() {
            Ok(v) => Self::of(v),
            Err(e) => Self {
                value: None,
                exception: Some(e.to_string()),
            },
        }
    }

    /// 对齐 Java: `Opt.isPresent()`
    pub fn is_present(&self) -> bool {
        self.value.is_some()
    }

    /// 对齐 Java: `Opt.isEmpty()`
    pub fn is_empty(&self) -> bool {
        self.value.is_none()
    }

    /// 对齐 Java: `Opt.isFail()`
    pub fn is_fail(&self) -> bool {
        self.exception.is_some()
    }

    /// 对齐 Java: `Opt.get()`
    pub fn get(&self) -> Option<&T> {
        self.value.as_ref()
    }

    /// 对齐 Java: `Opt.getException()`
    pub fn get_exception(&self) -> Option<&str> {
        self.exception.as_deref()
    }

    /// 对齐 Java: `Opt.orElse(T)`
    pub fn or_else(self, other: T) -> T {
        self.value.unwrap_or(other)
    }

    /// 对齐 Java: `Opt.orElseGet(Supplier)`
    pub fn or_else_get<F>(self, supplier: F) -> T
    where
        F: FnOnce() -> T,
    {
        self.value.unwrap_or_else(supplier)
    }

    /// 对齐 Java: `Opt.orElseThrow()`
    pub fn or_else_throw(self) -> T {
        self.value.expect("Opt value is empty")
    }

    /// 对齐 Java: `Opt.map(Function)`
    pub fn map<U, F>(self, f: F) -> Opt<U>
    where
        F: FnOnce(T) -> U,
    {
        Opt {
            value: self.value.map(f),
            exception: self.exception,
        }
    }

    /// 对齐 Java: `Opt.flatMap(Function)`
    pub fn flat_map<U, F>(self, f: F) -> Opt<U>
    where
        F: FnOnce(T) -> Opt<U>,
    {
        match self.value {
            Some(v) => {
                let mut next = f(v);
                if next.exception.is_none() {
                    next.exception = self.exception;
                }
                next
            }
            None => Opt {
                value: None,
                exception: self.exception,
            },
        }
    }

    /// 对齐 Java: `Opt.filter(Predicate)`
    pub fn filter<F>(self, predicate: F) -> Self
    where
        F: FnOnce(&T) -> bool,
    {
        match self.value {
            Some(ref v) if predicate(v) => self,
            Some(_) => Self {
                value: None,
                exception: self.exception,
            },
            None => self,
        }
    }

    /// 对齐 Java: `Opt.ifPresent(Consumer)`
    pub fn if_present<F>(&self, consumer: F)
    where
        F: FnOnce(&T),
    {
        if let Some(ref v) = self.value {
            consumer(v);
        }
    }

    /// 对齐 Java: `Opt.ifPresentOrElse`
    pub fn if_present_or_else<P, E>(&self, present: P, empty: E)
    where
        P: FnOnce(&T),
        E: FnOnce(),
    {
        match &self.value {
            Some(v) => present(v),
            None => empty(),
        }
    }

    /// 对齐 Java: `Opt.ifFail(Consumer)`
    pub fn if_fail<F>(&self, consumer: F)
    where
        F: FnOnce(&str),
    {
        if let Some(ref e) = self.exception {
            consumer(e);
        }
    }

    /// 对齐 Java: `Opt.or(Supplier)`
    pub fn or<F>(self, supplier: F) -> Self
    where
        F: FnOnce() -> Opt<T>,
    {
        if self.is_present() {
            self
        } else {
            supplier()
        }
    }

    /// 对齐 Java: `Opt.peek(Consumer)` —— 副作用后返回 self
    pub fn peek<F>(self, consumer: F) -> Self
    where
        F: FnOnce(&T),
    {
        if let Some(ref v) = self.value {
            consumer(v);
        }
        self
    }

    /// 对齐 Java: `Opt.toOptional()` —— 转为标准 Option
    pub fn to_optional(self) -> Option<T> {
        self.value
    }

    /// 对齐 Java: `Opt.exceptionOrElse`
    pub fn exception_or_else(self, other: T) -> T {
        if self.is_fail() {
            other
        } else {
            self.or_else(other)
        }
    }
}

impl Opt<String> {
    /// 对齐 Java: `Opt.ofBlankAble(CharSequence)` —— blank 视为 empty
    pub fn of_blank_able(value: Option<&str>) -> Opt<String> {
        match value {
            Some(s) if !crate::is_blank(s) => Opt::of(s.to_string()),
            _ => Opt::empty(),
        }
    }
}

impl<T> Opt<Vec<T>> {
    /// 对齐 Java: `Opt.ofEmptyAble(Collection)` —— 空集合视为 empty
    pub fn of_empty_able(value: Vec<T>) -> Self {
        if value.is_empty() {
            Opt::empty()
        } else {
            Opt::of(value)
        }
    }
}

impl<T: PartialEq> PartialEq for Opt<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Eq> Eq for Opt<T> {}
