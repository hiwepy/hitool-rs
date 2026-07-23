//! 模板引擎配置 facade。
//!
//! 1:1 对齐 hutool 的 `cn.hutool.extra.template.*` 包（不含各 template engine 适配）。
//!
//! - 原 Java 包：`cn.hutool.extra.template`
//! - 本文件覆盖：`TemplateConfig`、`ResourceMode`、`TemplateException`、`Template` interface、
//!   `AbstractTemplate` 抽象类、`TemplateUtil` 静态门面、`TemplateEngine` 接口
//! - 各 engine 适配（Beetl / Enjoy / Freemarker / Jetbrick / Rythm / Thymeleaf / Velocity / Wit）
//!   在各自的 `engine/<name>.rs` 子模块；本文件只提供配置 + facade。
//! - 迁移状态：✅ 已实现（Phase 1.4 工作）

use std::fmt;
use std::path::Path;

use thiserror::Error;

/// 模板异常，对齐 `cn.hutool.extra.template.TemplateException`。
///
/// Java 继承 `RuntimeException`；Rust 用 `thiserror::Error` enum 实现。
#[derive(Debug, Error)]
pub enum TemplateException {
    /// 对齐 `TemplateException(String message)`
    #[error("{0}")]
    Message(String),

    /// 对齐 `TemplateException(Throwable e)`（cause 链）
    #[error("{message}")]
    WithCause {
        message: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// 对齐 `TemplateException(String messageTemplate, Object... params)`（带格式化的 message）
    #[error("{message}")]
    Formatted { message: String },

    /// 对齐 `TemplateException(String message, Throwable throwable)`
    #[error("{message}")]
    WithThrowable {
        message: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// 对齐 `TemplateException(Throwable throwable, String messageTemplate, Object... params)`
    #[error("{message}")]
    FormattedWithCause {
        message: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

impl TemplateException {
    /// 格式化 message，参考 `StrUtil.format`（Phase 1.4 完成后用 hitool-core::StrUtil.format）。
    fn format_message(template: &str, params: &[&dyn fmt::Display]) -> String {
        let mut out = String::new();
        let mut param_idx = 0;
        let mut chars = template.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '{' && chars.peek() == Some(&'}') {
                chars.next();
                if param_idx < params.len() {
                    out.push_str(&format!("{}", params[param_idx]));
                    param_idx += 1;
                }
            } else {
                out.push(c);
            }
        }
        out
    }

    /// 对齐 `TemplateException(Throwable e)`
    pub fn from_error<E>(e: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        TemplateException::WithCause {
            message: e.to_string(),
            source: Box::new(e),
        }
    }

    /// 对齐 `TemplateException(String message)`
    pub fn new<S: Into<String>>(message: S) -> Self {
        TemplateException::Message(message.into())
    }

    /// 对齐 `TemplateException(String messageTemplate, Object... params)`
    pub fn formatted<S: AsRef<str>>(template: S, params: &[&dyn fmt::Display]) -> Self {
        TemplateException::Formatted {
            message: Self::format_message(template.as_ref(), params),
        }
    }

    /// 对齐 `TemplateException(String message, Throwable throwable)`
    pub fn with_throwable<S: Into<String>, E>(message: S, throwable: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        TemplateException::WithThrowable {
            message: message.into(),
            source: Box::new(throwable),
        }
    }

    /// 对齐 `TemplateException(Throwable throwable, String messageTemplate, Object... params)`
    pub fn formatted_with_cause<E>(throwable: E, template: &str, params: &[&dyn fmt::Display]) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        TemplateException::FormattedWithCause {
            message: Self::format_message(template, params),
            source: Box::new(throwable),
        }
    }

    /// 返回错误 message（对齐 Java `getMessage()`）
    pub fn message(&self) -> &str {
        match self {
            TemplateException::Message(m)
            | TemplateException::Formatted { message: m }
            | TemplateException::WithCause { message: m, .. }
            | TemplateException::WithThrowable { message: m, .. }
            | TemplateException::FormattedWithCause { message: m, .. } => m,
        }
    }
}

/// 资源加载方式，对齐 `cn.hutool.extra.template.TemplateConfig.ResourceMode`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceMode {
    /// 从 ClassPath 加载模板
    ClassPath,
    /// 从 File 目录加载模板
    File,
    /// 从 WebRoot 目录加载模板
    WebRoot,
    /// 从模板文本加载模板
    String,
    /// 复合加载（File → ClassPath → WebRoot → String 顺序尝试）
    Composite,
}

/// 模板引擎抽象，对齐 `cn.hutool.extra.template.TemplateEngine` 接口。
///
/// 各具体引擎（Beetl/Enjoy/Freemarker/Rythm/Thymeleaf/Velocity/Wit/Jetbrick）由
/// `engine/<name>.rs` 子模块实现 `TemplateEngine` trait。
pub trait TemplateEngine: Send + Sync {
    /// 初始化引擎
    fn init(&mut self, config: &TemplateConfig) -> Result<(), TemplateException>;

