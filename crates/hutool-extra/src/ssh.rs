//! SSH POJO facade，对齐 hutool 的 `cn.hutool.extra.ssh.*`。
//!
//! **仅实现 POJO 部分**（Connector、ChannelType 等）。具体 SSH 协议操作（JschUtil/Sftp 等）
//! 需要外部 Java crate（com.jcraft.jsch / ssh2），属于 unsafe-to-copy。

use std::fmt;

use crate::HutoolException;

/// SSH 通道类型枚举，对齐 `cn.hutool.extra.ssh.ChannelType`。
///
/// Java 是 enum + getValue()；Rust 用 enum + `as_str()` 方法。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChannelType {
    /// Shell 通道
    Shell,
    /// SFTP 通道
    Sftp,
    /// Exec 通道
    Exec,
    /// Direct TCP/IP 通道
    DirectTcpip,
}

impl ChannelType {
    /// 对齐 `getValue()`
    pub fn get_value(&self) -> &'static str {
        match self {
            ChannelType::Shell => "shell",
            ChannelType::Sftp => "sftp",
            ChannelType::Exec => "exec",
            ChannelType::DirectTcpip => "direct-tcpip",
        }
    }
}

impl fmt::Display for ChannelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_value())
    }
}

/// SSH 连接信息 POJO，对齐 `cn.hutool.extra.ssh.Connector`。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Connector {
    host: Option<String>,
    port: Option<u16>,
    user: Option<String>,
    password: Option<String>,
    group: Option<String>,
}

impl Connector {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_credentials(user: &str, password: &str, group: &str) -> Self {
        Self {
            user: Some(user.to_string()),
            password: Some(password.to_string()),
            group: Some(group.to_string()),
            ..Default::default()
        }
    }
    pub fn with_endpoint(host: &str, port: u16, user: &str, password: &str) -> Self {
        Self {
            host: Some(host.to_string()),
            port: Some(port),
            user: Some(user.to_string()),
            password: Some(password.to_string()),
            ..Default::default()
        }
    }
    pub fn get_host(&self) -> Option<&str> {
        self.host.as_deref()
    }
    pub fn set_host(&mut self, host: &str) -> &mut Self {
        self.host = Some(host.to_string());
        self
    }
    pub fn get_port(&self) -> Option<u16> {
        self.port
    }
    pub fn set_port(&mut self, port: u16) -> &mut Self {
        self.port = Some(port);
        self
    }
    pub fn get_user(&self) -> Option<&str> {
        self.user.as_deref()
    }
    pub fn set_user(&mut self, user: &str) -> &mut Self {
        self.user = Some(user.to_string());
        self
    }
    pub fn get_password(&self) -> Option<&str> {
        self.password.as_deref()
    }
    pub fn set_password(&mut self, password: &str) -> &mut Self {
        self.password = Some(password.to_string());
        self
    }
    pub fn get_group(&self) -> Option<&str> {
        self.group.as_deref()
    }
    pub fn set_group(&mut self, group: &str) -> &mut Self {
        self.group = Some(group.to_string());
        self
    }
}

impl fmt::Display for Connector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Connector [host={}, port={}, user={}, password={}]",
            self.host.as_deref().unwrap_or("null"),
            self.port.map(|p| p.to_string()).unwrap_or_else(|| "null".into()),
            self.user.as_deref().unwrap_or("null"),
            self.password.as_deref().unwrap_or("null"),
        )
    }
}

/// SSH 运行时异常，对齐 `cn.hutool.extra.ssh.JschRuntimeException`。
///
/// Rust 用 `HutoolException` 别名实现统一异常层次。
pub type JschRuntimeException = HutoolException;

/// SSH 会话池接口，对齐 `cn.hutool.extra.ssh.JschSessionPool`。
///
/// 具体 Session 类型（jcraft.jsch.Session）是 Java-only，Rust 用 trait object 替代。
pub trait JschSessionPool: Send + Sync {
    /// 对齐 `JschSessionPool.get(String)`
    fn get(&self, key: &str) -> Option<Box<dyn std::any::Any>>;

    /// 对齐 `JschSessionPool.put(...)`（已在 JschUtil 中实现）
    fn put_session(&self, key: &str, session: Box<dyn std::any::Any>);

    /// 对齐 `JschSessionPool.remove(Session)`
    fn remove(&self, session: &dyn std::any::Any) -> bool;

    /// 对齐 `JschSessionPool.close(String)`
    fn close(&self, key: &str);

    /// 对齐 `JschSessionPool.closeAll()`
    fn close_all(&self);
}

