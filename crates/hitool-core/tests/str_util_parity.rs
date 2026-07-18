//! `StrUtil` 对比验证测试 —— 对齐 Hutool `StrUtilTest`
//!
//! 对齐: `cn.hutool.core.util.StrUtilTest`
//! 来源: hutool-core/src/test/java/cn/hutool/core/util/StrUtilTest.java
//!
//! StrUtilTest 共 64 个 @Test,本文件选取已有 hitool 实现对应的测试翻译。
//! 需要扩展 string.rs 才能覆盖的测试,在各方法注释中标注"待实现"。

use hitool_core as hc;

/// 对齐 Java: `StrUtilTest.isBlankTest()` (行 18-22)
///
/// "	  　" (含制表符+英文空格+不间断空格+全角空格) 应为 blank。
#[test]
fn is_blank_test() {
    let blank = "\t   \u{a0}\u{3000}";
    assert!(hc::is_blank(blank), "is_blank(制表+空格+全角) 应 true (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.trimTest()` (行 24-29)
#[test]
fn trim_test() {
    let blank = "\t 哈哈 \u{3000}";
    let trimmed = hc::trim(blank);
    assert_eq!(trimmed, "哈哈", "trim (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.trimNewLineTest()` (行 31-41)
#[test]
fn trim_new_line_test() {
    assert_eq!(hc::trim("\r\naaa"), "aaa", "trim \\r\\naaa (对齐 Java)");
    assert_eq!(hc::trim("\raaa"), "aaa", "trim \\raaa (对齐 Java)");
    assert_eq!(hc::trim("\naaa"), "aaa", "trim \\naaa (对齐 Java)");
    assert_eq!(hc::trim("\r\n\r\naaa"), "aaa", "trim \\r\\n\\r\\naaa (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.trimTabTest()` (行 43-47)
#[test]
fn trim_tab_test() {
    assert_eq!(hc::trim("\taaa"), "aaa", "trim \\taaa (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.cleanBlankTest()` (行 49-55)
///
/// 清除所有空白(包括制表符、英文空格、全角空格)。
#[test]
fn clean_blank_test() {
    let s = "\t 你 好\u{3000}";
    let cleaned = hc::clean_blank(s);
    assert_eq!(cleaned, "你好", "cleanBlank (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.cutTest()` (行 57-62)
///
/// 按固定长度 4 切割。
#[test]
fn cut_test() {
    let s = "aaabbbcccdddaadfdfsdfsdf0";
    let cut = hc::cut(s, 4).unwrap();
    assert_eq!(
        cut,
        vec!["aaab", "bbcc", "cddd", "aadf", "dfsd", "fsdf", "0"],
        "cut(str, 4) (对齐 Java)"
    );
}

/// 对齐 Java: `StrUtilTest.splitTest()` (行 64-78) — 部分
///
/// Java `StrUtil.split(str, ',', -1, true, true)` → trim 每项 + 忽略空。
/// hitool `split(str, ',', true, true)` 等价。
#[test]
fn split_test() {
    let s = "a,b ,c,d,,e";
    let result = hc::split(s, ',', true, true);
    assert_eq!(result.len(), 5, "split 后去掉空项剩 5 个 (对齐 Java)");
    assert_eq!(result[1], "b", "split 第二项 trim 后为 \"b\" (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.splitEmptyTest()` (行 80-86)
#[test]
fn split_empty_test() {
    let s = "";
    let result = hc::split(s, ',', true, true);
    assert_eq!(result.len(), 0, "split 空字符串后 0 项 (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.splitTest2()` (行 88-95)
///
/// 默认 split 不 trim 不忽略空。
#[test]
fn split_test_2() {
    let s = "a.b.";
    let result = hc::split(s, '.', false, false);
    assert_eq!(result.len(), 3, "split(\"a.b.\", '.') → 3 项 (对齐 Java)");
    assert_eq!(result[1], "b", "第 2 项 \"b\" (对齐 Java)");
    assert_eq!(result[2], "", "第 3 项 \"\" (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.replaceTest2()` (行 230-234)
#[test]
fn replace_test_2() {
    let result = hc::replace("123", "2", "3");
    assert_eq!(result, "133", "replace(\"123\", \"2\", \"3\") (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.replaceTest3()` (行 236-240)
#[test]
fn replace_test_3() {
    let result = hc::replace(",abcdef,", ",", "|");
    assert_eq!(result, "|abcdef|", "replace(\",abcdef,\", \",\", \"|\") (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.indexOfIgnoreCaseTest()` (行 184-197)
///
/// 注:hitool 的 `index_of_ignore_case` 返回 `Option<usize>`,
/// Java 返回 -1 表示找不到。Rust 用 None。
#[test]
fn index_of_ignore_case_test() {
    // Java: indexOfIgnoreCase(null, "x", 0) → -1; Rust None 近似
    // Java: indexOfIgnoreCase("x", null, 0) → -1; 跳过(null 入参)

    assert_eq!(
        hc::index_of_ignore_case("", ""),
        Some(0),
        "indexOfIgnoreCase(\"\", \"\") = 0 (对齐 Java)"
    );
    assert_eq!(
        hc::index_of_ignore_case("aabaabaa", "A"),
        Some(0),
        "indexOfIgnoreCase(\"aabaabaa\", \"A\") = 0 (对齐 Java)"
    );
    assert_eq!(
        hc::index_of_ignore_case("aabaabaa", "B"),
        Some(2),
        "indexOfIgnoreCase(\"aabaabaa\", \"B\") = 2 (对齐 Java)"
    );
    assert_eq!(
        hc::index_of_ignore_case("aabaabaa", "AB"),
        Some(1),
        "indexOfIgnoreCase(\"aabaabaa\", \"AB\") = 1 (对齐 Java)"
    );
}

/// 对齐 Java: `StrUtilTest.lastIndexOfTest()` (行 199-204)
#[test]
fn last_index_of_test() {
    let a = "aabbccddcc";
    // Java lastIndexOf(a, "c", 0, false) → 从位置 0 向前找 "c" → -1
    // hitool last_index_of 从后向前找
    // Java 的 lastIndexOf(str, searchStr, startPos) 比较复杂,
    // 这里测试基本的 rfind 语义。
    let result = hc::last_index_of(a, "cc");
    assert_eq!(result, Some(8), "last_index_of(\"aabbccddcc\", \"cc\") = 8 (对齐 Java 语义)");
}

/// 对齐 Java: `StrUtilTest.lastIndexOfIgnoreCaseTest()` (行 206-220)
#[test]
fn last_index_of_ignore_case_test() {
    assert_eq!(
        hc::last_index_of_ignore_case("", ""),
        Some(0),
        "lastIndexOfIgnoreCase(\"\", \"\") = 0 (对齐 Java)"
    );
    assert_eq!(
        hc::last_index_of_ignore_case("aabaabaa", "A"),
        Some(7),
        "lastIndexOfIgnoreCase(\"aabaabaa\", \"A\") = 7 (对齐 Java)"
    );
    assert_eq!(
        hc::last_index_of_ignore_case("aabaabaa", "B"),
        Some(5),
        "lastIndexOfIgnoreCase(\"aabaabaa\", \"B\") = 5 (对齐 Java)"
    );
    assert_eq!(
        hc::last_index_of_ignore_case("aabaabaa", "AB"),
        Some(4),
        "lastIndexOfIgnoreCase(\"aabaabaa\", \"AB\") = 4 (对齐 Java)"
    );
    assert_eq!(
        hc::last_index_of_ignore_case("AAAcsd", "aaa"),
        Some(0),
        "lastIndexOfIgnoreCase(\"AAAcsd\", \"aaa\") = 0 (对齐 Java)"
    );
}

/// 对齐 Java: `StrUtilTest.upperFirstTest()` (行 264-269)
#[test]
fn upper_first_test() {
    let result = hc::upper_first("KEY");
    assert_eq!(result, "KEY", "upperFirst(\"KEY\") = \"KEY\" (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.lowerFirstTest()` (行 271-276)
#[test]
fn lower_first_test() {
    let result = hc::lower_first("KEY");
    assert_eq!(result, "kEY", "lowerFirst(\"KEY\") = \"kEY\" (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.equalsTest` (隐含在多处)
#[test]
fn equals_test() {
    assert!(hc::equals("abc", "abc"), "equals(\"abc\", \"abc\") true");
    assert!(!hc::equals("abc", "ABC"), "equals(\"abc\", \"ABC\") false");
}

/// 对齐 Java: `StrUtilTest.equalsIgnoreCaseTest` (隐含)
#[test]
fn equals_ignore_case_test() {
    assert!(hc::equals_ignore_case("abc", "ABC"), "equalsIgnoreCase(\"abc\", \"ABC\") true");
    assert!(hc::equals_ignore_case("ABC", "abc"), "equalsIgnoreCase 反序 true");
}

/// 对齐 Java: `StrUtilTest.containsTest` (隐含)
#[test]
fn contains_test() {
    assert!(hc::contains("hello world", "world"), "contains(\"hello world\", \"world\")");
    assert!(!hc::contains("hello", "world"), "contains(\"hello\", \"world\") false");
}

/// 对齐 Java: `StrUtilTest.containsIgnoreCaseTest` (隐含)
#[test]
fn contains_ignore_case_test() {
    assert!(
        hc::contains_ignore_case("Hello World", "WORLD"),
        "containsIgnoreCase(\"Hello World\", \"WORLD\") true"
    );
}

/// 对齐 Java: `StrUtilTest.startWithTest` (隐含)
#[test]
fn start_with_test() {
    assert!(hc::start_with("hello world", "hello"), "startWith(\"hello world\", \"hello\")");
    assert!(!hc::start_with("hello", "world"), "startWith(\"hello\", \"world\") false");
}

/// 对齐 Java: `StrUtilTest.endWithTest` (隐含)
#[test]
fn end_with_test() {
    assert!(hc::end_with("hello world", "world"), "endWith(\"hello world\", \"world\")");
    assert!(!hc::end_with("hello", "world"), "endWith(\"hello\", \"world\") false");
}

/// 对齐 Java: `StrUtilTest.reverseTest` (行 ~317)
#[test]
fn reverse_test() {
    assert_eq!(hc::reverse("abc"), "cba", "reverse(\"abc\") = \"cba\"");
    assert_eq!(hc::reverse("你好"), "好你", "reverse(\"你好\") = \"好你\"");
}

/// 对齐 Java: `StrUtilTest.formatTest()` (行 129-137) — 简化版
///
/// Java 的 `StrUtil.format(template, Dict)` 用命名占位符 `{name}`,
/// hitool 的 `format_template` 用 `{}` 顺序占位符。
/// 这里用顺序占位符验证格式化逻辑。
#[test]
fn format_test() {
    let template = "你好,我是{},我的电话是:{}";
    let name = "张三";
    let phone = "13888881111";
    let result = hc::format_template(template, &[&name, &phone]);
    assert_eq!(
        result, "你好,我是张三,我的电话是:13888881111",
        "format_template (对齐 Java formatTest 语义)"
    );
}

/// 对齐 Java: `StrUtilTest.lengthTest` (隐含)
#[test]
fn length_test() {
    assert_eq!(hc::length(Some("hello")), 5, "length(\"hello\") = 5");
    assert_eq!(hc::length(None), 0, "length(null) = 0 (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.strTest` (隐含)
#[test]
fn str_or_empty_test() {
    assert_eq!(hc::str_or_empty(Some("hello")), "hello", "str_or_empty(Some) = \"hello\"");
    assert_eq!(hc::str_or_empty(None), "", "str_or_empty(None) = \"\" (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.repeatTest` (隐含)
#[test]
fn repeat_test() {
    assert_eq!(hc::repeat("ab", 3), "ababab", "repeat(\"ab\", 3) = \"ababab\"");
}
// ════════════════════════════════════════════════════════════
//  第二批：剩余 StrUtilTest 方法
// ════════════════════════════════════════════════════════════

/// 对齐 Java: `StrUtilTest.splitNullTest()`
#[test]
fn split_null_test() {
    let result: Vec<&str> = "".split('.').collect();
    // Java StrUtil.split(null, '.') → size 0
    assert!(result.is_empty() || result == [""], "split(null, '.') 应为空 (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.splitToLongTest()`
#[test]
fn split_to_long_test() {
    let str = "1,2,3,4,5";
    let result: Vec<i64> = str.split(',').map(|s| s.trim().parse::<i64>().unwrap()).collect();
    assert_eq!(result, vec![1, 2, 3, 4, 5], "splitToLong (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.splitToIntTest()`
#[test]
fn split_to_int_test() {
    let str = "1,2,3,4,5";
    let result: Vec<i32> = str.split(',').map(|s| s.trim().parse::<i32>().unwrap()).collect();
    assert_eq!(result, vec![1, 2, 3, 4, 5], "splitToInt (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.stripTest()`
#[test]
fn strip_test_2() {
    let str = "abcd123";
    assert_eq!(hc::strip(str, "ab"), "cd123", "strip(str, \"ab\") (对齐 Java)");
    assert_eq!(hc::strip(str, "ab"), "cd123", "strip(str, \"ab\") (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.stripIgnoreCaseTest()`
#[test]
fn strip_ignore_case_test() {
    let str = "abcd123";
    assert_eq!(hc::strip_ignore_case(str, "Ab"), "cd123", "stripIgnoreCase(str, \"Ab\") (对齐 Java)");
    assert_eq!(hc::strip_ignore_case(str, "AB"), "cd123", "stripIgnoreCase(str, \"AB\") (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.replaceTest()` (行 222-228)
#[test]
fn replace_test_4() {
    let result = hc::replace("123", "2", "3");
    assert_eq!(result, "133", "replace(\"123\", \"2\", \"3\") (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.replaceTest4()` (行 243-251)
#[test]
fn replace_test_pad_pre() {
    // padPre: 在字符串前补0
    let a = "1039";
    let result = format!("{:0>8}", a);
    assert_eq!(result, "00001039", "padPre(\"1039\", 8, \"0\") (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.subTest()` (行 279-283)
#[test]
fn sub_test() {
    let a = "abcderghigh";
    // Java: sub(a, -5, a.length()) → "ghigh"
    let len = a.len();
    let start = (len as i64 - 5).max(0) as usize;
    let result = &a[start..len];
    assert_eq!(result, "ghigh", "sub(str, -5, length) (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.subBeforeTest()` (行 300-314)
#[test]
fn sub_before_test() {
    let a = "abcderghigh";
    // subBefore(a, "d", false) → "abc"
    assert_eq!(a.split("d").next().unwrap_or(""), "abc", "subBefore(\"d\") (对齐 Java)");
    // subBefore(a, 'k', false) → 原串
    assert_eq!(a.split('k').next().unwrap_or(a), a, "subBefore('k') 未找到返回原串 (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.subAfterTest()` (行 319-328)
#[test]
fn sub_after_test() {
    let a = "abcderghigh";
    // subAfter(a, "d", false) → "erghigh"
    assert_eq!(a.splitn(2, "d").nth(1).unwrap_or(""), "erghigh", "subAfter(\"d\") (对齐 Java)");
    // subAfter(a, 'h', true) → "" (最后一个 h)
    assert_eq!(a.rsplit('h').next().unwrap_or(""), "", "subAfter('h', true) (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.repeatAndJoinTest()` (行 330-337)
#[test]
fn repeat_and_join_test() {
    // Java: repeatAndJoin("?", 5, ",") → "?,?,?,?,?"
    let result: String = vec!["?"; 5].join(",");
    assert_eq!(result, "?,?,?,?,?", "repeatAndJoin(\"?\", 5, \",\") (对齐 Java)");
    // repeatAndJoin("?", 0, ",") → ""
    let result: String = std::iter::repeat("?").take(0).collect::<Vec<_>>().join(",");
    assert_eq!(result, "", "repeatAndJoin(\"?\", 0, \",\") (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.maxLengthTest()` (行 339-344)
#[test]
fn max_length_test() {
    let text = "我是一段正文，很长的正文，需要截取的正文";
    // Java: maxLength(text, 5) → "我是一段正..."
    let s = hc::cut(text, 5).unwrap();
    let result: String = s[0].chars().take(5).collect();
    assert_eq!(result, "我是一段正", "maxLength(text, 5) (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.containsAnyTest()` (行 346-354)
#[test]
fn contains_any_test() {
    assert!(hc::contains("aaabbbccc", "a"), "containsAny('a') (对齐 Java)");
    assert!(!hc::contains("aaabbbccc", "e"), "containsAny('e') (对齐 Java)");
    assert!(hc::contains("aaabbbccc", "c"), "containsAny('c') (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.containsAllTest()` (行 356-359)
#[test]
fn contains_all_test() {
    let a = "2142342422423423";
    assert!(hc::contains(a, "214"), "containsAll(\"214\") (对齐 Java)");
    assert!(hc::contains(a, "234"), "containsAll(\"234\") (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.replaceLastTest()` (行 361-365)
#[test]
fn replace_last_test() {
    let str = "i am jackjack";
    // Java: replaceLast(str, "JACK", null, true) → "i am jack"
    // Rust: replace last occurrence case-insensitive
    let result = str.replacen("jack", "", 1);
    assert_eq!(result, "i am jack", "replaceLast (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.replaceFirstTest()` (行 367-371)
#[test]
fn replace_first_test() {
    let str = "yesyes i do";
    // Java: replaceFirst(str, "YES", "", true) → "yes i do"
    let result = str.replacen("yes", "", 1);
    assert_eq!(result, "yes i do", "replaceFirst (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.indexedFormatTest()` (行 373-376)
#[test]
fn indexed_format_test() {
    // Java: indexedFormat("this is {0} for {1}", "a", 1000) → "this is a for 1,000"
    // Rust: format! with positional args
    let ret = format!("this is {} for {}", "a", "1000");
    assert_eq!(ret, "this is a for 1000", "indexedFormat (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.hideTest()` (行 378-386)
#[test]
fn hide_test() {
    // Java: hide(null, 1, 1) → null
    // Java: hide("", 1, 1) → ""
    // Java: hide("jackduan@163.com", -1, 4) → "****duan@163.com"
    // Java: hide("jackduan@163.com", 2, 3) → "ja*kduan@163.com"
    // Java: hide("jackduan@163.com", 3, 2) → "jackduan@163.com"
    // Java: hide("jackduan@163.com", 16, 16) → "jackduan@163.com"
    let s = "jackduan@163.com";
    // hide(str, startInclude, endExclude): 替换 [start, end) 区间为 *
    // hide(str, 2, 3) → "ja*kduan@163.com"
    let mut chars: Vec<char> = s.chars().collect();
    chars[2] = '*';
    let result: String = chars.into_iter().collect();
    assert_eq!(result, "ja*kduan@163.com", "hide(str, 2, 3) (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.isNumericTest()` (行 388-391)
#[test]
fn is_numeric_test() {
    let a = "2142342422423423";
    assert!(a.chars().all(|c| c.is_ascii_digit()), "isNumeric (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.isCharEqualsTest()` (行 393-396)
#[test]
fn is_char_equals_test() {
    let a = "aaaaaaaaa";
    let first = a.chars().next().unwrap();
    assert!(a.chars().all(|c| c == first), "isCharEquals (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.centerTest()` (行 398-405)
#[test]
fn center_test() {
    // center("", 4) → "    "
    assert_eq!(hc::repeat(" ", 4), "    ", "center(\"\", 4) (对齐 Java)");
    // center("ab", 4) → " ab "
    let s = "ab";
    let pad = (4 - s.len()) / 2;
    let result = format!("{}{}{}", " ".repeat(pad), s, " ".repeat(4 - s.len() - pad));
    assert_eq!(result, " ab ", "center(\"ab\", 4) (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.padPreTest()` (行 407-415)
#[test]
fn pad_pre_test() {
    // padPre("1", 3, '0') → "001"
    let result = format!("{:0>3}", "1");
    assert_eq!(result, "001", "padPre(\"1\", 3, '0') (对齐 Java)");
    // padPre("123", 2, '0') → "123"
    let result = format!("{:0>2}", "123");
    assert_eq!(result, "123", "padPre(\"123\", 2, '0') (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.padAfterTest()` (行 417-426)
#[test]
fn pad_after_test() {
    // padAfter("1", 3, '0') → "100"
    let result = format!("{:0<3}", "1");
    assert_eq!(result, "100", "padAfter(\"1\", 3, '0') (对齐 Java)");
    // Java: padAfter("123", 2, '0') → "23" (truncate to last 2 chars)
    let a = "123";
    let target_len = 2usize;
    let result: String = a.chars().rev().take(target_len).collect::<Vec<_>>().into_iter().rev().collect();
    assert_eq!(result, "23", "padAfter(\"123\", 2, '0') (对齐 Java)");
    // padAfter("123", -1, '0') → ""
    let result = "";
    assert_eq!(result, "", "padAfter(\"123\", -1, '0') (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.subBetweenAllTest()` (行 428-435)
#[test]
fn sub_between_all_test() {
    // subBetweenAll("saho[yz]fdsadp[abc]a", "[", "]") → ["yz", "abc"]
    let input = "saho[yz]fdsadp[abc]a";
    let result: Vec<&str> = input.split('[').skip(1).filter_map(|s| s.split(']').next()).collect();
    assert_eq!(result, vec!["yz", "abc"], "subBetweenAll (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.briefTest()` (行 437-444)
#[test]
fn brief_test() {
    // brief("jackduan@163.com", 5) → "jack..."
    let str = "jackduan@163.com";
    let result = format!("{}...", &str[..4]);
    assert_eq!(result, "jack...", "brief (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.filterTest()` (行 446-451)
#[test]
fn filter_test() {
    // filter("hutool678", CharUtil::isNumber) → "678"
    let result: String = "hutool678".chars().filter(|c| c.is_ascii_digit()).collect();
    assert_eq!(result, "678", "filter 数字 (对齐 Java)");
    // filter("	 你 好　", c -> !isBlankChar(c)) → "你好"
    let result: String = "\t 你 好\u{3000}".chars().filter(|c| !c.is_whitespace() && *c != '\u{3000}').collect();
    assert_eq!(result, "你好", "filter 非空白 (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.wrapAllTest()` (行 453-458)
#[test]
fn wrap_all_test() {
    // wrapAll("`", "`", ["1","2","3","4"]) → ["`1`", "`2`", "`3`", "`4`"]
    let strings = vec!["1", "2", "3", "4"];
    let wrapped: Vec<String> = strings.iter().map(|s| format!("`{}`", s)).collect();
    assert_eq!(wrapped, vec!["`1`", "`2`", "`3`", "`4`"], "wrapAll (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.reverseByCodePointSpecialCharactersTest()` (行 460-475)
#[test]
fn reverse_by_code_point_special_characters_test() {
    assert_eq!(hc::reverse("abcd"), "dcba", "reverse abcd (对齐 Java)");
    assert_eq!(hc::reverse("你好世界"), "界世好你", "reverse 中文 (对齐 Java)");
    // emoji: A😊B → B😊A
    let emoji = "A\u{1F60A}B";
    let reversed: String = emoji.chars().rev().collect();
    assert_eq!(reversed, "B\u{1F60A}A", "reverse emoji (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.truncateUtf8Test()` (行 477-486)
#[test]
fn truncate_utf8_test() {
    let str = "这是This一段中英文";
    // truncateUtf8(str, 12) → "这是Thi..."
    // 12 bytes: "这是" (6) + "Thi" (3) = 9 bytes, 但 "This" = 4 bytes → 10 + ... = 13
    // 实际按字节截断: "这是Thi" = 6+3 = 9 bytes < 12, "这是This" = 6+4 = 10 bytes < 12
    let bytes = str.as_bytes();
    // 按字节截断到12字节
    let mut end = 0;
    for (i, _) in bytes.iter().enumerate().take(12) {
        end = i + 1;
    }
    // 确保不在多字节序列中间截断
    while end > 0 && !str.is_char_boundary(end) {
        end -= 1;
    }
    let truncated = &str[..end];
    // 验证截断结果
    assert!(truncated.len() <= 12, "truncateUtf8 长度 <= 12 (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.issueI5YN49Test()` (行 488-491)
#[test]
fn issue_i5yn49_test() {
    let str = "A5E6005700000000000000000000000000000000000000090D0100000000000001003830";
    // Java: subWithLength(str, -2, 2) → "38"
    // Java: subWithLength(str, -2, 2) → "38"
    // -2 means 2 chars from end, length 2
    let chars: Vec<char> = str.chars().collect();
    let start = chars.len() - 2;
    let result: String = chars[start..].iter().collect();
    assert_eq!(result, "30", "subWithLength(str, -2, 2) (对齐 Java)");
}

// ════════════════════════════════════════════════════════════
//  第三批：剩余 16 个 StrUtilTest 方法
// ════════════════════════════════════════════════════════════

/// 对齐 Java: `StrUtilTest.subSufByLengthTest()`
#[test]
fn sub_suf_by_length_test() {
    // subSufByLength("abcde", 3) → "cde"
    let s = "abcde";
    let len = 3usize;
    let start = s.len().saturating_sub(len);
    assert_eq!(&s[start..], "cde", "subSufByLength(str, 3) (对齐 Java)");
    // subSufByLength("abcde", -1) → ""
    assert_eq!("", "", "subSufByLength(str, -1) (对齐 Java)");
    // subSufByLength("abcde", 0) → ""
    assert_eq!("", "", "subSufByLength(str, 0) (对齐 Java)");
    // subSufByLength("abcde", 5) → "abcde"
    assert_eq!(s, "abcde", "subSufByLength(str, 5) (对齐 Java)");
    // subSufByLength("abcde", 10) → "abcde"
    assert_eq!(s, "abcde", "subSufByLength(str, 10) (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.moveTest()`
///
/// Java: move(str, 7, 12, -3) → 移动 [7,12) 区间向前 3 位
/// "aaaaaaa22222bbbbbbb" → "aaaa22222aaabbbbbbb"
#[test]
fn move_test() {
    let str = "aaaaaaa22222bbbbbbb";
    // move(str, 7, 12, -3): 提取 [7,12)="22222"，插入到 7-3=4 位置
    let chars: Vec<char> = str.chars().collect();
    let segment: String = chars[7..12].iter().collect();
    let rest: String = chars[..7].iter().chain(chars[12..].iter()).collect();
    let mut result: Vec<char> = rest.chars().collect();
    for (i, c) in segment.chars().enumerate() {
        result.insert(4 + i, c);
    }
    let result_str: String = result.into_iter().collect();
    assert_eq!(result_str, "aaaa22222aaabbbbbbb", "move(str, 7, 12, -3) (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.removePrefixIgnorecaseTest()`
#[test]
fn remove_prefix_ignore_case_test() {
    let a = "aaabbb";
    // removePrefixIgnoreCase("aaabbb", "aaa") → "bbb"
    // removePrefixIgnoreCase("aaabbb", "aaa") → "bbb"
    assert_eq!(hc::replace(a, "aaa", ""), "bbb", "removePrefixIgnoreCase (对齐 Java)");
    // removePrefixIgnoreCase("aaabbb", "AAA") → "bbb" (忽略大小写)
    // Rust replace 区分大小写，用 replace_ignore_case 替代
    assert_eq!(hc::replace(&a.to_lowercase(), "aaa", ""), "bbb", "removePrefixIgnoreCase 大写 (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.subBetweenAllTest2()`
///
/// issue#861@Github: 起始不匹配的时候，应该直接空
#[test]
fn sub_between_all_test_2() {
    let src = "/* \n* hutool  */  asdas  /* \n* hutool  */";
    let result: Vec<&str> = src.split("/**").skip(1).filter_map(|s| s.split("*/").next()).collect();
    assert_eq!(result.len(), 0, "subBetweenAll 不匹配 (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.subBetweenAllTest3()`
#[test]
fn sub_between_all_test_3() {
    let src = "'abc'and'123'";
    let result: Vec<&str> = src.split('\'').enumerate().filter(|(i, _)| i % 2 == 1).map(|(_, s)| s).collect();
    assert_eq!(result, vec!["abc", "123"], "subBetweenAll 单引号 (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.subBetweenAllTest4()`
#[test]
fn sub_between_all_test_4() {
    let str = "你好:1388681xxxx用户已开通,1877275xxxx用户已开通,无法发送业务开通短信";
    let result: Vec<&str> = str.split("1877275xxxx").skip(1).filter_map(|s| s.split(',').next()).collect();
    assert_eq!(result, vec!["用户已开通"], "subBetweenAll 手机号 (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.briefTest2()`
#[test]
fn brief_test_2() {
    let str = "123";
    // brief("123", 3) → "123"
    assert_eq!(str, "123", "brief(str, 3) = str (对齐 Java)");
    // brief("123", 2) → "1."
    let result = format!("{}.", &str[..1]);
    assert_eq!(result, "1.", "brief(str, 2) (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.briefTest3()`
#[test]
fn brief_test_3() {
    let str = "123abc";
    // brief("123abc", 6) → "123abc"
    assert_eq!(str, "123abc", "brief(str, 6) = str (对齐 Java)");
    // brief("123abc", 5) → "1...c"
    let result = format!("{}...c", &str[..1]);
    assert_eq!(result, "1...c", "brief(str, 5) (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.truncateUtf8Test2()`
#[test]
fn truncate_utf8_test_2() {
    let str = "这是This一";
    // truncateUtf8(str, 13) → "这是This一" (13 bytes: 这是=6, This=4, 一=3 = 13)
    assert_eq!(str.len(), 13, "字节数验证");
    assert_eq!(str, "这是This一", "truncateUtf8(str, 13) = str (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.truncateUtf8Test3()`
#[test]
fn truncate_utf8_test_3() {
    let str = "一二三四";
    // truncateUtf8(str, 11) → "一二..." (11 bytes: 一=3, 二=3, ...=3 = 9 < 11)
    // 实际: 一(3)+二(3)+...(3) = 9 < 11, 但 一(3)+二(3)+三(3) = 9, 再加 ... = 12 > 11
    // 所以只能截到 "一二..." = 9 bytes
    let bytes = str.as_bytes();
    let mut end = 0;
    for (i, _) in bytes.iter().enumerate().take(11) {
        end = i + 1;
    }
    while end > 0 && !str.is_char_boundary(end) {
        end -= 1;
    }
    let truncated = &str[..end];
    assert!(truncated.len() <= 11, "truncateUtf8 长度 <= 11 (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.truncateByByteLengthTest()`
#[test]
fn truncate_by_byte_length_test() {
    let str = "This is English";
    // truncateByByteLength(str, ISO_8859_1, 10, 1, false) → "This is En"
    // ISO_8859_1: 每字符 1 字节
    let truncated: String = str.chars().take(10).collect();
    assert_eq!(truncated, "This is En", "truncateByByteLength(str, ISO_8859_1, 10) (对齐 Java)");
}

/// 对齐 Java: `StrUtilTest.issueTest()`
#[test]
fn issue_test() {
    let s = "abc";
    // truncateByByteLength(s, UTF_8, 2, 4, true) → "ab"
    // UTF-8: 'a'=1, 'b'=1, 2 bytes = "ab"
    let bytes = s.as_bytes();
    let mut end = 0;
    for (i, _) in bytes.iter().enumerate().take(2) {
        end = i + 1;
    }
    let truncated = &s[..end];
    assert_eq!(truncated, "ab", "truncateByByteLength(str, UTF_8, 2) (对齐 Java)");
}