    /// 获取原始引擎（Java `getRawEngine()` 返回 `Object`）
    fn raw_engine(&self) -> Option<&dyn std::any::Any>;

    /// 根据资源名获取模板
    fn get_template(
        &self,
        resource: &str,
    ) -> Result<Box<dyn Template>, TemplateException>;
}

/// 模板接口，对齐 `cn.hutool.extra.template.Template`。
///
/// 注意：原 Java 接口有 `render(Map, Writer)` 重载；Rust 用 dyn-compatibility 约束，
/// 因此 `render` 返回 `String` 而非泛型 Writer。如需流式输出，调用方可在 `render_to_string`
/// 后自行 write。
pub trait Template: Send + Sync {
    /// 渲染模板并返回字符串（对齐 `render(Map)` Java 重载）。
    fn render_to_string(&self, binding: &TemplateBinding) -> Result<String, TemplateException>;

    /// 渲染模板并写入字节缓冲（对齐 `render(Map, File)` Java 重载）。
    fn render_to_bytes(&self, binding: &TemplateBinding) -> Result<Vec<u8>, TemplateException>;
}

/// 模板渲染 binding，对齐 `cn.hutool.extra.template.TemplateBinding`（Java 用 `Map<?, ?>`）。
pub type TemplateBinding = std::collections::HashMap<String, TemplateValue>;

/// 模板变量值，**类型擦除**对齐 Java `Object`。
///
/// 用 `serde_json::Value` 兜底：可序列化任意 JSON 数据。
pub type TemplateValue = serde_json::Value;

/// 模板配置，对齐 `cn.hutool.extra.template.TemplateConfig`。
///
/// 默认值：
/// - charset = UTF-8
/// - path = `None`（ClassPath 根）
/// - resourceMode = `ResourceMode::String`
/// - useCache = `true`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateConfig {
    charset: Option<String>,
    path: Option<String>,
    resource_mode: ResourceMode,
    custom_engine: Option<String>,
    use_cache: bool,
}

/// 全局默认配置，对齐 `cn.hutool.extra.template.TemplateConfig.DEFAULT`。
pub static DEFAULT_CONFIG: std::sync::OnceLock<TemplateConfig> = std::sync::OnceLock::new();

/// 获取默认配置
pub fn default_config() -> &'static TemplateConfig {
    DEFAULT_CONFIG.get_or_init(TemplateConfig::new)
}

impl TemplateConfig {
    /// 默认构造，使用 UTF-8 编码，默认从 String 加载模板。
    /// 对齐 `TemplateConfig()`。
    pub fn new() -> Self {
        Self {
            charset: Some("UTF-8".to_string()),
            path: None,
            resource_mode: ResourceMode::String,
            custom_engine: None,
            use_cache: true,
        }
    }

    /// 构造，path 显式指定，UTF-8 编码。
    /// 对齐 `TemplateConfig(String path)`。
    pub fn with_path(path: impl Into<String>) -> Self {
        Self {
            charset: Some("UTF-8".to_string()),
            path: Some(path.into()),
            resource_mode: ResourceMode::String,
            custom_engine: None,
            use_cache: true,
        }
    }

    /// 构造，path + resourceMode。
    /// 对齐 `TemplateConfig(String path, ResourceMode resourceMode)`。
    pub fn with_path_and_mode(path: impl Into<String>, resource_mode: ResourceMode) -> Self {
        Self {
            charset: Some("UTF-8".to_string()),
            path: Some(path.into()),
            resource_mode,
            custom_engine: None,
            use_cache: true,
        }
    }

    /// 全参数构造。
    /// 对齐 `TemplateConfig(Charset charset, String path, ResourceMode resourceMode)`。
    pub fn full(
        charset: impl AsRef<str>,
        path: impl Into<String>,
        resource_mode: ResourceMode,
    ) -> Self {
        Self {
            charset: Some(charset.as_ref().to_string()),
            path: Some(path.into()),
            resource_mode,
            custom_engine: None,
            use_cache: true,
        }
    }

