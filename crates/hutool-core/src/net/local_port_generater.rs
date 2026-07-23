//! 对齐: `cn.hutool.core.net.LocalPortGenerater`
//! 来源: hutool-core/src/main/java/cn/hutool/core/net/LocalPortGenerater.java

use std::sync::atomic::{AtomicU16, Ordering};

use crate::net::net_util::NetUtil;

/// 对齐 Java 类: `cn.hutool.core.net.LocalPortGenerater`
#[derive(Debug)]
pub struct LocalPortGenerater {
    next: AtomicU16,
    max: u16,
}

impl LocalPortGenerater {
    /// 对齐 Java: `new LocalPortGenerater(int)`
    pub fn new(min_port: u16) -> Self {
        Self {
            next: AtomicU16::new(min_port.max(NetUtil::PORT_RANGE_MIN)),
            max: NetUtil::PORT_RANGE_MAX,
        }
    }

    /// 对齐 Java: `LocalPortGenerater.generate()`
    pub fn generate(&self) -> Option<u16> {
        loop {
            let candidate = self.next.fetch_add(1, Ordering::Relaxed);
            if candidate > self.max {
                return None;
            }
            if NetUtil::is_usable_local_port(candidate as i32) {
                return Some(candidate);
            }
        }
    }
}
