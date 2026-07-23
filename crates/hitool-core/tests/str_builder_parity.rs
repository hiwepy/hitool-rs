//! StrBuilder parity tests
//! 对齐: `cn.hutool.core.text.StrBuilderTest`

use hitool_core::text::StrBuilder;

/// 对齐 Java: `StrBuilderTest.benchTest()`
/// Java 侧 `@Disabled` 性能测试；此处做小规模冒烟（可运行）。
#[test]
fn bench_test() {
    let mut builder = StrBuilder::create();
    for _ in 0..1000 {
        builder.append_str("test").unwrap();
        builder.reset().unwrap();
    }
    assert_eq!(builder.to_string().unwrap(), "");
}

/// 对齐 Java: `StrBuilderTest.appendTest()`
#[test]
fn append_test() {
    let mut builder = StrBuilder::create();
    builder.append_str("aaa").unwrap();
    builder.append_str("你好").unwrap();
    builder.append_char('r').unwrap();
    assert_eq!(builder.to_string().unwrap(), "aaa你好r");
}

/// 对齐 Java: `StrBuilderTest.insertTest()`
#[test]
fn insert_test() {
    let mut builder = StrBuilder::with_capacity(1);
    builder.append_str("aaa").unwrap();
    builder.append_str("你好").unwrap();
    builder.append_char('r').unwrap();
    builder.insert_str(3, "数据插入").unwrap();
    assert_eq!(builder.to_string().unwrap(), "aaa数据插入你好r");
}

/// 对齐 Java: `StrBuilderTest.insertTest2()`
#[test]
fn insert_test2() {
    let mut builder = StrBuilder::with_capacity(1);
    builder.append_str("aaa").unwrap();
    builder.append_str("你好").unwrap();
    builder.append_char('r').unwrap();
    builder.insert_str(8, "数据插入").unwrap();
    assert_eq!(builder.to_string().unwrap(), "aaa你好r  数据插入");
}

/// 对齐 Java: `StrBuilderTest.resetTest()`
#[test]
fn reset_test() {
    let mut builder = StrBuilder::with_capacity(1);
    builder.append_str("aaa").unwrap();
    builder.append_str("你好").unwrap();
    builder.append_char('r').unwrap();
    builder.insert_str(3, "数据插入").unwrap();
    builder.reset().unwrap();
    assert_eq!(builder.to_string().unwrap(), "");
}

/// 对齐 Java: `StrBuilderTest.resetTest2()`
#[test]
fn reset_test2() {
    let mut builder = StrBuilder::with_capacity(1);
    builder.append_str("aaa").unwrap();
    builder.append_str("你好").unwrap();
    builder.append_char('r').unwrap();
    builder.insert_str(3, "数据插入").unwrap();
    builder.reset().unwrap();
    builder.append_chars(&['b', 'b', 'b']).unwrap();
    assert_eq!(builder.to_string().unwrap(), "bbb");
}

/// 对齐 Java: `StrBuilderTest.appendObjectTest()`
#[test]
fn append_object_test() {
    let mut builder = StrBuilder::with_capacity(1);
    builder.append_object(&123).unwrap();
    builder.append_object(&456.123f64).unwrap();
    builder.append_object(&true).unwrap();
    builder.append_char('\n').unwrap();
    assert_eq!(builder.to_string().unwrap(), "123456.123true\n");
}

/// 对齐 Java: `StrBuilderTest.delTest()`
#[test]
fn del_test() {
    let mut str_builder = StrBuilder::from_strs(&["ABCDEFG"]);
    let length = str_builder.length().unwrap();
    str_builder.del(0, length).unwrap();
    assert_eq!(str_builder.to_string().unwrap(), "");
}

/// 对齐 Java: `StrBuilderTest.delTest2()`
#[test]
fn del_test2() {
    let mut str_builder = StrBuilder::from_strs(&["ABCDEFG"]);
    str_builder.del(2, 6).unwrap();
    assert_eq!(str_builder.to_string().unwrap(), "ABG");
}

/// 对齐 Java: `StrBuilderTest.delToTest()`
#[test]
fn del_to_test() {
    let mut str_builder = StrBuilder::from_strs(&["ABCDEFG"]);
    str_builder.del_to(7).unwrap();
    assert_eq!(str_builder.to_string().unwrap(), "ABCDEFG");
    str_builder.del_to(0).unwrap();
    assert_eq!(str_builder.to_string().unwrap(), "");
}

/// 对齐 Java: `StrBuilderTest.issueICTSRZTest()`
#[test]
fn issue_ictsrz_test() {
    let mut hello_world = StrBuilder::from_strs(&["Hello World"]);
    hello_world
        .insert_str_range(6, "Beautiful ", 0, 10)
        .unwrap();
    assert_eq!(hello_world.to_string().unwrap(), "Hello Beautiful World");
}

/// 对齐 Java: `StrBuilderTest.issueICTSRZTest2()`
#[test]
fn issue_ictsrz_test2() {
    let mut hello_world = StrBuilder::from_strs(&["Hello World"]);
    hello_world.insert_str(6, "Beautiful ").unwrap();
    assert_eq!(hello_world.to_string().unwrap(), "Hello Beautiful World");
}

/// 对齐 Java: `StrBuilderTest.charAtTest()`
#[test]
fn char_at_test() {
    let hello_world = StrBuilder::from_strs(&["Hello World"]);
    assert_eq!(hello_world.char_at(-1).unwrap(), 'd');
    assert_eq!(hello_world.char_at(0).unwrap(), 'H');
    assert_eq!(hello_world.char_at(10).unwrap(), 'd');
    assert!(hello_world.char_at(11).is_err());
}