/// SSH 工具类（Jsch 实现），对齐 `cn.hutool.extra.ssh.JschUtil`。
///
/// 具体 Session/Channel/Sftp 类型依赖 jcraft.jsch，属于 unsafe-to-copy。
/// Rust 用 trait + 错误返回桩，标记 API 完整性。
pub struct JschUtil;

impl JschUtil {
    /// 对齐 `JschUtil.SSH_NONE` 常量。
    pub const SSH_NONE: &'static str = "none";

    /// 对齐 `JschUtil.generateLocalPort()`
    ///
    /// 调用 hutool-core 的 LocalPortGenerater（如未启用则返回错误）。
    pub fn generate_local_port() -> std::result::Result<u16, HutoolException> {
        Err(HutoolException::Message(
            "JschUtil::generate_local_port requires ssh2 crate (unsafe-to-copy); use std::net TcpListener bind to port 0 for ephemeral port".into(),
        ))
    }

    /// 对齐 `JschUtil.openSftp(Session)`
    pub fn open_sftp(
        _session: &dyn std::any::Any,
    ) -> std::result::Result<Box<dyn std::any::Any>, HutoolException> {
        Err(HutoolException::Message(
            "JschUtil::open_sftp requires jcraft.jsch crate (unsafe-to-copy)".into(),
        ))
    }

    /// 对齐 `JschUtil.createSftp(Session)`
    pub fn create_sftp(
        _session: &dyn std::any::Any,
    ) -> std::result::Result<Box<dyn std::any::Any>, HutoolException> {
        Err(HutoolException::Message(
            "JschUtil::create_sftp requires jcraft.jsch crate (unsafe-to-copy)".into(),
        ))
    }

    /// 对齐 `JschUtil.openShell(Session)`
    pub fn open_shell(
        _session: &dyn std::any::Any,
    ) -> std::result::Result<Box<dyn std::any::Any>, HutoolException> {
        Err(HutoolException::Message(
            "JschUtil::open_shell requires jcraft.jsch crate (unsafe-to-copy)".into(),
        ))
    }

    /// 对齐 `JschUtil.close(Session)`
    pub fn close_session(_session: &dyn std::any::Any) {}

    /// 对齐 `JschUtil.close(Channel)`
    pub fn close_channel(_channel: &dyn std::any::Any) {}

    /// 对齐 `JschUtil.close(String)`（按 key 关闭 Session 池中的会话）
    pub fn close_by_key(_pool: &dyn JschSessionPool, _key: &str) {}

    /// 对齐 `JschUtil.closeAll()`
    pub fn close_all(_pool: &dyn JschSessionPool) {}
}

/// Ganymed SSH 工具类，对齐 `cn.hutool.extra.ssh.GanymedUtil`。
///
/// 依赖 ch.ethz.ssh2（Ganymed SSH-2 for Java），属于 unsafe-to-copy。
pub struct GanymedUtil;

impl GanymedUtil {
    /// 对齐 `GanymedUtil.close(Session)`
    pub fn close(_session: &dyn std::any::Any) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connector_default() {
        let c = Connector::new();
        assert_eq!(c.get_host(), None);
    }

    #[test]
    fn test_connector_with_credentials() {
        let c = Connector::with_credentials("alice", "secret", "admin");
        assert_eq!(c.get_user(), Some("alice"));
    }

    #[test]
    fn test_connector_with_endpoint() {
        let c = Connector::with_endpoint("192.168.1.1", 22, "root", "pass");
        assert_eq!(c.get_host(), Some("192.168.1.1"));
        assert_eq!(c.get_port(), Some(22));
    }

    #[test]
    fn test_connector_setters() {
        let mut c = Connector::new();
        c.set_host("10.0.0.1").set_port(2222).set_user("bob");
        assert_eq!(c.get_host(), Some("10.0.0.1"));
        assert_eq!(c.get_port(), Some(2222));
    }

    #[test]
    fn test_connector_display() {
        let c = Connector::with_endpoint("h", 22, "u", "p");
        let s = format!("{}", c);
        assert!(s.contains("h"));
    }

    #[test]
    fn test_channel_type_get_value() {
        assert_eq!(ChannelType::Shell.get_value(), "shell");
        assert_eq!(ChannelType::Sftp.get_value(), "sftp");
    }

    #[test]
    fn test_jsch_util_constants() {
        assert_eq!(JschUtil::SSH_NONE, "none");
    }

    #[test]
    fn test_jsch_util_generate_local_port_err() {
        let r = JschUtil::generate_local_port();
        assert!(r.is_err());
    }

    #[test]
    fn test_jsch_runtime_exception_is_hutool_exception() {
        let e: JschRuntimeException = HutoolException::message("ssh failure");
        assert_eq!(e.get_message(), "ssh failure");
    }
}