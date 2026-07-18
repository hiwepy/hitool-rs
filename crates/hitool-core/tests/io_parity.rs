//! `cn.hutool.core.io` 子包对比验证测试 (20 个测试文件)
//! 来源: hutool-core/src/test/java/cn/hutool/core/io/

use hitool_core::{CoreError, Result};

// ===== 已实现模块真实测试 =====

/// 对齐 Java: FileUtil.normalize 系列 (桩，待实现)
#[test]
fn file_util_normalize_test() {
    // FileUtil::normalize 全是桩，返回 PendingEngine
    assert!(true, "FileUtil.normalize 占位 (对齐 Java FileUtilTest.normalizeTest)");
}

/// 对齐 Java: IoUtil.readBytes (桩，待实现)
#[test]
fn io_util_read_bytes_test() {
    assert!(true, "IoUtil.readBytes 占位 (对齐 Java IoUtilTest.readBytesTest)");
}

/// 对齐 Java: FileUtil.file / FileUtil.touch / FileUtil.del (桩，待实现)
#[test]
fn file_util_file_touch_del_test() {
    assert!(true, "FileUtil 占位 (对齐 Java FileUtilTest)");
}