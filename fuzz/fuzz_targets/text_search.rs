#![no_main]

use hitool_dfa::WordTree;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(text) = std::str::from_utf8(data) {
        let tree = WordTree::new(["hitool", "工具", "sensitive"]).unwrap();
        let _ = tree.find_all(text);
        let _ = tree.replace_all(text, "***");
    }
});
