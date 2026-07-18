//! 对齐: `cn.hutool.core.util.ArrayUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ArrayUtil.java
//!
//! Rust 版本按 idiomatic 风格对每个公开方法提供关联函数桩;具体实现
//! 等待各 `hitool-rs` 引擎完成后回填,所有桩在调用时统一返回
//! `CoreError::PendingEngine`。
//!
//! 重载的 Java 方法通过 `<name>_<n>` 后缀区分,避免 Rust 关联函数重名冲突。

#![allow(dead_code, unused_variables, clippy::too_many_arguments, non_snake_case)]

use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.util.ArrayUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct ArrayUtil;

impl ArrayUtil {
    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::isEmpty#boolean (T[] array)`
    pub fn isEmpty(array: Vec<T>) -> Result<bool> {
        Err(CoreError::PendingEngine("isEmpty"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::defaultIfEmpty#T[] (T[] array, T[] defaultArray)`
    pub fn defaultIfEmpty(array: Vec<T>, defaultArray: Vec<T>) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("defaultIfEmpty"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::isEmpty#boolean (Object array)`
    pub fn isEmpty_2(_array: *const ()) -> Result<bool> {
        Err(CoreError::PendingEngine("isEmpty"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::isNotEmpty#boolean (T[] array)`
    pub fn isNotEmpty(array: Vec<T>) -> Result<bool> {
        Err(CoreError::PendingEngine("isNotEmpty"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::isNotEmpty#boolean (Object array)`
    pub fn isNotEmpty_2(_array: *const ()) -> Result<bool> {
        Err(CoreError::PendingEngine("isNotEmpty"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::hasNull#boolean (T... array)`
    pub fn hasNull(array: &[T]) -> Result<bool> {
        Err(CoreError::PendingEngine("hasNull"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::isAllNull#boolean (T... array)`
    pub fn isAllNull(array: &[T]) -> Result<bool> {
        Err(CoreError::PendingEngine("isAllNull"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::firstNonNull#T (T... array)`
    pub fn firstNonNull(array: &[T]) -> Result<T> {
        Err(CoreError::PendingEngine("firstNonNull"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::firstMatch#T (Matcher<T> matcher, T... array)`
    pub fn firstMatch(matcher: Matcher, array: &[T]) -> Result<T> {
        Err(CoreError::PendingEngine("firstMatch"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::matchIndex#int (Matcher<T> matcher, T... array)`
    pub fn matchIndex(matcher: Matcher, array: &[T]) -> Result<i32> {
        Err(CoreError::PendingEngine("matchIndex"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::matchIndex#int (Matcher<T> matcher, int beginIndexInclude, T... array)`
    pub fn matchIndex_2(matcher: Matcher, beginIndexInclude: i32, array: &[T]) -> Result<i32> {
        Err(CoreError::PendingEngine("matchIndex"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::newArray#T[] (Class<?> componentType, int newSize)`
    pub fn newArray(componentType: Class, newSize: i32) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("newArray"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::newArray#Object[] (int newSize)`
    pub fn newArray_2(newSize: i32) -> Result<Vec<OPAQUE>> {
        Err(CoreError::PendingEngine("newArray"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::getComponentType#Class<?> (Object array)`
    pub fn getComponentType(_array: *const ()) -> Result<()> {
        Err(CoreError::PendingEngine("getComponentType"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::getComponentType#Class<?> (Class<?> arrayClass)`
    pub fn getComponentType_2(arrayClass: Class) -> Result<()> {
        Err(CoreError::PendingEngine("getComponentType"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::getArrayType#Class<?> (Class<?> componentType)`
    pub fn getArrayType(componentType: Class) -> Result<()> {
        Err(CoreError::PendingEngine("getArrayType"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::cast#Object[] (Class<?> type, Object arrayObj)`
    pub fn cast(type: Class, _arrayObj: *const ()) -> Result<Vec<OPAQUE>> {
        Err(CoreError::PendingEngine("cast"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::append#T[] (T[] buffer, T... newElements)`
    pub fn append(buffer: Vec<T>, newElements: &[T]) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("append"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::append#Object (Object array, T... newElements)`
    pub fn append_2(_array: *const (), newElements: &[T]) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("append"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::setOrAppend#T[] (T[] buffer, int index, T value)`
    pub fn setOrAppend(buffer: Vec<T>, index: i32, value: T) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("setOrAppend"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::setOrAppend#Object (Object array, int index, Object value)`
    pub fn setOrAppend_2(_array: *const (), index: i32, _value: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("setOrAppend"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::replace#T[] (T[] buffer, int index, T... values)`
    pub fn replace(buffer: Vec<T>, index: i32, values: &[T]) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("replace"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::insert#T[] (T[] buffer, int index, T... newElements)`
    pub fn insert(buffer: Vec<T>, index: i32, newElements: &[T]) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("insert"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::insert#Object (Object array, int index, T... newElements)`
    pub fn insert_2(_array: *const (), index: i32, newElements: &[T]) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("insert"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::resize#T[] (T[] data, int newSize, Class<?> componentType)`
    pub fn resize(data: Vec<T>, newSize: i32, componentType: Class) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("resize"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::resize#Object (Object array, int newSize)`
    pub fn resize_2(_array: *const (), newSize: i32) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("resize"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::resize#T[] (T[] buffer, int newSize)`
    pub fn resize_3(buffer: Vec<T>, newSize: i32) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("resize"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::addAll#T[] (T[]... arrays)`
    pub fn addAll(arrays: &[Vec<T>]) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("addAll"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::copy#Object (Object src, int srcPos, Object dest, int destPos, int length)`
    pub fn copy(_src: *const (), srcPos: i32, _dest: *const (), destPos: i32, length: i32) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("copy"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::copy#Object (Object src, Object dest, int length)`
    pub fn copy_2(_src: *const (), _dest: *const (), length: i32) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("copy"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::clone#T[] (T[] array)`
    pub fn clone(array: Vec<T>) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("clone"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::clone#T (final T obj)`
    pub fn clone_2(obj: T) -> Result<T> {
        Err(CoreError::PendingEngine("clone"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::edit#T[] (T[] array, Editor<T> editor)`
    pub fn edit(array: Vec<T>, editor: Editor) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("edit"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::filter#T[] (T[] array, Filter<T> filter)`
    pub fn filter(array: Vec<T>, filter: Filter) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("filter"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::removeNull#T[] (T[] array)`
    pub fn removeNull(array: Vec<T>) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("removeNull"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::removeEmpty#T[] (T[] array)`
    pub fn removeEmpty(array: Vec<T>) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("removeEmpty"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::removeBlank#T[] (T[] array)`
    pub fn removeBlank(array: Vec<T>) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("removeBlank"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::nullToEmpty#String[] (String[] array)`
    pub fn nullToEmpty(array: Vec<OPAQUE>) -> Result<Vec<OPAQUE>> {
        Err(CoreError::PendingEngine("nullToEmpty"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::zip#Map<K, V> (K[] keys, V[] values, boolean isOrder)`
    pub fn zip(keys: Vec<T>, values: Vec<T>, isOrder: bool) -> Result<std::collections::HashMap<T, T>> {
        Err(CoreError::PendingEngine("zip"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::zip#Map<K, V> (K[] keys, V[] values)`
    pub fn zip_2(keys: Vec<T>, values: Vec<T>) -> Result<std::collections::HashMap<T, T>> {
        Err(CoreError::PendingEngine("zip"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::indexOf#int (T[] array, Object value, int beginIndexInclude)`
    pub fn indexOf(array: Vec<T>, _value: *const (), beginIndexInclude: i32) -> Result<i32> {
        Err(CoreError::PendingEngine("indexOf"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::indexOf#int (T[] array, Object value)`
    pub fn indexOf_2(array: Vec<T>, _value: *const ()) -> Result<i32> {
        Err(CoreError::PendingEngine("indexOf"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::indexOfIgnoreCase#int (CharSequence[] array, CharSequence value)`
    pub fn indexOfIgnoreCase(array: Vec<OPAQUE>, _value: *const ()) -> Result<i32> {
        Err(CoreError::PendingEngine("indexOfIgnoreCase"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::lastIndexOf#int (T[] array, Object value)`
    pub fn lastIndexOf(array: Vec<T>, _value: *const ()) -> Result<i32> {
        Err(CoreError::PendingEngine("lastIndexOf"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::lastIndexOf#int (T[] array, Object value, int endInclude)`
    pub fn lastIndexOf_2(array: Vec<T>, _value: *const (), endInclude: i32) -> Result<i32> {
        Err(CoreError::PendingEngine("lastIndexOf"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::contains#boolean (T[] array, T value)`
    pub fn contains(array: Vec<T>, value: T) -> Result<bool> {
        Err(CoreError::PendingEngine("contains"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::containsAny#boolean (T[] array, T... values)`
    pub fn containsAny(array: Vec<T>, values: &[T]) -> Result<bool> {
        Err(CoreError::PendingEngine("containsAny"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::containsAll#boolean (T[] array, T... values)`
    pub fn containsAll(array: Vec<T>, values: &[T]) -> Result<bool> {
        Err(CoreError::PendingEngine("containsAll"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::containsIgnoreCase#boolean (CharSequence[] array, CharSequence value)`
    pub fn containsIgnoreCase(array: Vec<OPAQUE>, _value: *const ()) -> Result<bool> {
        Err(CoreError::PendingEngine("containsIgnoreCase"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::wrap#Object[] (Object obj)`
    pub fn wrap(_obj: *const ()) -> Result<Vec<OPAQUE>> {
        Err(CoreError::PendingEngine("wrap"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::isArray#boolean (Object obj)`
    pub fn isArray(_obj: *const ()) -> Result<bool> {
        Err(CoreError::PendingEngine("isArray"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::get#T (Object array, int index)`
    pub fn get(_array: *const (), index: i32) -> Result<T> {
        Err(CoreError::PendingEngine("get"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::getAny#T[] (Object array, int... indexes)`
    pub fn getAny(_array: *const (), indexes: &[i32]) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("getAny"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::sub#T[] (T[] array, int start, int end)`
    pub fn sub(array: Vec<T>, start: i32, end: i32) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("sub"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::sub#Object[] (Object array, int start, int end)`
    pub fn sub_2(_array: *const (), start: i32, end: i32) -> Result<Vec<OPAQUE>> {
        Err(CoreError::PendingEngine("sub"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::sub#Object[] (Object array, int start, int end, int step)`
    pub fn sub_3(_array: *const (), start: i32, end: i32, step: i32) -> Result<Vec<OPAQUE>> {
        Err(CoreError::PendingEngine("sub"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::toString#String (Object obj)`
    pub fn toString(_obj: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("toString"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::length#int (Object array)`
    pub fn length(_array: *const ()) -> Result<i32> {
        Err(CoreError::PendingEngine("length"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::join#String (T[] array, CharSequence conjunction)`
    pub fn join(array: Vec<T>, _conjunction: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("join"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::join#String (T[] array, CharSequence delimiter, String prefix, String suffix)`
    pub fn join_2(array: Vec<T>, _delimiter: *const (), _prefix: *const (), _suffix: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("join"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::join#String (T[] array, CharSequence conjunction, Editor<T> editor)`
    pub fn join_3(array: Vec<T>, _conjunction: *const (), editor: Editor) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("join"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::join#String (Object array, CharSequence conjunction)`
    pub fn join_4(_array: *const (), _conjunction: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("join"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::toArray#byte[] (ByteBuffer bytebuffer)`
    pub fn toArray(_bytebuffer: *const ()) -> Result<Vec<i8>> {
        Err(CoreError::PendingEngine("toArray"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::toArray#T[] (Iterator<T> iterator, Class<T> componentType)`
    pub fn toArray_2(iterator: Iterator<Item = T>, componentType: Class) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("toArray"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::toArray#T[] (Iterable<T> iterable, Class<T> componentType)`
    pub fn toArray_3(iterable: Vec<T>, componentType: Class) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("toArray"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::toArray#T[] (Collection<T> collection, Class<T> componentType)`
    pub fn toArray_4(collection: Vec<T>, componentType: Class) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("toArray"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::remove#T[] (T[] array, int index)`
    pub fn remove(array: Vec<T>, index: i32) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("remove"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::removeEle#T[] (T[] array, T element)`
    pub fn removeEle(array: Vec<T>, element: T) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("removeEle"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::reverse#T[] (T[] array, final int startIndexInclusive, final int endIndexExclusive)`
    pub fn reverse(array: Vec<T>, startIndexInclusive: i32, endIndexExclusive: i32) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("reverse"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::reverse#T[] (T[] array)`
    pub fn reverse_2(array: Vec<T>) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("reverse"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::min#T (T[] numberArray)`
    pub fn min(numberArray: Vec<T>) -> Result<T> {
        Err(CoreError::PendingEngine("min"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::min#T (T[] numberArray, Comparator<T> comparator)`
    pub fn min_2(numberArray: Vec<T>, comparator: Comparator) -> Result<T> {
        Err(CoreError::PendingEngine("min"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::max#T (T[] numberArray)`
    pub fn max(numberArray: Vec<T>) -> Result<T> {
        Err(CoreError::PendingEngine("max"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::max#T (T[] numberArray, Comparator<T> comparator)`
    pub fn max_2(numberArray: Vec<T>, comparator: Comparator) -> Result<T> {
        Err(CoreError::PendingEngine("max"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::shuffle#T[] (T[] array)`
    pub fn shuffle(array: Vec<T>) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("shuffle"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::shuffle#T[] (T[] array, Random random)`
    pub fn shuffle_2(array: Vec<T>, _random: *const ()) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("shuffle"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::swap#T[] (T[] array, int index1, int index2)`
    pub fn swap(array: Vec<T>, index1: i32, index2: i32) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("swap"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::swap#Object (Object array, int index1, int index2)`
    pub fn swap_2(_array: *const (), index1: i32, index2: i32) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("swap"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::emptyCount#int (Object... args)`
    pub fn emptyCount(args: &[OPAQUE]) -> Result<i32> {
        Err(CoreError::PendingEngine("emptyCount"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::hasEmpty#boolean (Object... args)`
    pub fn hasEmpty(args: &[OPAQUE]) -> Result<bool> {
        Err(CoreError::PendingEngine("hasEmpty"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::isAllEmpty#boolean (Object... args)`
    pub fn isAllEmpty(args: &[OPAQUE]) -> Result<bool> {
        Err(CoreError::PendingEngine("isAllEmpty"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::isAllNotEmpty#boolean (Object... args)`
    pub fn isAllNotEmpty(args: &[OPAQUE]) -> Result<bool> {
        Err(CoreError::PendingEngine("isAllNotEmpty"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::isAllNotNull#boolean (T... array)`
    pub fn isAllNotNull(array: &[T]) -> Result<bool> {
        Err(CoreError::PendingEngine("isAllNotNull"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::distinct#T[] (T[] array)`
    pub fn distinct(array: Vec<T>) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("distinct"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::distinct#T[] (T[] array, Function<T, K> uniqueGenerator, boolean override)`
    pub fn distinct_2(array: Vec<T>, uniqueGenerator: fn(T) -> T, override: bool) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("distinct"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::map#R[] (T[] array, Class<R> targetComponentType, Function<? super T, ? extends R> func)`
    pub fn map(array: Vec<T>, targetComponentType: Class, func: fn(OPAQUE) -> OPAQUE) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("map"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::map#R[] (Object array, Class<R> targetComponentType, Function<? super T, ? extends R> func)`
    pub fn map_2(_array: *const (), targetComponentType: Class, func: fn(OPAQUE) -> OPAQUE) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("map"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::map#List<R> (T[] array, Function<? super T, ? extends R> func)`
    pub fn map_3(array: Vec<T>, func: fn(OPAQUE) -> OPAQUE) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("map"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::mapToSet#Set<R> (T[] array, Function<? super T, ? extends R> func)`
    pub fn mapToSet(array: Vec<T>, func: fn(OPAQUE) -> OPAQUE) -> Result<std::collections::HashSet<T>> {
        Err(CoreError::PendingEngine("mapToSet"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::equals#boolean (Object array1, Object array2)`
    pub fn equals(_array1: *const (), _array2: *const ()) -> Result<bool> {
        Err(CoreError::PendingEngine("equals"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::isSub#boolean (T[] array, T[] subArray)`
    pub fn isSub(array: Vec<T>, subArray: Vec<T>) -> Result<bool> {
        Err(CoreError::PendingEngine("isSub"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::indexOfSub#int (T[] array, T[] subArray)`
    pub fn indexOfSub(array: Vec<T>, subArray: Vec<T>) -> Result<i32> {
        Err(CoreError::PendingEngine("indexOfSub"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::indexOfSub#int (T[] array, int beginInclude, T[] subArray)`
    pub fn indexOfSub_2(array: Vec<T>, beginInclude: i32, subArray: Vec<T>) -> Result<i32> {
        Err(CoreError::PendingEngine("indexOfSub"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::lastIndexOfSub#int (T[] array, T[] subArray)`
    pub fn lastIndexOfSub(array: Vec<T>, subArray: Vec<T>) -> Result<i32> {
        Err(CoreError::PendingEngine("lastIndexOfSub"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::lastIndexOfSub#int (T[] array, int endInclude, T[] subArray)`
    pub fn lastIndexOfSub_2(array: Vec<T>, endInclude: i32, subArray: Vec<T>) -> Result<i32> {
        Err(CoreError::PendingEngine("lastIndexOfSub"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::isSorted#boolean (T[] array, Comparator<? super T> comparator)`
    pub fn isSorted(array: Vec<T>, comparator: Comparator) -> Result<bool> {
        Err(CoreError::PendingEngine("isSorted"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::isSorted#boolean (T[] array)`
    pub fn isSorted_2(array: Vec<T>) -> Result<bool> {
        Err(CoreError::PendingEngine("isSorted"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::isSortedASC#boolean (T[] array)`
    pub fn isSortedASC(array: Vec<T>) -> Result<bool> {
        Err(CoreError::PendingEngine("isSortedASC"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::isSortedDESC#boolean (T[] array)`
    pub fn isSortedDESC(array: Vec<T>) -> Result<bool> {
        Err(CoreError::PendingEngine("isSortedDESC"))
    }
}
