//! FTP POJO facade，对齐 hutool 的 `cn.hutool.extra.ftp.*`。
//!
//! **仅实现 POJO 部分**（FtpConfig、FtpMode、FtpException、AbstractFtp trait）。
//! 具体 FTP 协议操作（Ftp/SimpleFtpServer）需要外部 Java crate（Apache Commons Net /
//! Apache FtpServer），属于 unsafe-to-copy。

use std::fmt;

use crate::HutoolException;

/// FTP 连接模式，对齐 `cn.hutool.extra.ftp.FtpMode`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FtpMode {
    /// 主动模式（PORT）
    #[default]
    Active,
    /// 被动模式（PASV）
    Passive,
}

impl fmt::Display for FtpMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FtpMode::Active => write!(f, "Active"),
            FtpMode::Passive => write!(f, "Passive"),
        }
    }
}

/// FTP 配置 POJO，对齐 `cn.hutool.extra.ftp.FtpConfig`。
///
/// 所有 setter 返回 `&mut Self` 以匹配 Java 的链式 API（`FtpConfig.setXxx` 返回 this）。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FtpConfig {
    host: Option<String>,
    port: u16,
    user: Option<String>,
    password: Option<String>,
    charset: Option<String>,
    /// 连接超时（毫秒）
    connection_timeout: i64,
    /// Socket 超时（毫秒）
    so_timeout: i64,
    server_language_code: Option<String>,
    system_key: Option<String>,
}

impl FtpConfig {
    /// 对齐 `FtpConfig.create()` 静态工厂
    pub fn create() -> Self {
        Self::default()
    }

    /// 对齐 `FtpConfig()` 默认构造
    pub fn new() -> Self {
        Self::default()
    }

    /// 对齐 `FtpConfig(String host, int port, String user, String password, Charset charset)`
    pub fn new_simple(
        host: &str,
        port: u16,
        user: &str,
        password: &str,
        charset: &str,
    ) -> Self {
        Self {
            host: Some(host.to_string()),
            port,
            user: Some(user.to_string()),
            password: Some(password.to_string()),
            charset: Some(charset.to_string()),
            ..Default::default()
        }
    }

