//! 对齐: `cn.hutool.core.thread.ThreadUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/thread/ThreadUtil.java
//!
//! 以 `std::thread` 提供可移植子集；JVM `ThreadLocal` / `ThreadGroup` 全局语义保持 planned。

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock};
use std::thread::{self, JoinHandle, Thread, ThreadId};
use std::time::{Duration, Instant};

use super::concurrency_tester::ConcurrencyTester;
use super::executor_builder::{ExecutorBuilder, SimpleExecutor};
use super::global_thread_pool::GlobalThreadPool;
use super::named_thread_factory::NamedThreadFactory;
use super::reject_policy::RejectPolicy;
use super::thread_factory_builder::ThreadFactoryBuilder;

mod thread_util;
mod scheduled_handle;
mod count_down_latch;
mod scheduled_pool;

pub use thread_util::ThreadUtil;
pub use scheduled_handle::ScheduledHandle;
pub use count_down_latch::CountDownLatch;
pub use scheduled_pool::ScheduledPool;
