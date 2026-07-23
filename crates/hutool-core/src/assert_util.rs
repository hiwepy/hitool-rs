//! 对齐: `cn.hutool.core.lang.Assert`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Assert.java
//!
//! Rust 惯用实现：断言失败时 `panic!`（对齐 Java `IllegalArgumentException` /
//! `IllegalStateException` 抛出语义），可用 `std::panic::catch_unwind` 捕获。

use crate::format_template;
use std::fmt::Display;

/// 对齐 Java: `cn.hutool.core.lang.Assert`（静态工具 → ZST + 关联函数）
#[derive(Debug, Clone, Copy, Default)]
pub struct Assert;

impl Assert {
    /// 格式化 `{}` 模板消息。
    fn fmt_msg(template: &str, params: &[&dyn Display]) -> String {
        if params.is_empty() {
            template.to_string()
        } else {
            format_template(template, params)
        }
    }

    /// 对齐 Java: `Assert.isTrue(boolean)`
    pub fn is_true(expression: bool) {
        Self::is_true_msg(expression, "[Assertion failed] - this expression must be true", &[]);
    }

    /// 对齐 Java: `Assert.isTrue(boolean, String, Object...)`
    pub fn is_true_msg(expression: bool, template: &str, params: &[&dyn Display]) {
        if !expression {
            panic!("{}", Self::fmt_msg(template, params));
        }
    }

    /// 对齐 Java: `Assert.isTrue(boolean, Supplier)`
    pub fn is_true_with<F>(expression: bool, supplier: F)
    where
        F: FnOnce() -> String,
    {
        if !expression {
            panic!("{}", supplier());
        }
    }

    /// 对齐 Java: `Assert.isFalse(boolean)`
    pub fn is_false(expression: bool) {
        Self::is_false_msg(expression, "[Assertion failed] - this expression must be false", &[]);
    }

    /// 对齐 Java: `Assert.isFalse(boolean, String, Object...)`
    pub fn is_false_msg(expression: bool, template: &str, params: &[&dyn Display]) {
        if expression {
            panic!("{}", Self::fmt_msg(template, params));
        }
    }

    /// 对齐 Java: `Assert.isFalse(boolean, Supplier)`
    pub fn is_false_with<F>(expression: bool, supplier: F)
    where
        F: FnOnce() -> String,
    {
        if expression {
            panic!("{}", supplier());
        }
    }

    /// 对齐 Java: `Assert.isNull(Object)` —— `None` 视为 null。
    pub fn is_null<T>(value: Option<T>) {
        Self::is_null_msg(value, "[Assertion failed] - the object argument must be null", &[]);
    }

    /// 对齐 Java: `Assert.isNull(Object, String, Object...)`
    pub fn is_null_msg<T>(value: Option<T>, template: &str, params: &[&dyn Display]) {
        if value.is_some() {
            panic!("{}", Self::fmt_msg(template, params));
        }
    }

    /// 对齐 Java: `Assert.isNull(Object, Supplier)`
    pub fn is_null_with<T, F>(value: Option<T>, supplier: F)
    where
        F: FnOnce() -> String,
    {
        if value.is_some() {
            panic!("{}", supplier());
        }
    }

    /// 对齐 Java: `Assert.notNull(T)`
    pub fn not_null<T>(value: Option<T>) -> T {
        Self::not_null_msg(
            value,
            "[Assertion failed] - this argument is required; it must not be null",
            &[],
        )
    }

    /// 对齐 Java: `Assert.notNull(T, String, Object...)`
    pub fn not_null_msg<T>(value: Option<T>, template: &str, params: &[&dyn Display]) -> T {
        match value {
            Some(v) => v,
            None => panic!("{}", Self::fmt_msg(template, params)),
        }
    }

    /// 对齐 Java: `Assert.notNull(T, Supplier)`
    pub fn not_null_with<T, F>(value: Option<T>, supplier: F) -> T
    where
        F: FnOnce() -> String,
    {
        match value {
            Some(v) => v,
            None => panic!("{}", supplier()),
        }
    }

