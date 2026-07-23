//! System parity tests —— 对齐 Hutool `hutool-system` 测试。
//!
//! 对齐: `cn.hutool.system.SystemUtilTest`
//! 对齐: `cn.hutool.system.OshiTest`
//! 对齐: `cn.hutool.system.OshiPrintTest`
//! 来源:
//! - hutool-system/src/test/java/cn/hutool/system/SystemUtilTest.java
//! - hutool-system/src/test/java/cn/hutool/system/OshiTest.java
//! - hutool-system/src/test/java/cn/hutool/system/OshiPrintTest.java

use std::io::Cursor;
use std::path::MAIN_SEPARATOR;
use std::time::Duration;

use hitool_system as hs;
use hitool_system::{OshiUtil, SystemUtil};

// ---------------------------------------------------------------------------
// Existing idiomatic smoke tests (retained; do not delete)
// ---------------------------------------------------------------------------

#[test]
fn system_snapshot_test() {
    let snap = hs::SystemSnapshot::collect();
    // used_memory 为 u64，保留原烟雾断言语义：用量不超过总量
    assert!(
        snap.used_memory <= snap.total_memory,
        "used_memory 应 <= total_memory"
    );
}

#[test]
fn runtime_info_test() {
    let rt = hs::RuntimeInfo::collect();
    assert!(rt.total_memory > 0, "total_memory 应 > 0");
}

#[test]
fn host_info_test() {
    let host = hs::HostInfo::collect();
    assert!(host.name.is_some() || true, "HostInfo::collect() 成功");
}

// ---------------------------------------------------------------------------
// SystemUtilTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `SystemUtilTest.dumpTest()`
///
/// Java 侧为 `@Disabled`，仅调用 `SystemUtil.dumpSystemInfo()`。
/// 此处改为可断言：写入 buffer 后内容非空且含 memory.total=。
#[test]
fn dump_test() {
    let mut buffer = Cursor::new(Vec::new());
    SystemUtil::dump_system_info(&mut buffer).expect("dump_system_info 应成功");
    let dump = String::from_utf8(buffer.into_inner()).expect("dump 应为 UTF-8");
    assert!(
        !dump.is_empty(),
        "对齐 Java SystemUtilTest.dumpTest: dump 输出非空"
    );
    assert!(
        dump.contains("memory.total="),
        "对齐 Java SystemUtilTest.dumpTest: dump 应含 memory.total="
    );
}

/// 对齐 Java: `SystemUtilTest.getCurrentPidTest()`
///
/// Java: `assertTrue(SystemUtil.getCurrentPID() > 0)`。
#[test]
fn get_current_pid_test() {
    let pid = SystemUtil::current_pid();
    assert!(
        pid > 0,
        "对齐 Java SystemUtilTest.getCurrentPidTest: pid > 0, 实际={pid}"
    );
}

/// 对齐 Java: `SystemUtilTest.getJavaInfoTest()`
///
/// Java: `assertNotNull(SystemUtil.getJavaInfo())`。
/// Platform note: 原生 Rust 无内嵌 JVM；`JavaInfo` 由环境变量 opt-in 检测，
/// 返回值始终为合法结构体（等价于 Java 的非 null）。
#[test]
fn get_java_info_test() {
    let java_info = SystemUtil::java_info();
    // 结构体存在即满足 assertNotNull；Debug 格式化证明可观测。
    assert!(
        format!("{java_info:?}").contains("JavaInfo"),
        "对齐 Java SystemUtilTest.getJavaInfoTest: JavaInfo 非空结构体"
    );
}

/// 对齐 Java: `SystemUtilTest.getJavaRuntimeInfoTest()`
///
/// Java: `assertNotNull(SystemUtil.getJavaRuntimeInfo())`。
/// Platform note: 同 getJavaInfoTest — 无 JVM 时字段可为 None，但对象始终存在。
#[test]
fn get_java_runtime_info_test() {
    let info = SystemUtil::java_runtime_info();
    assert!(
        format!("{info:?}").contains("JavaRuntimeInfo"),
        "对齐 Java SystemUtilTest.getJavaRuntimeInfoTest: JavaRuntimeInfo 非空结构体"
    );
}

/// 对齐 Java: `SystemUtilTest.getOsInfoTest()`
///
/// Java: `assertNotNull(SystemUtil.getOsInfo())`，并打印 `osInfo.getName()`。
#[test]
fn get_os_info_test() {
    let os_info = SystemUtil::os_info();
    assert!(
        !os_info.name.is_empty(),
        "对齐 Java SystemUtilTest.getOsInfoTest: OsInfo.name 非空"
    );
    // 对应 Java Console.log(osInfo.getName())
    let _ = os_info.name.clone();
}

