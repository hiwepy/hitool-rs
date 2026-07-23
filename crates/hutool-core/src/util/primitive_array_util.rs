//! 对齐: `cn.hutool.core.util.PrimitiveArrayUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/PrimitiveArrayUtil.java
//!
//! Rust 版本按 idiomatic 风格对每个公开方法提供关联函数桩;具体实现
//! 等待各 `hutool-rs` 引擎完成后回填,所有桩在调用时统一返回
//! `CoreError::PendingEngine`。
//!
//! 重载的 Java 方法通过 `<name>_<n>` 后缀区分,避免 Rust 关联函数重名冲突。

#![allow(dead_code, unused_variables, clippy::too_many_arguments, non_snake_case)]

use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.util.PrimitiveArrayUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct PrimitiveArrayUtil;

impl PrimitiveArrayUtil {
    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isEmpty#boolean (long[] array)`
    pub fn isEmpty(array: Vec<i64>) -> Result<bool> {
        Err(CoreError::PendingEngine("isEmpty"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isEmpty#boolean (int[] array)`
    pub fn isEmpty_2(array: Vec<i32>) -> Result<bool> {
        Err(CoreError::PendingEngine("isEmpty"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isEmpty#boolean (short[] array)`
    pub fn isEmpty_3(array: Vec<i16>) -> Result<bool> {
        Err(CoreError::PendingEngine("isEmpty"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isEmpty#boolean (char[] array)`
    pub fn isEmpty_4(array: Vec<char>) -> Result<bool> {
        Err(CoreError::PendingEngine("isEmpty"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isEmpty#boolean (byte[] array)`
    pub fn isEmpty_5(array: Vec<i8>) -> Result<bool> {
        Err(CoreError::PendingEngine("isEmpty"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isEmpty#boolean (double[] array)`
    pub fn isEmpty_6(array: Vec<f64>) -> Result<bool> {
        Err(CoreError::PendingEngine("isEmpty"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isEmpty#boolean (float[] array)`
    pub fn isEmpty_7(array: Vec<f32>) -> Result<bool> {
        Err(CoreError::PendingEngine("isEmpty"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isEmpty#boolean (boolean[] array)`
    pub fn isEmpty_8(array: Vec<bool>) -> Result<bool> {
        Err(CoreError::PendingEngine("isEmpty"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isNotEmpty#boolean (long[] array)`
    pub fn isNotEmpty(array: Vec<i64>) -> Result<bool> {
        Err(CoreError::PendingEngine("isNotEmpty"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isNotEmpty#boolean (int[] array)`
    pub fn isNotEmpty_2(array: Vec<i32>) -> Result<bool> {
        Err(CoreError::PendingEngine("isNotEmpty"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isNotEmpty#boolean (short[] array)`
    pub fn isNotEmpty_3(array: Vec<i16>) -> Result<bool> {
        Err(CoreError::PendingEngine("isNotEmpty"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isNotEmpty#boolean (char[] array)`
    pub fn isNotEmpty_4(array: Vec<char>) -> Result<bool> {
        Err(CoreError::PendingEngine("isNotEmpty"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isNotEmpty#boolean (byte[] array)`
    pub fn isNotEmpty_5(array: Vec<i8>) -> Result<bool> {
        Err(CoreError::PendingEngine("isNotEmpty"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isNotEmpty#boolean (double[] array)`
    pub fn isNotEmpty_6(array: Vec<f64>) -> Result<bool> {
        Err(CoreError::PendingEngine("isNotEmpty"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isNotEmpty#boolean (float[] array)`
    pub fn isNotEmpty_7(array: Vec<f32>) -> Result<bool> {
        Err(CoreError::PendingEngine("isNotEmpty"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isNotEmpty#boolean (boolean[] array)`
    pub fn isNotEmpty_8(array: Vec<bool>) -> Result<bool> {
        Err(CoreError::PendingEngine("isNotEmpty"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::resize#byte[] (byte[] bytes, int newSize)`
    pub fn resize(bytes: Vec<i8>, newSize: i32) -> Result<Vec<i8>> {
        Err(CoreError::PendingEngine("resize"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::addAll#byte[] (byte[]... arrays)`
    pub fn addAll(arrays: &[Vec<i8>]) -> Result<Vec<i8>> {
        Err(CoreError::PendingEngine("addAll"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::addAll#int[] (int[]... arrays)`
    pub fn addAll_2(arrays: &[Vec<i32>]) -> Result<Vec<i32>> {
        Err(CoreError::PendingEngine("addAll"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::addAll#long[] (long[]... arrays)`
    pub fn addAll_3(arrays: &[Vec<i64>]) -> Result<Vec<i64>> {
        Err(CoreError::PendingEngine("addAll"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::addAll#double[] (double[]... arrays)`
    pub fn addAll_4(arrays: &[Vec<f64>]) -> Result<Vec<f64>> {
        Err(CoreError::PendingEngine("addAll"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::addAll#float[] (float[]... arrays)`
    pub fn addAll_5(arrays: &[Vec<f32>]) -> Result<Vec<f32>> {
        Err(CoreError::PendingEngine("addAll"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::addAll#char[] (char[]... arrays)`
    pub fn addAll_6(arrays: &[Vec<char>]) -> Result<Vec<char>> {
        Err(CoreError::PendingEngine("addAll"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::addAll#boolean[] (boolean[]... arrays)`
    pub fn addAll_7(arrays: &[Vec<bool>]) -> Result<Vec<bool>> {
        Err(CoreError::PendingEngine("addAll"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::addAll#short[] (short[]... arrays)`
    pub fn addAll_8(arrays: &[Vec<i16>]) -> Result<Vec<i16>> {
        Err(CoreError::PendingEngine("addAll"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::range#int[] (int excludedEnd)`
    pub fn range(excludedEnd: i32) -> Result<Vec<i32>> {
        Err(CoreError::PendingEngine("range"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::range#int[] (int includedStart, int excludedEnd)`
    pub fn range_2(includedStart: i32, excludedEnd: i32) -> Result<Vec<i32>> {
        Err(CoreError::PendingEngine("range"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::range#int[] (int includedStart, int excludedEnd, int step)`
    pub fn range_3(includedStart: i32, excludedEnd: i32, step: i32) -> Result<Vec<i32>> {
        Err(CoreError::PendingEngine("range"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::split#byte[][] (byte[] array, int len)`
    pub fn split(array: Vec<i8>, len: i32) -> Result<Vec<Vec<i8>>> {
        Err(CoreError::PendingEngine("split"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::indexOf#int (long[] array, long value)`
    pub fn indexOf(array: Vec<i64>, value: i64) -> Result<i32> {
        Err(CoreError::PendingEngine("indexOf"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::lastIndexOf#int (long[] array, long value)`
    pub fn lastIndexOf(array: Vec<i64>, value: i64) -> Result<i32> {
        Err(CoreError::PendingEngine("lastIndexOf"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::contains#boolean (long[] array, long value)`
    pub fn contains(array: Vec<i64>, value: i64) -> Result<bool> {
        Err(CoreError::PendingEngine("contains"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::indexOf#int (int[] array, int value)`
    pub fn indexOf_2(array: Vec<i32>, value: i32) -> Result<i32> {
        Err(CoreError::PendingEngine("indexOf"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::lastIndexOf#int (int[] array, int value)`
    pub fn lastIndexOf_2(array: Vec<i32>, value: i32) -> Result<i32> {
        Err(CoreError::PendingEngine("lastIndexOf"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::contains#boolean (int[] array, int value)`
    pub fn contains_2(array: Vec<i32>, value: i32) -> Result<bool> {
        Err(CoreError::PendingEngine("contains"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::indexOf#int (short[] array, short value)`
    pub fn indexOf_3(array: Vec<i16>, value: i16) -> Result<i32> {
        Err(CoreError::PendingEngine("indexOf"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::lastIndexOf#int (short[] array, short value)`
    pub fn lastIndexOf_3(array: Vec<i16>, value: i16) -> Result<i32> {
        Err(CoreError::PendingEngine("lastIndexOf"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::contains#boolean (short[] array, short value)`
    pub fn contains_3(array: Vec<i16>, value: i16) -> Result<bool> {
        Err(CoreError::PendingEngine("contains"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::indexOf#int (char[] array, char value)`
    pub fn indexOf_4(array: Vec<char>, value: char) -> Result<i32> {
        Err(CoreError::PendingEngine("indexOf"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::lastIndexOf#int (char[] array, char value)`
    pub fn lastIndexOf_4(array: Vec<char>, value: char) -> Result<i32> {
        Err(CoreError::PendingEngine("lastIndexOf"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::contains#boolean (char[] array, char value)`
    pub fn contains_4(array: Vec<char>, value: char) -> Result<bool> {
        Err(CoreError::PendingEngine("contains"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::indexOf#int (byte[] array, byte value)`
    pub fn indexOf_5(array: Vec<i8>, value: i8) -> Result<i32> {
        Err(CoreError::PendingEngine("indexOf"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::lastIndexOf#int (byte[] array, byte value)`
    pub fn lastIndexOf_5(array: Vec<i8>, value: i8) -> Result<i32> {
        Err(CoreError::PendingEngine("lastIndexOf"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::contains#boolean (byte[] array, byte value)`
    pub fn contains_5(array: Vec<i8>, value: i8) -> Result<bool> {
        Err(CoreError::PendingEngine("contains"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::indexOf#int (double[] array, double value)`
    pub fn indexOf_6(array: Vec<f64>, value: f64) -> Result<i32> {
        Err(CoreError::PendingEngine("indexOf"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::lastIndexOf#int (double[] array, double value)`
    pub fn lastIndexOf_6(array: Vec<f64>, value: f64) -> Result<i32> {
        Err(CoreError::PendingEngine("lastIndexOf"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::contains#boolean (double[] array, double value)`
    pub fn contains_6(array: Vec<f64>, value: f64) -> Result<bool> {
        Err(CoreError::PendingEngine("contains"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::indexOf#int (float[] array, float value)`
    pub fn indexOf_7(array: Vec<f32>, value: f32) -> Result<i32> {
        Err(CoreError::PendingEngine("indexOf"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::lastIndexOf#int (float[] array, float value)`
    pub fn lastIndexOf_7(array: Vec<f32>, value: f32) -> Result<i32> {
        Err(CoreError::PendingEngine("lastIndexOf"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::contains#boolean (float[] array, float value)`
    pub fn contains_7(array: Vec<f32>, value: f32) -> Result<bool> {
        Err(CoreError::PendingEngine("contains"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::indexOf#int (boolean[] array, boolean value)`
    pub fn indexOf_8(array: Vec<bool>, value: bool) -> Result<i32> {
        Err(CoreError::PendingEngine("indexOf"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::lastIndexOf#int (boolean[] array, boolean value)`
    pub fn lastIndexOf_8(array: Vec<bool>, value: bool) -> Result<i32> {
        Err(CoreError::PendingEngine("lastIndexOf"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::contains#boolean (boolean[] array, boolean value)`
    pub fn contains_8(array: Vec<bool>, value: bool) -> Result<bool> {
        Err(CoreError::PendingEngine("contains"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::wrap#Integer[] (int... values)`
    pub fn wrap(values: &[i32]) -> Result<Vec<OPAQUE>> {
        Err(CoreError::PendingEngine("wrap"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::unWrap#int[] (Integer... values)`
    pub fn unWrap(values: &[OPAQUE]) -> Result<Vec<i32>> {
        Err(CoreError::PendingEngine("unWrap"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::wrap#Long[] (long... values)`
    pub fn wrap_2(values: &[i64]) -> Result<Vec<OPAQUE>> {
        Err(CoreError::PendingEngine("wrap"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::unWrap#long[] (Long... values)`
    pub fn unWrap_2(values: &[OPAQUE]) -> Result<Vec<i64>> {
        Err(CoreError::PendingEngine("unWrap"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::wrap#Character[] (char... values)`
    pub fn wrap_3(values: &[char]) -> Result<Vec<OPAQUE>> {
        Err(CoreError::PendingEngine("wrap"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::unWrap#char[] (Character... values)`
    pub fn unWrap_3(values: &[OPAQUE]) -> Result<Vec<char>> {
        Err(CoreError::PendingEngine("unWrap"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::wrap#Byte[] (byte... values)`
    pub fn wrap_4(values: &[i8]) -> Result<Vec<OPAQUE>> {
        Err(CoreError::PendingEngine("wrap"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::unWrap#byte[] (Byte... values)`
    pub fn unWrap_4(values: &[OPAQUE]) -> Result<Vec<i8>> {
        Err(CoreError::PendingEngine("unWrap"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::wrap#Short[] (short... values)`
    pub fn wrap_5(values: &[i16]) -> Result<Vec<OPAQUE>> {
        Err(CoreError::PendingEngine("wrap"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::unWrap#short[] (Short... values)`
    pub fn unWrap_5(values: &[OPAQUE]) -> Result<Vec<i16>> {
        Err(CoreError::PendingEngine("unWrap"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::wrap#Float[] (float... values)`
    pub fn wrap_6(values: &[f32]) -> Result<Vec<OPAQUE>> {
        Err(CoreError::PendingEngine("wrap"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::unWrap#float[] (Float... values)`
    pub fn unWrap_6(values: &[OPAQUE]) -> Result<Vec<f32>> {
        Err(CoreError::PendingEngine("unWrap"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::wrap#Double[] (double... values)`
    pub fn wrap_7(values: &[f64]) -> Result<Vec<OPAQUE>> {
        Err(CoreError::PendingEngine("wrap"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::unWrap#double[] (Double... values)`
    pub fn unWrap_7(values: &[OPAQUE]) -> Result<Vec<f64>> {
        Err(CoreError::PendingEngine("unWrap"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::wrap#Boolean[] (boolean... values)`
    pub fn wrap_8(values: &[bool]) -> Result<Vec<OPAQUE>> {
        Err(CoreError::PendingEngine("wrap"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::unWrap#boolean[] (Boolean... values)`
    pub fn unWrap_8(values: &[OPAQUE]) -> Result<Vec<bool>> {
        Err(CoreError::PendingEngine("unWrap"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::sub#byte[] (byte[] array, int start, int end)`
    pub fn sub(array: Vec<i8>, start: i32, end: i32) -> Result<Vec<i8>> {
        Err(CoreError::PendingEngine("sub"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::sub#int[] (int[] array, int start, int end)`
    pub fn sub_2(array: Vec<i32>, start: i32, end: i32) -> Result<Vec<i32>> {
        Err(CoreError::PendingEngine("sub"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::sub#long[] (long[] array, int start, int end)`
    pub fn sub_3(array: Vec<i64>, start: i32, end: i32) -> Result<Vec<i64>> {
        Err(CoreError::PendingEngine("sub"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::sub#short[] (short[] array, int start, int end)`
    pub fn sub_4(array: Vec<i16>, start: i32, end: i32) -> Result<Vec<i16>> {
        Err(CoreError::PendingEngine("sub"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::sub#char[] (char[] array, int start, int end)`
    pub fn sub_5(array: Vec<char>, start: i32, end: i32) -> Result<Vec<char>> {
        Err(CoreError::PendingEngine("sub"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::sub#double[] (double[] array, int start, int end)`
    pub fn sub_6(array: Vec<f64>, start: i32, end: i32) -> Result<Vec<f64>> {
        Err(CoreError::PendingEngine("sub"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::sub#float[] (float[] array, int start, int end)`
    pub fn sub_7(array: Vec<f32>, start: i32, end: i32) -> Result<Vec<f32>> {
        Err(CoreError::PendingEngine("sub"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::sub#boolean[] (boolean[] array, int start, int end)`
    pub fn sub_8(array: Vec<bool>, start: i32, end: i32) -> Result<Vec<bool>> {
        Err(CoreError::PendingEngine("sub"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::remove#long[] (long[] array, int index)`
    pub fn remove(array: Vec<i64>, index: i32) -> Result<Vec<i64>> {
        Err(CoreError::PendingEngine("remove"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::remove#int[] (int[] array, int index)`
    pub fn remove_2(array: Vec<i32>, index: i32) -> Result<Vec<i32>> {
        Err(CoreError::PendingEngine("remove"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::remove#short[] (short[] array, int index)`
    pub fn remove_3(array: Vec<i16>, index: i32) -> Result<Vec<i16>> {
        Err(CoreError::PendingEngine("remove"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::remove#char[] (char[] array, int index)`
    pub fn remove_4(array: Vec<char>, index: i32) -> Result<Vec<char>> {
        Err(CoreError::PendingEngine("remove"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::remove#byte[] (byte[] array, int index)`
    pub fn remove_5(array: Vec<i8>, index: i32) -> Result<Vec<i8>> {
        Err(CoreError::PendingEngine("remove"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::remove#double[] (double[] array, int index)`
    pub fn remove_6(array: Vec<f64>, index: i32) -> Result<Vec<f64>> {
        Err(CoreError::PendingEngine("remove"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::remove#float[] (float[] array, int index)`
    pub fn remove_7(array: Vec<f32>, index: i32) -> Result<Vec<f32>> {
        Err(CoreError::PendingEngine("remove"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::remove#boolean[] (boolean[] array, int index)`
    pub fn remove_8(array: Vec<bool>, index: i32) -> Result<Vec<bool>> {
        Err(CoreError::PendingEngine("remove"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::remove#Object (Object array, int index)`
    pub fn remove_9(_array: *const (), index: i32) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("remove"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::removeEle#long[] (long[] array, long element)`
    pub fn removeEle(array: Vec<i64>, element: i64) -> Result<Vec<i64>> {
        Err(CoreError::PendingEngine("removeEle"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::removeEle#int[] (int[] array, int element)`
    pub fn removeEle_2(array: Vec<i32>, element: i32) -> Result<Vec<i32>> {
        Err(CoreError::PendingEngine("removeEle"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::removeEle#short[] (short[] array, short element)`
    pub fn removeEle_3(array: Vec<i16>, element: i16) -> Result<Vec<i16>> {
        Err(CoreError::PendingEngine("removeEle"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::removeEle#char[] (char[] array, char element)`
    pub fn removeEle_4(array: Vec<char>, element: char) -> Result<Vec<char>> {
        Err(CoreError::PendingEngine("removeEle"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::removeEle#byte[] (byte[] array, byte element)`
    pub fn removeEle_5(array: Vec<i8>, element: i8) -> Result<Vec<i8>> {
        Err(CoreError::PendingEngine("removeEle"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::removeEle#double[] (double[] array, double element)`
    pub fn removeEle_6(array: Vec<f64>, element: f64) -> Result<Vec<f64>> {
        Err(CoreError::PendingEngine("removeEle"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::removeEle#float[] (float[] array, float element)`
    pub fn removeEle_7(array: Vec<f32>, element: f32) -> Result<Vec<f32>> {
        Err(CoreError::PendingEngine("removeEle"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::removeEle#boolean[] (boolean[] array, boolean element)`
    pub fn removeEle_8(array: Vec<bool>, element: bool) -> Result<Vec<bool>> {
        Err(CoreError::PendingEngine("removeEle"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::reverse#long[] (long[] array, final int startIndexInclusive, final int endIndexExclusive)`
    pub fn reverse(array: Vec<i64>, startIndexInclusive: i32, endIndexExclusive: i32) -> Result<Vec<i64>> {
        Err(CoreError::PendingEngine("reverse"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::reverse#long[] (long[] array)`
    pub fn reverse_2(array: Vec<i64>) -> Result<Vec<i64>> {
        Err(CoreError::PendingEngine("reverse"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::reverse#int[] (int[] array, final int startIndexInclusive, final int endIndexExclusive)`
    pub fn reverse_3(array: Vec<i32>, startIndexInclusive: i32, endIndexExclusive: i32) -> Result<Vec<i32>> {
        Err(CoreError::PendingEngine("reverse"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::reverse#int[] (int[] array)`
    pub fn reverse_4(array: Vec<i32>) -> Result<Vec<i32>> {
        Err(CoreError::PendingEngine("reverse"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::reverse#short[] (short[] array, final int startIndexInclusive, final int endIndexExclusive)`
    pub fn reverse_5(array: Vec<i16>, startIndexInclusive: i32, endIndexExclusive: i32) -> Result<Vec<i16>> {
        Err(CoreError::PendingEngine("reverse"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::reverse#short[] (short[] array)`
    pub fn reverse_6(array: Vec<i16>) -> Result<Vec<i16>> {
        Err(CoreError::PendingEngine("reverse"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::reverse#char[] (char[] array, final int startIndexInclusive, final int endIndexExclusive)`
    pub fn reverse_7(array: Vec<char>, startIndexInclusive: i32, endIndexExclusive: i32) -> Result<Vec<char>> {
        Err(CoreError::PendingEngine("reverse"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::reverse#char[] (char[] array)`
    pub fn reverse_8(array: Vec<char>) -> Result<Vec<char>> {
        Err(CoreError::PendingEngine("reverse"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::reverse#byte[] (byte[] array, final int startIndexInclusive, final int endIndexExclusive)`
    pub fn reverse_9(array: Vec<i8>, startIndexInclusive: i32, endIndexExclusive: i32) -> Result<Vec<i8>> {
        Err(CoreError::PendingEngine("reverse"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::reverse#byte[] (byte[] array)`
    pub fn reverse_10(array: Vec<i8>) -> Result<Vec<i8>> {
        Err(CoreError::PendingEngine("reverse"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::reverse#double[] (double[] array, final int startIndexInclusive, final int endIndexExclusive)`
    pub fn reverse_11(array: Vec<f64>, startIndexInclusive: i32, endIndexExclusive: i32) -> Result<Vec<f64>> {
        Err(CoreError::PendingEngine("reverse"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::reverse#double[] (double[] array)`
    pub fn reverse_12(array: Vec<f64>) -> Result<Vec<f64>> {
        Err(CoreError::PendingEngine("reverse"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::reverse#float[] (float[] array, final int startIndexInclusive, final int endIndexExclusive)`
    pub fn reverse_13(array: Vec<f32>, startIndexInclusive: i32, endIndexExclusive: i32) -> Result<Vec<f32>> {
        Err(CoreError::PendingEngine("reverse"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::reverse#float[] (float[] array)`
    pub fn reverse_14(array: Vec<f32>) -> Result<Vec<f32>> {
        Err(CoreError::PendingEngine("reverse"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::reverse#boolean[] (boolean[] array, final int startIndexInclusive, final int endIndexExclusive)`
    pub fn reverse_15(array: Vec<bool>, startIndexInclusive: i32, endIndexExclusive: i32) -> Result<Vec<bool>> {
        Err(CoreError::PendingEngine("reverse"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::reverse#boolean[] (boolean[] array)`
    pub fn reverse_16(array: Vec<bool>) -> Result<Vec<bool>> {
        Err(CoreError::PendingEngine("reverse"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::min#long (long... numberArray)`
    pub fn min(numberArray: &[i64]) -> Result<i64> {
        Err(CoreError::PendingEngine("min"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::min#int (int... numberArray)`
    pub fn min_2(numberArray: &[i32]) -> Result<i32> {
        Err(CoreError::PendingEngine("min"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::min#short (short... numberArray)`
    pub fn min_3(numberArray: &[i16]) -> Result<i16> {
        Err(CoreError::PendingEngine("min"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::min#char (char... numberArray)`
    pub fn min_4(numberArray: &[char]) -> Result<char> {
        Err(CoreError::PendingEngine("min"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::min#byte (byte... numberArray)`
    pub fn min_5(numberArray: &[i8]) -> Result<i8> {
        Err(CoreError::PendingEngine("min"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::min#double (double... numberArray)`
    pub fn min_6(numberArray: &[f64]) -> Result<f64> {
        Err(CoreError::PendingEngine("min"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::min#float (float... numberArray)`
    pub fn min_7(numberArray: &[f32]) -> Result<f32> {
        Err(CoreError::PendingEngine("min"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::max#long (long... numberArray)`
    pub fn max(numberArray: &[i64]) -> Result<i64> {
        Err(CoreError::PendingEngine("max"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::max#int (int... numberArray)`
    pub fn max_2(numberArray: &[i32]) -> Result<i32> {
        Err(CoreError::PendingEngine("max"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::max#short (short... numberArray)`
    pub fn max_3(numberArray: &[i16]) -> Result<i16> {
        Err(CoreError::PendingEngine("max"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::max#char (char... numberArray)`
    pub fn max_4(numberArray: &[char]) -> Result<char> {
        Err(CoreError::PendingEngine("max"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::max#byte (byte... numberArray)`
    pub fn max_5(numberArray: &[i8]) -> Result<i8> {
        Err(CoreError::PendingEngine("max"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::max#double (double... numberArray)`
    pub fn max_6(numberArray: &[f64]) -> Result<f64> {
        Err(CoreError::PendingEngine("max"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::max#float (float... numberArray)`
    pub fn max_7(numberArray: &[f32]) -> Result<f32> {
        Err(CoreError::PendingEngine("max"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::shuffle#int[] (int[] array)`
    pub fn shuffle(array: Vec<i32>) -> Result<Vec<i32>> {
        Err(CoreError::PendingEngine("shuffle"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::shuffle#int[] (int[] array, Random random)`
    pub fn shuffle_2(array: Vec<i32>, _random: *const ()) -> Result<Vec<i32>> {
        Err(CoreError::PendingEngine("shuffle"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::shuffle#long[] (long[] array)`
    pub fn shuffle_3(array: Vec<i64>) -> Result<Vec<i64>> {
        Err(CoreError::PendingEngine("shuffle"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::shuffle#long[] (long[] array, Random random)`
    pub fn shuffle_4(array: Vec<i64>, _random: *const ()) -> Result<Vec<i64>> {
        Err(CoreError::PendingEngine("shuffle"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::shuffle#double[] (double[] array)`
    pub fn shuffle_5(array: Vec<f64>) -> Result<Vec<f64>> {
        Err(CoreError::PendingEngine("shuffle"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::shuffle#double[] (double[] array, Random random)`
    pub fn shuffle_6(array: Vec<f64>, _random: *const ()) -> Result<Vec<f64>> {
        Err(CoreError::PendingEngine("shuffle"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::shuffle#float[] (float[] array)`
    pub fn shuffle_7(array: Vec<f32>) -> Result<Vec<f32>> {
        Err(CoreError::PendingEngine("shuffle"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::shuffle#float[] (float[] array, Random random)`
    pub fn shuffle_8(array: Vec<f32>, _random: *const ()) -> Result<Vec<f32>> {
        Err(CoreError::PendingEngine("shuffle"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::shuffle#boolean[] (boolean[] array)`
    pub fn shuffle_9(array: Vec<bool>) -> Result<Vec<bool>> {
        Err(CoreError::PendingEngine("shuffle"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::shuffle#boolean[] (boolean[] array, Random random)`
    pub fn shuffle_10(array: Vec<bool>, _random: *const ()) -> Result<Vec<bool>> {
        Err(CoreError::PendingEngine("shuffle"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::shuffle#byte[] (byte[] array)`
    pub fn shuffle_11(array: Vec<i8>) -> Result<Vec<i8>> {
        Err(CoreError::PendingEngine("shuffle"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::shuffle#byte[] (byte[] array, Random random)`
    pub fn shuffle_12(array: Vec<i8>, _random: *const ()) -> Result<Vec<i8>> {
        Err(CoreError::PendingEngine("shuffle"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::shuffle#char[] (char[] array)`
    pub fn shuffle_13(array: Vec<char>) -> Result<Vec<char>> {
        Err(CoreError::PendingEngine("shuffle"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::shuffle#char[] (char[] array, Random random)`
    pub fn shuffle_14(array: Vec<char>, _random: *const ()) -> Result<Vec<char>> {
        Err(CoreError::PendingEngine("shuffle"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::shuffle#short[] (short[] array)`
    pub fn shuffle_15(array: Vec<i16>) -> Result<Vec<i16>> {
        Err(CoreError::PendingEngine("shuffle"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::shuffle#short[] (short[] array, Random random)`
    pub fn shuffle_16(array: Vec<i16>, _random: *const ()) -> Result<Vec<i16>> {
        Err(CoreError::PendingEngine("shuffle"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::swap#int[] (int[] array, int index1, int index2)`
    pub fn swap(array: Vec<i32>, index1: i32, index2: i32) -> Result<Vec<i32>> {
        Err(CoreError::PendingEngine("swap"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::swap#long[] (long[] array, int index1, int index2)`
    pub fn swap_2(array: Vec<i64>, index1: i32, index2: i32) -> Result<Vec<i64>> {
        Err(CoreError::PendingEngine("swap"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::swap#double[] (double[] array, int index1, int index2)`
    pub fn swap_3(array: Vec<f64>, index1: i32, index2: i32) -> Result<Vec<f64>> {
        Err(CoreError::PendingEngine("swap"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::swap#float[] (float[] array, int index1, int index2)`
    pub fn swap_4(array: Vec<f32>, index1: i32, index2: i32) -> Result<Vec<f32>> {
        Err(CoreError::PendingEngine("swap"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::swap#boolean[] (boolean[] array, int index1, int index2)`
    pub fn swap_5(array: Vec<bool>, index1: i32, index2: i32) -> Result<Vec<bool>> {
        Err(CoreError::PendingEngine("swap"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::swap#byte[] (byte[] array, int index1, int index2)`
    pub fn swap_6(array: Vec<i8>, index1: i32, index2: i32) -> Result<Vec<i8>> {
        Err(CoreError::PendingEngine("swap"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::swap#char[] (char[] array, int index1, int index2)`
    pub fn swap_7(array: Vec<char>, index1: i32, index2: i32) -> Result<Vec<char>> {
        Err(CoreError::PendingEngine("swap"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::swap#short[] (short[] array, int index1, int index2)`
    pub fn swap_8(array: Vec<i16>, index1: i32, index2: i32) -> Result<Vec<i16>> {
        Err(CoreError::PendingEngine("swap"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isSorted#boolean (byte[] array)`
    pub fn isSorted(array: Vec<i8>) -> Result<bool> {
        Err(CoreError::PendingEngine("isSorted"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isSortedASC#boolean (byte[] array)`
    pub fn isSortedASC(array: Vec<i8>) -> Result<bool> {
        Err(CoreError::PendingEngine("isSortedASC"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isSortedDESC#boolean (byte[] array)`
    pub fn isSortedDESC(array: Vec<i8>) -> Result<bool> {
        Err(CoreError::PendingEngine("isSortedDESC"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isSorted#boolean (short[] array)`
    pub fn isSorted_2(array: Vec<i16>) -> Result<bool> {
        Err(CoreError::PendingEngine("isSorted"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isSortedASC#boolean (short[] array)`
    pub fn isSortedASC_2(array: Vec<i16>) -> Result<bool> {
        Err(CoreError::PendingEngine("isSortedASC"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isSortedDESC#boolean (short[] array)`
    pub fn isSortedDESC_2(array: Vec<i16>) -> Result<bool> {
        Err(CoreError::PendingEngine("isSortedDESC"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isSorted#boolean (char[] array)`
    pub fn isSorted_3(array: Vec<char>) -> Result<bool> {
        Err(CoreError::PendingEngine("isSorted"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isSortedASC#boolean (char[] array)`
    pub fn isSortedASC_3(array: Vec<char>) -> Result<bool> {
        Err(CoreError::PendingEngine("isSortedASC"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isSortedDESC#boolean (char[] array)`
    pub fn isSortedDESC_3(array: Vec<char>) -> Result<bool> {
        Err(CoreError::PendingEngine("isSortedDESC"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isSorted#boolean (int[] array)`
    pub fn isSorted_4(array: Vec<i32>) -> Result<bool> {
        Err(CoreError::PendingEngine("isSorted"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isSortedASC#boolean (int[] array)`
    pub fn isSortedASC_4(array: Vec<i32>) -> Result<bool> {
        Err(CoreError::PendingEngine("isSortedASC"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isSortedDESC#boolean (int[] array)`
    pub fn isSortedDESC_4(array: Vec<i32>) -> Result<bool> {
        Err(CoreError::PendingEngine("isSortedDESC"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isSorted#boolean (long[] array)`
    pub fn isSorted_5(array: Vec<i64>) -> Result<bool> {
        Err(CoreError::PendingEngine("isSorted"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isSortedASC#boolean (long[] array)`
    pub fn isSortedASC_5(array: Vec<i64>) -> Result<bool> {
        Err(CoreError::PendingEngine("isSortedASC"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isSortedDESC#boolean (long[] array)`
    pub fn isSortedDESC_5(array: Vec<i64>) -> Result<bool> {
        Err(CoreError::PendingEngine("isSortedDESC"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isSorted#boolean (double[] array)`
    pub fn isSorted_6(array: Vec<f64>) -> Result<bool> {
        Err(CoreError::PendingEngine("isSorted"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isSortedASC#boolean (double[] array)`
    pub fn isSortedASC_6(array: Vec<f64>) -> Result<bool> {
        Err(CoreError::PendingEngine("isSortedASC"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isSortedDESC#boolean (double[] array)`
    pub fn isSortedDESC_6(array: Vec<f64>) -> Result<bool> {
        Err(CoreError::PendingEngine("isSortedDESC"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isSorted#boolean (float[] array)`
    pub fn isSorted_7(array: Vec<f32>) -> Result<bool> {
        Err(CoreError::PendingEngine("isSorted"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isSortedASC#boolean (float[] array)`
    pub fn isSortedASC_7(array: Vec<f32>) -> Result<bool> {
        Err(CoreError::PendingEngine("isSortedASC"))
    }

    /// 对齐 Java: `cn.hutool.core.util::PrimitiveArrayUtil::isSortedDESC#boolean (float[] array)`
    pub fn isSortedDESC_7(array: Vec<f32>) -> Result<bool> {
        Err(CoreError::PendingEngine("isSortedDESC"))
    }
}