    /// 对齐 Java: `Assert.notEmpty(CharSequence)`
    pub fn not_empty_str<'a>(text: Option<&'a str>) -> &'a str {
        Self::not_empty_str_msg(
            text,
            "[Assertion failed] - this String argument must have length; it must not be null or empty",
            &[],
        )
    }

    /// 对齐 Java: `Assert.notEmpty(CharSequence, String, Object...)`
    pub fn not_empty_str_msg<'a>(
        text: Option<&'a str>,
        template: &str,
        params: &[&dyn Display],
    ) -> &'a str {
        match text {
            Some(s) if !s.is_empty() => s,
            _ => panic!("{}", Self::fmt_msg(template, params)),
        }
    }

    /// 对齐 Java: `Assert.notEmpty(CharSequence, Supplier)`
    pub fn not_empty_str_with<'a, F>(text: Option<&'a str>, supplier: F) -> &'a str
    where
        F: FnOnce() -> String,
    {
        match text {
            Some(s) if !s.is_empty() => s,
            _ => panic!("{}", supplier()),
        }
    }

    /// 对齐 Java: `Assert.notEmpty(Collection)` / 数组切片
    pub fn not_empty_slice<'a, T>(items: Option<&'a [T]>) -> &'a [T] {
        Self::not_empty_slice_msg(
            items,
            "[Assertion failed] - this collection must not be empty: it must contain at least 1 element",
            &[],
        )
    }

    /// 对齐 Java: `Assert.notEmpty(Collection, String, Object...)`
    pub fn not_empty_slice_msg<'a, T>(
        items: Option<&'a [T]>,
        template: &str,
        params: &[&dyn Display],
    ) -> &'a [T] {
        match items {
            Some(s) if !s.is_empty() => s,
            _ => panic!("{}", Self::fmt_msg(template, params)),
        }
    }

    /// 对齐 Java: `Assert.notEmpty(Map)` —— 以键值对切片表达
    pub fn not_empty_map<K, V>(map: &std::collections::HashMap<K, V>) -> &std::collections::HashMap<K, V>
    where
        K: Eq + std::hash::Hash,
    {
        if map.is_empty() {
            panic!(
                "[Assertion failed] - this map must not be empty; it must contain at least one entry"
            );
        }
        map
    }

    /// 对齐 Java: `Assert.notBlank(CharSequence)`
    pub fn not_blank<'a>(text: Option<&'a str>) -> &'a str {
        Self::not_blank_msg(
            text,
            "[Assertion failed] - this String argument must have text; it must not be null, empty, or blank",
            &[],
        )
    }

    /// 对齐 Java: `Assert.notBlank(CharSequence, String, Object...)`
    pub fn not_blank_msg<'a>(
        text: Option<&'a str>,
        template: &str,
        params: &[&dyn Display],
    ) -> &'a str {
        match text {
            Some(s) if !crate::is_blank(s) => s,
            _ => panic!("{}", Self::fmt_msg(template, params)),
        }
    }

    /// 对齐 Java: `Assert.notBlank(CharSequence, Supplier)`
    pub fn not_blank_with<'a, F>(text: Option<&'a str>, supplier: F) -> &'a str
    where
        F: FnOnce() -> String,
    {
        match text {
            Some(s) if !crate::is_blank(s) => s,
            _ => panic!("{}", supplier()),
        }
    }

    /// 对齐 Java: `Assert.empty(Collection)` —— 断言为空
    pub fn empty_slice<T>(items: &[T]) {
        Self::empty_slice_msg(
            items,
            "[Assertion failed] - this collection must be empty: it must not contain any element",
            &[],
        );
    }

    /// 对齐 Java: `Assert.empty(Collection, String, Object...)`
    pub fn empty_slice_msg<T>(items: &[T], template: &str, params: &[&dyn Display]) {
        if !items.is_empty() {
            panic!("{}", Self::fmt_msg(template, params));
        }
    }

    /// 对齐 Java: `Assert.empty` + Supplier
    pub fn empty_slice_with<T, F>(items: &[T], supplier: F)
    where
        F: FnOnce() -> String,
    {
        if !items.is_empty() {
            panic!("{}", supplier());
        }
    }

    /// 对齐 Java: `Assert.notContain(String, String)` —— text 不含 test_str
    pub fn not_contain(text: &str, test_str: &str) {
        Self::not_contain_msg(
            text,
            test_str,
            "[Assertion failed] - this String argument must not contain the substring [{}]",
            &[&test_str],
        );
    }

    /// 对齐 Java: `Assert.notContain(..., String, Object...)`
    pub fn not_contain_msg(
        text: &str,
        test_str: &str,
        template: &str,
        params: &[&dyn Display],
    ) {
        if text.contains(test_str) {
            panic!("{}", Self::fmt_msg(template, params));
        }
    }

    /// 对齐 Java: `Assert.notContain` + Supplier
    pub fn not_contain_with<F>(text: &str, test_str: &str, supplier: F)
    where
        F: FnOnce() -> String,
    {
        if text.contains(test_str) {
            panic!("{}", supplier());
        }
    }

    /// 对齐 Java: `Assert.noNullElements(Object[])` —— 切片元素均 Some
    pub fn no_null_elements<T>(array: &[Option<T>]) {
        Self::no_null_elements_msg(
            array,
            "[Assertion failed] - this array must not contain any null elements",
            &[],
        );
    }

    /// 对齐 Java: `Assert.noNullElements` + 消息
    pub fn no_null_elements_msg<T>(
        array: &[Option<T>],
        template: &str,
        params: &[&dyn Display],
    ) {
        if array.iter().any(|e| e.is_none()) {
            panic!("{}", Self::fmt_msg(template, params));
        }
    }

    /// 对齐 Java: `Assert.noNullElements` + Supplier
    pub fn no_null_elements_with<T, F>(array: &[Option<T>], supplier: F)
    where
        F: FnOnce() -> String,
    {
        if array.iter().any(|e| e.is_none()) {
            panic!("{}", supplier());
        }
    }

    /// 对齐 Java: `Assert.equals(Object, Object)`
    pub fn equals<T: PartialEq + Display>(a: &T, b: &T) {
        Self::equals_msg(a, b, "[Assertion failed] - these objects must be equal", &[]);
    }

    /// 对齐 Java: `Assert.equals` + 消息
    pub fn equals_msg<T: PartialEq>(
        a: &T,
        b: &T,
        template: &str,
        params: &[&dyn Display],
    ) {
        if a != b {
            panic!("{}", Self::fmt_msg(template, params));
        }
    }

    /// 对齐 Java: `Assert.equals` + Supplier
    pub fn equals_with<T: PartialEq, F>(a: &T, b: &T, supplier: F)
    where
        F: FnOnce() -> String,
    {
        if a != b {
            panic!("{}", supplier());
        }
    }

    /// 对齐 Java: `Assert.notEquals(Object, Object)`
    pub fn not_equals<T: PartialEq>(a: &T, b: &T) {
        Self::not_equals_msg(
            a,
            b,
            "[Assertion failed] - these objects must not be equal",
            &[],
        );
    }

    /// 对齐 Java: `Assert.notEquals` + 消息
    pub fn not_equals_msg<T: PartialEq>(
        a: &T,
        b: &T,
        template: &str,
        params: &[&dyn Display],
    ) {
        if a == b {
            panic!("{}", Self::fmt_msg(template, params));
        }
    }

    /// 对齐 Java: `Assert.notEquals` + Supplier
    pub fn not_equals_with<T: PartialEq, F>(a: &T, b: &T, supplier: F)
    where
        F: FnOnce() -> String,
    {
        if a == b {
            panic!("{}", supplier());
        }
    }

    /// 对齐 Java: `Assert.state(boolean)` —— IllegalStateException 语义
    pub fn state(expression: bool) {
        Self::state_msg(
            expression,
            "[Assertion failed] - this state invariant must be true",
            &[],
        );
    }

    /// 对齐 Java: `Assert.state(boolean, String, Object...)`
    pub fn state_msg(expression: bool, template: &str, params: &[&dyn Display]) {
        if !expression {
            panic!("{}", Self::fmt_msg(template, params));
        }
    }

    /// 对齐 Java: `Assert.state` + Supplier
    pub fn state_with<F>(expression: bool, supplier: F)
    where
        F: FnOnce() -> String,
    {
        if !expression {
            panic!("{}", supplier());
        }
    }

    /// 对齐 Java: `Assert.checkIndex(int, int)` —— index ∈ [0, size)
    pub fn check_index(index: isize, size: isize) -> isize {
        Self::check_index_msg(
            index,
            size,
            "[Assertion failed]",
            &[],
        )
    }

    /// 对齐 Java: `Assert.checkIndex` + 消息
    pub fn check_index_msg(
        index: isize,
        size: isize,
        template: &str,
        params: &[&dyn Display],
    ) -> isize {
        if index < 0 || index >= size {
            panic!("{}", Self::fmt_msg(template, params));
        }
        index
    }

    /// 对齐 Java: `Assert.checkBetween(int, int, int)`
    pub fn check_between_i32(value: i32, min: i32, max: i32) -> i32 {
        if value < min || value > max {
            panic!("The value must be between {} and {}.", min, max);
        }
        value
    }

    /// 对齐 Java: `Assert.checkBetween(long, long, long)`
    pub fn check_between_i64(value: i64, min: i64, max: i64) -> i64 {
        if value < min || value > max {
            panic!("The value must be between {} and {}.", min, max);
        }
        value
    }

    /// 对齐 Java: `Assert.checkBetween(double, double, double)`
    pub fn check_between_f64(value: f64, min: f64, max: f64) -> f64 {
        if value < min || value > max {
            panic!("The value must be between {} and {}.", min, max);
        }
        value
    }

    /// 对齐 Java: `Assert.checkBetween(Number, Number, Number)` —— 用 f64 表达
    pub fn check_between_number(value: f64, min: f64, max: f64) -> f64 {
        Self::check_between_f64(value, min, max)
    }

    /// 对齐 Java: `Assert.checkBetween` + 自定义消息（i64）
    pub fn check_between_i64_msg(
        value: i64,
        min: i64,
        max: i64,
        template: &str,
        params: &[&dyn Display],
    ) -> i64 {
        if value < min || value > max {
            panic!("{}", Self::fmt_msg(template, params));
        }
        value
    }

    /// 对齐 Java: `Assert.checkBetween` + Supplier（i64）
    pub fn check_between_i64_with<F>(value: i64, min: i64, max: i64, supplier: F) -> i64
    where
        F: FnOnce() -> String,
    {
        if value < min || value > max {
            panic!("{}", supplier());
        }
        value
    }

    /// 对齐 Java: `Assert.isInstanceOf` —— Rust 用 `Any` + TypeId 无法跨类型擦除；
    /// 此处提供类型名字符串检查门面（真正反射归 planned）。
    pub fn is_instance_of(type_name: &str, actual_type_name: &str) {
        if type_name != actual_type_name {
            panic!(
                "[Assertion failed] - object of class [{}] must be an instance of {}",
                actual_type_name, type_name
            );
        }
    }

    /// 对齐 Java: `Assert.isAssignable` —— 类型名字符串门面
    pub fn is_assignable(super_type: &str, sub_type: &str) {
        if super_type != sub_type {
            panic!(
                "[Assertion failed] - {} is not assignable to {}",
                sub_type, super_type
            );
        }
    }
}
