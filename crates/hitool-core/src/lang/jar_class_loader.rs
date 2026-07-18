//! 对齐: `cn.hutool.core.lang.JarClassLoader`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/JarClassLoader.java
//!
//! Hutool 的 `JarClassLoader` Java 类型,等待完整实现。
//! 状态: 对齐桩(对象/方法/参数已对齐),等待 `hitool-core` 内部继续迁移。

use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.lang.JarClassLoader`
#[derive(Debug, Clone, Default)]
pub struct JarClassLoader;

impl JarClassLoader {
    /// 对齐 Java: `JarClassLoader.load(File dir)`
    #[allow(clippy::too_many_arguments)]
    pub fn load(File dir) -> Result<JarClassLoader> {
        Err(CoreError::PendingEngine("JarClassLoader::load (waiting for full impl)"))
    }
    /// 对齐 Java: `JarClassLoader.loadJar(File jarFile)`
    #[allow(clippy::too_many_arguments)]
    pub fn loadJar(File jarFile) -> Result<JarClassLoader> {
        Err(CoreError::PendingEngine("JarClassLoader::loadJar (waiting for full impl)"))
    }
    /// 对齐 Java: `JarClassLoader.loadJarToSystemClassLoader(File jarFile)`
    #[allow(clippy::too_many_arguments)]
    pub fn loadJarToSystemClassLoader(File jarFile) -> Result<URLClassLoader> {
        Err(CoreError::PendingEngine("JarClassLoader::loadJarToSystemClassLoader (waiting for full impl)"))
    }
    /// 对齐 Java: `JarClassLoader.addJar(File jarFileOrDir)`
    #[allow(clippy::too_many_arguments)]
    pub fn addJar(File jarFileOrDir) -> Result<JarClassLoader> {
        Err(CoreError::PendingEngine("JarClassLoader::addJar (waiting for full impl)"))
    }
    /// 对齐 Java: `JarClassLoader.addURL(URL url)`
    #[allow(clippy::too_many_arguments)]
    pub fn addURL(URL url) -> Result<()> {
        Err(CoreError::PendingEngine("JarClassLoader::addURL (waiting for full impl)"))
    }
}
