use hitool_dfa as hd;

#[test]
fn dfa_word_tree_test() {
    let mut tree = hd::WordTree::new();
    tree.add_word("ab");
    tree.add_word("abc");
    tree.add_word("abcd");
    let result = tree.match_first("hello abc world");
    assert!(result.is_some(), "应找到匹配");
    // DFA 匹配第一个遇到的完整词，可能是 "ab"
    let found = result.unwrap();
    assert!(found == "ab" || found == "abc" || found == "abcd",
        "匹配结果应为 ab/abc/abcd 中的一个, 实际: {}", found);
}

#[test]
fn dfa_word_tree_no_match_test() {
    let mut tree = hd::WordTree::new();
    tree.add_word("xyz");
    let result = tree.match_first("hello abc");
    assert!(result.is_none(), "xyz 不应匹配");
}

#[test]
fn dfa_matcher_test() {
    let matcher = hd::DfaMatcher::new(["ab", "abc", "abcd"]).unwrap();
    let results = matcher.find_all("hello abc world");
    assert!(results.len() > 0, "应找到匹配");
}