    /// 对齐 `FtpConfig(String, int, String, String, Charset, String, String)` 全参数构造
    pub fn new_full(
        host: &str,
        port: u16,
        user: &str,
        password: &str,
        charset: &str,
        server_language_code: &str,
        system_key: &str,
    ) -> Self {
        Self {
            host: Some(host.to_string()),
            port,
            user: Some(user.to_string()),
            password: Some(password.to_string()),
            charset: Some(charset.to_string()),
            server_language_code: Some(server_language_code.to_string()),
            system_key: Some(system_key.to_string()),
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

    pub fn get_port(&self) -> u16 {
        self.port
    }
    pub fn set_port(&mut self, port: u16) -> &mut Self {
        self.port = port;
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

    pub fn get_charset(&self) -> Option<&str> {
        self.charset.as_deref()
    }
    pub fn set_charset(&mut self, charset: &str) -> &mut Self {
        self.charset = Some(charset.to_string());
        self
    }

    pub fn get_connection_timeout(&self) -> i64 {
        self.connection_timeout
    }
    pub fn set_connection_timeout(&mut self, timeout_ms: i64) -> &mut Self {
        self.connection_timeout = timeout_ms;
        self
    }

    pub fn get_so_timeout(&self) -> i64 {
        self.so_timeout
    }
    pub fn set_so_timeout(&mut self, timeout_ms: i64) -> &mut Self {
        self.so_timeout = timeout_ms;
        self
    }

    pub fn get_server_language_code(&self) -> Option<&str> {
        self.server_language_code.as_deref()
    }
    pub fn set_server_language_code(&mut self, code: &str) -> &mut Self {
        self.server_language_code = Some(code.to_string());
        self
    }

    pub fn get_system_key(&self) -> Option<&str> {
        self.system_key.as_deref()
    }
    pub fn set_system_key(&mut self, key: &str) -> &mut Self {
        self.system_key = Some(key.to_string());
        self
    }
}

/// FTP 异常，对齐 `cn.hutool.extra.ftp.FtpException`（统一委托到 HutoolException）。
pub type FtpException = HutoolException;

/// 抽象 FTP 操作 trait，对齐 `cn.hutool.extra.ftp.AbstractFtp`。
///
/// 具体 Ftp / SshjSftp 实现需要外部 Java crate，属于 unsafe-to-copy。
pub trait AbstractFtp: Send + Sync {
    /// 对齐 `AbstractFtp.reconnectIfTimeout()`
    fn reconnect_if_timeout(&self) -> std::result::Result<(), HutoolException>;

    /// 对齐 `AbstractFtp.cd(String)`
    fn cd(&mut self, dir: &str) -> std::result::Result<(), HutoolException>;

    /// 对齐 `AbstractFtp.toParent()`
    fn to_parent(&mut self) -> std::result::Result<(), HutoolException>;

    /// 对齐 `AbstractFtp.pwd()`
    fn pwd(&self) -> std::result::Result<String, HutoolException>;

    /// 对齐 `AbstractFtp.isDir(String)`
    fn is_dir(&self, dir: &str) -> bool;

    /// 对齐 `AbstractFtp.mkdir(String)`
    fn mkdir(&mut self, dir: &str) -> std::result::Result<(), HutoolException>;

    /// 对齐 `AbstractFtp.exist(String)`
    fn exist(&self, path: &str) -> bool;

    /// 对齐 `AbstractFtp.delFile(String)`
    fn del_file(&mut self, path: &str) -> std::result::Result<(), HutoolException>;

    /// 对齐 `AbstractFtp.delDir(String)`
    fn del_dir(&mut self, dir: &str) -> std::result::Result<(), HutoolException>;

    /// 对齐 `AbstractFtp.mkDirs(String)`
    fn mk_dirs(&mut self, dir: &str) -> std::result::Result<(), HutoolException>;

    /// 对齐 `AbstractFtp.upload(...)`
    fn upload(&mut self, dest: &str, _data: &[u8]) -> std::result::Result<(), HutoolException>;

    /// 对齐 `AbstractFtp.recursiveDownloadFolder(String, File)`
    fn recursive_download_folder(
        &mut self,
        remote: &str,
        local: &std::path::Path,
    ) -> std::result::Result<(), HutoolException>;

    /// 对齐 `AbstractFtp.rename(String, String)`
    fn rename(&mut self, from: &str, to: &str) -> std::result::Result<(), HutoolException>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ftp_config_default() {
        let cfg = FtpConfig::new();
        assert_eq!(cfg.get_host(), None);
        assert_eq!(cfg.get_port(), 0);
    }

    #[test]
    fn test_ftp_config_create() {
        let cfg = FtpConfig::create();
        assert_eq!(cfg.get_host(), None);
    }

    #[test]
    fn test_ftp_config_new_simple() {
        let cfg = FtpConfig::new_simple("1.2.3.4", 21, "user", "pass", "UTF-8");
        assert_eq!(cfg.get_host(), Some("1.2.3.4"));
        assert_eq!(cfg.get_port(), 21);
        assert_eq!(cfg.get_user(), Some("user"));
        assert_eq!(cfg.get_charset(), Some("UTF-8"));
    }

    #[test]
    fn test_ftp_config_full() {
        let cfg = FtpConfig::new_full("h", 21, "u", "p", "UTF-8", "en", "UNIX");
        assert_eq!(cfg.get_server_language_code(), Some("en"));
        assert_eq!(cfg.get_system_key(), Some("UNIX"));
    }

    #[test]
    fn test_ftp_config_builder_chain() {
        let mut cfg = FtpConfig::new();
        cfg.set_host("h").set_port(2222).set_user("u");
        assert_eq!(cfg.get_host(), Some("h"));
        assert_eq!(cfg.get_port(), 2222);
        assert_eq!(cfg.get_user(), Some("u"));
    }

    #[test]
    fn test_ftp_config_timeouts() {
        let mut cfg = FtpConfig::new();
        cfg.set_connection_timeout(5000).set_so_timeout(10000);
        assert_eq!(cfg.get_connection_timeout(), 5000);
        assert_eq!(cfg.get_so_timeout(), 10000);
    }

    #[test]
    fn test_ftp_mode_display() {
        assert_eq!(FtpMode::Active.to_string(), "Active");
        assert_eq!(FtpMode::Passive.to_string(), "Passive");
    }
}