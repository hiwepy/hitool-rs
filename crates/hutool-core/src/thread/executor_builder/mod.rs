//! 对齐: `cn.hutool.core.thread.ExecutorBuilder`
//! 来源: hutool-core/src/main/java/cn/hutool/core/thread/ExecutorBuilder.java

use crate::thread::named_thread_factory::NamedThreadFactory;
use crate::thread::reject_policy::RejectPolicy;
use std::sync::mpsc::{self, Receiver, Sender, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

mod simple_executor;
mod executor_builder;

pub use simple_executor::SimpleExecutor;
pub use executor_builder::ExecutorBuilder;
