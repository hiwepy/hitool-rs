//! AsyncUtil 对比验证测试
//! 对齐: `cn.hutool.core.thread.AsyncUtilTest`
//! 来源: hutool-core/src/test/java/cn/hutool/core/thread/AsyncUtilTest.java
//!
//! 需要启用 feature: `cargo test -p hitool-core --features async --test async_util_parity`

#![cfg(feature = "async")]

use hitool_core::thread::async_util::AsyncUtil;
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time::sleep;

/// 对齐 Java: `AsyncUtilTest.waitAndGetTest`
///
/// Java 原测试 `@Disabled`,此处作为可执行 parity。
/// 差异:Java `CompletableFuture` 可先 `waitAll` 再多次 `get`;
/// Rust `JoinHandle` 只能 await 一次,故用 `get` 依次取结果,
/// 并用独立一组 handle 验证 `wait_all`。
#[tokio::test]
async fn wait_and_get_test() {
    // --- get 路径:拼接 "hutool卫衣真暖和" ---
    let hutool = tokio::spawn(async {
        sleep(Duration::from_millis(10)).await;
        "hutool".to_string()
    });
    let sweater = tokio::spawn(async {
        sleep(Duration::from_millis(20)).await;
        "卫衣".to_string()
    });
    let warm = tokio::spawn(async {
        sleep(Duration::from_millis(30)).await;
        "真暖和".to_string()
    });

    let combined = format!(
        "{}{}{}",
        AsyncUtil::get(hutool).await.unwrap(),
        AsyncUtil::get(sweater).await.unwrap(),
        AsyncUtil::get(warm).await.unwrap()
    );
    assert_eq!(combined, "hutool卫衣真暖和");

    // --- wait_all 路径 ---
    AsyncUtil::wait_all(vec![
        tokio::spawn(async {
            sleep(Duration::from_millis(10)).await;
            "hutool".to_string()
        }),
        tokio::spawn(async {
            sleep(Duration::from_millis(20)).await;
            "卫衣".to_string()
        }),
        tokio::spawn(async {
            sleep(Duration::from_millis(30)).await;
            "真暖和".to_string()
        }),
    ])
    .await
    .expect("wait_all should succeed");
}

/// 对齐 Java: `AsyncUtil.waitAll` 空任务立即返回。
#[tokio::test]
async fn wait_all_empty_ok() {
    let empty: Vec<JoinHandle<()>> = Vec::new();
    AsyncUtil::wait_all(empty).await.unwrap();
}

/// 对齐 Java: `AsyncUtil.waitAny` — 先完成的任务决定返回值。
#[tokio::test]
async fn wait_any_returns_first_completed() {
    let slow = tokio::spawn(async {
        sleep(Duration::from_millis(50)).await;
        "slow"
    });
    let fast = tokio::spawn(async {
        sleep(Duration::from_millis(5)).await;
        "fast"
    });

    let result = AsyncUtil::wait_any(vec![slow, fast]).await.unwrap();
    assert_eq!(result, "fast");
}

/// 对齐 Java: `AsyncUtil.waitAny` 空任务应失败(Rust 显式错误)。
#[tokio::test]
async fn wait_any_empty_err() {
    let empty: Vec<JoinHandle<&'static str>> = Vec::new();
    let err = AsyncUtil::wait_any(empty).await.unwrap_err();
    assert!(err.message.contains("no tasks"));
}

/// 对齐 Java: `AsyncUtil.get` — 单任务取值。
#[tokio::test]
async fn get_returns_value() {
    let handle = tokio::spawn(async { 42 });
    assert_eq!(AsyncUtil::get(handle).await.unwrap(), 42);
}

/// 对齐 Java: `AsyncUtil.get` — 任务 panic 时包装为 ThreadException。
#[tokio::test]
async fn get_wraps_panic_as_thread_exception() {
    let handle = tokio::spawn(async {
        panic!("boom");
    });
    let err = AsyncUtil::get(handle).await.unwrap_err();
    assert!(
        err.message.contains("panic") || err.message.contains("boom"),
        "unexpected message: {}",
        err.message
    );
}

/// 便捷 API:`wait_all_futures` / `wait_any_futures` / `get_future`。
#[tokio::test]
async fn wait_futures_helpers() {
    // 使用 map 产生同一 Future 类型,避免 vec![async{}, async{}] 类型不一致
    AsyncUtil::wait_all_futures((0..2).map(|i| async move {
        sleep(Duration::from_millis(5)).await;
        i
    }))
    .await
    .unwrap();

    let first = AsyncUtil::wait_any_futures([40_u64, 5].into_iter().map(|ms| async move {
        sleep(Duration::from_millis(ms)).await;
        if ms < 10 { "b" } else { "a" }
    }))
    .await
    .unwrap();
    assert_eq!(first, "b");

    let v = AsyncUtil::get_future(async { 7 }).await.unwrap();
    assert_eq!(v, 7);
}