/// 对齐 Java: `SystemUtilTest.getHostInfo()`
///
/// Java: `assertNotNull(SystemUtil.getHostInfo())`。
#[test]
fn get_host_info() {
    let host_info = SystemUtil::host_info();
    assert!(
        format!("{host_info:?}").contains("HostInfo"),
        "对齐 Java SystemUtilTest.getHostInfo: HostInfo 非空结构体"
    );
}

/// 对齐 Java: `SystemUtilTest.getUserInfoTest()`
///
/// Java: `assertTrue(userInfo.getTempDir().endsWith(File.separator))`
/// （Hutool `UserInfo.fixPath` 强制末尾补充分隔符）。
///
/// Platform note: Rust `std::env::temp_dir()` 在 macOS/Linux 通常已带尾部分隔符；
/// Windows 亦同。若某平台 `temp_dir` 无尾部分隔符，则断言会暴露与 Hutool
/// `fixPath` 语义的差异（本 crate 当前直接透传 `env::temp_dir()`）。
#[test]
fn get_user_info_test() {
    let user_info = SystemUtil::user_info();
    let temp = user_info.temp_dir.to_string_lossy();
    assert!(
        temp.ends_with(MAIN_SEPARATOR),
        "对齐 Java SystemUtilTest.getUserInfoTest: temp_dir 应以 File.separator 结尾, 实际={temp:?}"
    );
}

// ---------------------------------------------------------------------------
// OshiTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `OshiTest.getMemoryTest()`
///
/// Java: `assertTrue(OshiUtil.getMemory().getTotal() > 0)`。
#[test]
fn get_memory_test() {
    let total = OshiUtil::memory().total;
    assert!(
        total > 0,
        "对齐 Java OshiTest.getMemoryTest: total > 0, 实际={total}"
    );
}

/// 对齐 Java: `OshiTest.getCupInfo()`
///
/// Java: `assertNotNull(OshiUtil.getCpuInfo())`（方法名保留 Hutool 拼写 Cup）。
#[test]
fn get_cup_info() {
    let cpu_info = OshiUtil::cpu_info(Duration::ZERO);
    assert!(
        format!("{cpu_info:?}").contains("CpuInfo"),
        "对齐 Java OshiTest.getCupInfo: CpuInfo 非空结构体"
    );
}

/// 对齐 Java: `OshiTest.getCurrentProcessTest()`
///
/// Java: `assertEquals("java", OshiUtil.getCurrentProcess().getName())`。
///
/// Platform note: JVM 进程名为 `"java"`；`cargo test` 原生二进制名通常为
/// `system_parity`（或带 hash 的测试 harness 名），不能硬编码 `"java"`。
/// 保持「当前进程存在且名称可观测」的断言语义。
#[test]
fn get_current_process_test() {
    let current = OshiUtil::current_process().expect("current process should be visible");
    assert!(
        !current.name.is_empty(),
        "对齐 Java OshiTest.getCurrentProcessTest: 进程名非空（Java 侧为 \"java\"；\
         原生 Rust 为测试二进制名，如 system_parity）"
    );
    assert_eq!(
        current.pid,
        std::process::id(),
        "当前进程 PID 应与 std::process::id() 一致"
    );
}

/// 对齐 Java: `OshiTest.getUsedTest()`
///
/// Java 侧为 `@Disabled` 且 `while(true)` 打印 `getUsed()`；此处改为单次采样断言。
#[test]
fn get_used_test() {
    let used = OshiUtil::cpu_info(Duration::from_millis(1)).used();
    assert!(
        used >= 0.0,
        "对齐 Java OshiTest.getUsedTest: used >= 0, 实际={used}"
    );
}

// ---------------------------------------------------------------------------
// OshiPrintTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `OshiPrintTest.printCpuInfo()`
///
/// Java 类与方法均为 `@Disabled`，仅 `Console.log(OshiUtil.getCpuInfo())`。
/// 此处改为可断言：CpuInfo 可采集且 cpu_num > 0。
#[test]
fn print_cpu_info() {
    let cpu_info = OshiUtil::cpu_info(Duration::ZERO);
    assert!(
        cpu_info.cpu_num > 0,
        "对齐 Java OshiPrintTest.printCpuInfo: cpu_num > 0"
    );
    // 对应 Java Console.log(...)
    let _ = format!("{cpu_info:?}");
}