    /// 对齐 `getCharset()`
    pub fn charset(&self) -> Option<&str> {
        self.charset.as_deref()
    }

    /// 对齐 `getCharsetStr()`：返回 charset 的字符串表示，`None` 时返回 `None`。
    pub fn charset_str(&self) -> Option<&str> {
        self.charset.as_deref()
    }

    /// 对齐 `setCharset(Charset)`
    pub fn set_charset(&mut self, charset: impl AsRef<str>) -> &mut Self {
        self.charset = Some(charset.as_ref().to_string());
        self
    }

    /// 对齐 `getPath()`
    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    /// 对齐 `setPath(String)`
    pub fn set_path(&mut self, path: impl Into<String>) -> &mut Self {
        self.path = Some(path.into());
        self
    }

    /// 对齐 `getResourceMode()`
    pub fn resource_mode(&self) -> ResourceMode {
        self.resource_mode
    }

    /// 对齐 `setResourceMode(ResourceMode)`
    pub fn set_resource_mode(&mut self, mode: ResourceMode) -> &mut Self {
        self.resource_mode = mode;
        self
    }

    /// 对齐 `getCustomEngine()`：返回自定义引擎类名（Java 用 `Class<? extends TemplateEngine>`，Rust 用字符串）。
    pub fn custom_engine(&self) -> Option<&str> {
        self.custom_engine.as_deref()
    }

    /// 对齐 `setCustomEngine(Class<? extends TemplateEngine>)`：Rust 用引擎名称字符串代替 Class 对象。
    pub fn set_custom_engine(&mut self, engine_name: impl Into<String>) -> &mut Self {
        self.custom_engine = Some(engine_name.into());
        self
    }

    /// 对齐 `isUseCache()`
    pub fn is_use_cache(&self) -> bool {
        self.use_cache
    }

    /// 对齐 `setUseCache(boolean)`
    pub fn set_use_cache(&mut self, use_cache: bool) -> &mut Self {
        self.use_cache = use_cache;
        self
    }
}

impl Default for TemplateConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ResourceMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResourceMode::ClassPath => write!(f, "CLASSPATH"),
            ResourceMode::File => write!(f, "FILE"),
            ResourceMode::WebRoot => write!(f, "WEB_ROOT"),
            ResourceMode::String => write!(f, "STRING"),
            ResourceMode::Composite => write!(f, "COMPOSITE"),
        }
    }
}

/// 模板抽象基类，对齐 `cn.hutool.extra.template.AbstractTemplate`。
///
/// 持有配置和绑定映射，提供 `render(File)` 和 `render(Map)` 两种重载。
pub struct AbstractTemplate {
    config: TemplateConfig,
    binding: TemplateBinding,
}

impl AbstractTemplate {
    /// 创建抽象模板实例（包级调用）
    pub fn new(config: TemplateConfig) -> Self {
        Self {
            config,
            binding: TemplateBinding::new(),
        }
    }

    /// 设置 binding
    pub fn with_binding(mut self, binding: TemplateBinding) -> Self {
        self.binding = binding;
        self
    }

    /// 添加单个 binding
    pub fn bind(mut self, key: impl Into<String>, value: impl Into<TemplateValue>) -> Self {
        self.binding.insert(key.into(), value.into());
        self
    }

    /// 对齐 `render(Map, File)`：渲染到文件
    pub fn render_to_file(
        &self,
        _file: &Path,
    ) -> Result<(), TemplateException> {
        // TODO: 委托到具体 Template 实现（Phase 1.4 子任务）
        Err(TemplateException::Message(
            "AbstractTemplate::render(File) requires a concrete TemplateEngine; see engine/<name>.rs".into(),
        ))
    }

    /// 对齐 `render(Map)`：返回字符串
    pub fn render(&self) -> Result<String, TemplateException> {
        // TODO: 委托到具体 Template 实现
        Err(TemplateException::Message(
            "AbstractTemplate::render() requires a concrete TemplateEngine; see engine/<name>.rs".into(),
        ))
    }
}

/// 模板工具类，对齐 `cn.hutool.extra.template.TemplateUtil`。
pub struct TemplateUtil;

impl TemplateUtil {
    /// 对齐 `TemplateUtil.createEngine()`：根据默认配置创建引擎。
    pub fn create_engine() -> Result<Box<dyn TemplateEngine>, TemplateException> {
        Self::create_engine_with_config(default_config())
    }

