//! 对齐: `cn.hutool.core.lang.ClassScanner`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/ClassScanner.java
//!
//! Hutool 的 `ClassScanner` Java 类型,等待完整实现。
//! 状态: 对齐桩(对象/方法/参数已对齐),等待 `hitool-core` 内部继续迁移。

use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.lang.ClassScanner`
#[derive(Debug, Clone, Default)]
pub struct ClassScanner;

impl ClassScanner {
    /// 对齐 Java: `ClassScanner.scanAllPackageByAnnotation(String packageName, Class<? extends Annotation> annotationClass)`
    #[allow(clippy::too_many_arguments)]
    pub fn scanAllPackageByAnnotation(&str packageName, Class<? extends Annotation> annotationClass) -> Result<Set<Class<?>>> {
        Err(CoreError::PendingEngine("ClassScanner::scanAllPackageByAnnotation (waiting for full impl)"))
    }
    /// 对齐 Java: `ClassScanner.scanPackageByAnnotation(String packageName, Class<? extends Annotation> annotationClass)`
    #[allow(clippy::too_many_arguments)]
    pub fn scanPackageByAnnotation(&str packageName, Class<? extends Annotation> annotationClass) -> Result<Set<Class<?>>> {
        Err(CoreError::PendingEngine("ClassScanner::scanPackageByAnnotation (waiting for full impl)"))
    }
    /// 对齐 Java: `ClassScanner.scanAllPackageBySuper(String packageName, Class<?> superClass)`
    #[allow(clippy::too_many_arguments)]
    pub fn scanAllPackageBySuper(&str packageName, Class<?> superClass) -> Result<Set<Class<?>>> {
        Err(CoreError::PendingEngine("ClassScanner::scanAllPackageBySuper (waiting for full impl)"))
    }
    /// 对齐 Java: `ClassScanner.scanPackageBySuper(String packageName, Class<?> superClass)`
    #[allow(clippy::too_many_arguments)]
    pub fn scanPackageBySuper(&str packageName, Class<?> superClass) -> Result<Set<Class<?>>> {
        Err(CoreError::PendingEngine("ClassScanner::scanPackageBySuper (waiting for full impl)"))
    }
    /// 对齐 Java: `ClassScanner.scanAllPackage()`
    #[allow(clippy::too_many_arguments)]
    pub fn scanAllPackage() -> Result<Set<Class<?>>> {
        Err(CoreError::PendingEngine("ClassScanner::scanAllPackage (waiting for full impl)"))
    }
    /// 对齐 Java: `ClassScanner.scanPackage()`
    #[allow(clippy::too_many_arguments)]
    pub fn scanPackage() -> Result<Set<Class<?>>> {
        Err(CoreError::PendingEngine("ClassScanner::scanPackage (waiting for full impl)"))
    }
    /// 对齐 Java: `ClassScanner.setIgnoreLoadError(boolean ignoreLoadError)`
    #[allow(clippy::too_many_arguments)]
    pub fn setIgnoreLoadError(bool ignoreLoadError) -> Result<ClassScanner> {
        Err(CoreError::PendingEngine("ClassScanner::setIgnoreLoadError (waiting for full impl)"))
    }
    /// 对齐 Java: `ClassScanner.scan()`
    #[allow(clippy::too_many_arguments)]
    pub fn scan() -> Result<Set<Class<?>>> {
        Err(CoreError::PendingEngine("ClassScanner::scan (waiting for full impl)"))
    }
    /// 对齐 Java: `ClassScanner.setInitialize(boolean initialize)`
    #[allow(clippy::too_many_arguments)]
    pub fn setInitialize(bool initialize) -> Result<()> {
        Err(CoreError::PendingEngine("ClassScanner::setInitialize (waiting for full impl)"))
    }
    /// 对齐 Java: `ClassScanner.setClassLoader(ClassLoader classLoader)`
    #[allow(clippy::too_many_arguments)]
    pub fn setClassLoader(ClassLoader classLoader) -> Result<()> {
        Err(CoreError::PendingEngine("ClassScanner::setClassLoader (waiting for full impl)"))
    }
    /// 对齐 Java: `ClassScanner.getClassesOfLoadError()`
    #[allow(clippy::too_many_arguments)]
    pub fn getClassesOfLoadError() -> Result<Set<String>> {
        Err(CoreError::PendingEngine("ClassScanner::getClassesOfLoadError (waiting for full impl)"))
    }
}
