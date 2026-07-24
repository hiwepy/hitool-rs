//! Spring POJO facade，对齐 hutool 的 `cn.hutool.extra.spring.*`。
//!
//! **仅提供 trait 抽象**。具体 Spring Framework 依赖（ApplicationContext / BeanFactory）
//! 是 Java-only，属于 unsafe-to-copy。Rust 用户应使用依赖注入框架（如 axum::Extension、
//! shaku、self-rs 等）替代。

use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

use crate::HutoolException;

mod application_context;
mod configurable_bean_factory;
mod spring_util;
mod application_context_ext;

pub use application_context::ApplicationContext;
pub use configurable_bean_factory::ConfigurableBeanFactory;
pub use spring_util::SpringUtil;
pub use application_context_ext::ApplicationContextExt;
pub use application_context::enable_spring_util;