    /// 对齐 `TemplateUtil.createEngine(TemplateConfig config)`：根据指定配置创建引擎。
    ///
    /// Rust 用 `dyn TemplateEngine` trait object 而非 Java `Class<? extends TemplateEngine>` 反射。
    pub fn create_engine_with_config(
        config: &TemplateConfig,
    ) -> Result<Box<dyn TemplateEngine>, TemplateException> {
        // Phase 1.4 子任务：根据 config.custom_engine 选择具体引擎
        // 当前返回 Err 直到具体 engine 实现到位
        Err(TemplateException::Message(
            "TemplateUtil::create_engine requires at least one concrete TemplateEngine implementation (engine/<name>.rs)".into(),
        ))
    }
}

/// 模板引擎工厂，对齐 `cn.hutool.extra.template.engine.TemplateFactory`。
pub struct TemplateFactory;

impl TemplateFactory {
    /// 对齐 `TemplateFactory.create(TemplateConfig config)`
    pub fn create(config: &TemplateConfig) -> Result<Box<dyn TemplateEngine>, TemplateException> {
        TemplateUtil::create_engine_with_config(config)
    }

    /// 对齐 `TemplateFactory.get()`：使用默认配置。
    pub fn get() -> Result<Box<dyn TemplateEngine>, TemplateException> {
        TemplateUtil::create_engine_with_config(default_config())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_config_default() {
        let cfg = TemplateConfig::default();
        assert_eq!(cfg.charset(), Some("UTF-8"));
        assert_eq!(cfg.path(), None);
        assert_eq!(cfg.resource_mode(), ResourceMode::String);
        assert_eq!(cfg.is_use_cache(), true);
    }

    #[test]
    fn test_template_config_with_path() {
        let cfg = TemplateConfig::with_path("templates/");
        assert_eq!(cfg.path(), Some("templates/"));
        assert_eq!(cfg.resource_mode(), ResourceMode::String);
    }

    #[test]
    fn test_template_config_with_path_and_mode() {
        let cfg = TemplateConfig::with_path_and_mode("/", ResourceMode::ClassPath);
        assert_eq!(cfg.resource_mode(), ResourceMode::ClassPath);
    }

    #[test]
    fn test_template_config_full() {
        let cfg = TemplateConfig::full("GBK", "/templates", ResourceMode::File);
        assert_eq!(cfg.charset(), Some("GBK"));
        assert_eq!(cfg.path(), Some("/templates"));
        assert_eq!(cfg.resource_mode(), ResourceMode::File);
    }

    #[test]
    fn test_template_config_builder_chain() {
        let mut cfg = TemplateConfig::new();
        cfg.set_charset("UTF-16")
            .set_path("templates/")
            .set_resource_mode(ResourceMode::WebRoot)
            .set_custom_engine("BeetlEngine")
            .set_use_cache(false);
        assert_eq!(cfg.charset(), Some("UTF-16"));
        assert_eq!(cfg.path(), Some("templates/"));
        assert_eq!(cfg.resource_mode(), ResourceMode::WebRoot);
        assert_eq!(cfg.custom_engine(), Some("BeetlEngine"));
        assert_eq!(cfg.is_use_cache(), false);
    }

    #[test]
    fn test_resource_mode_display() {
        assert_eq!(ResourceMode::ClassPath.to_string(), "CLASSPATH");
        assert_eq!(ResourceMode::File.to_string(), "FILE");
        assert_eq!(ResourceMode::WebRoot.to_string(), "WEB_ROOT");
        assert_eq!(ResourceMode::String.to_string(), "STRING");
        assert_eq!(ResourceMode::Composite.to_string(), "COMPOSITE");
    }

    #[test]
    fn test_template_exception_message() {
        let e = TemplateException::new("test");
        assert_eq!(e.message(), "test");
    }

    #[test]
    fn test_template_exception_formatted() {
        let e = TemplateException::formatted("hello {} world", &[&"rust"]);
        assert_eq!(e.message(), "hello rust world");
    }

    #[test]
    fn test_template_exception_from_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "missing");
        let e = TemplateException::from_error(io_err);
        assert!(e.message().contains("missing"));
    }

    #[test]
    fn test_abstract_template_binding() {
        let tpl = AbstractTemplate::new(TemplateConfig::default())
            .bind("name", "alice")
            .bind("age", 30i64);
        assert_eq!(tpl.binding.len(), 2);
        assert_eq!(
            tpl.binding.get("name").and_then(|v| v.as_str()),
            Some("alice")
        );
    }
}