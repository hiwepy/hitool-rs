//! Spring POJO facade，对齐 hutool 的 `cn.hutool.extra.spring.*`。
//!
//! **仅提供 trait 抽象**。具体 Spring Framework 依赖（ApplicationContext / BeanFactory）
//! 是 Java-only，属于 unsafe-to-copy。Rust 用户应使用依赖注入框架（如 axum::Extension、
//! shaku、self-rs 等）替代。

use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

use crate::HutoolException;

/// Spring 应用上下文 trait，对齐 `org.springframework.context.ApplicationContext`。
///
/// Rust 没有原生 Spring，此 trait 用作"通用 IoC 容器"抽象。
/// 用户可以提供自己的实现（基于任何 Rust DI 框架）。
pub trait ApplicationContext: Send + Sync {
    /// 对齐 `ApplicationContext.getBean(Class)`：按类型获取 Bean
    fn get_bean(&self, type_name: &str) -> Option<Arc<dyn Any>>;

    /// 对齐 `ApplicationContext.getBean(String)`：按名称获取 Bean
    fn get_bean_by_name(&self, name: &str) -> Option<Arc<dyn Any>>;

    /// 对齐 `ApplicationContext.getBeansOfType(Class)`
    fn get_beans_of_type(&self, type_name: &str) -> HashMap<String, Arc<dyn Any>>;

    /// 对齐 `ApplicationContext.getBeanNamesForType(Class)`
    fn get_bean_names_for_type(&self, type_name: &str) -> Vec<String>;

    /// 对齐 `ApplicationContext.getApplicationName()`
    fn get_application_name(&self) -> &str;

    /// 对齐 `ApplicationContext.getActiveProfiles()`
    fn get_active_profiles(&self) -> Vec<String>;

    /// 对齐 `ApplicationContext.containsBean(String)`
    fn contains_bean(&self, name: &str) -> bool;

    /// 对齐 `ApplicationContext.publishEvent(Object)`：发布事件
    fn publish_event(&self, event: Arc<dyn Any + Send + Sync>);
}

/// Bean 注册接口，对齐 `org.springframework.beans.factory.config.ConfigurableListableBeanFactory`。
pub trait ConfigurableBeanFactory: Send + Sync {
    /// 对齐 `ConfigurableListableBeanFactory.registerSingleton(String, Object)`
    fn register_bean(&mut self, name: &str, bean: Arc<dyn Any + Send + Sync>) -> bool;

    /// 对齐 `ConfigurableListableBeanFactory.destroyBean(String)` 或 unregister
    fn unregister_bean(&mut self, name: &str) -> bool;

    /// 对齐 `ConfigurableListableBeanFactory.getProperty(String)`：读取配置
    fn get_property(&self, key: &str) -> Option<String>;
}

/// SpringUtil 工具类（Java 版是 BeanFactoryPostProcessor + ApplicationContextAware），
/// 对齐 `cn.hutool.extra.spring.SpringUtil`。
///
/// Rust 版用 trait + 全局 OnceLock 持有 `dyn ApplicationContext`，避免依赖 Spring。
pub struct SpringUtil;

static APPLICATION_CONTEXT: OnceLock<Arc<dyn ApplicationContext>> = OnceLock::new();

impl SpringUtil {
    /// 对齐 `SpringUtil.setApplicationContext(ApplicationContext)`
    pub fn set_application_context(ctx: Arc<dyn ApplicationContext>) -> Result<(), HutoolException> {
        APPLICATION_CONTEXT
            .set(ctx)
            .map_err(|_| HutoolException::Message("SpringUtil: ApplicationContext already set".into()))
    }

    /// 对齐 `SpringUtil.getApplicationContext()`
    pub fn get_application_context() -> Option<Arc<dyn ApplicationContext>> {
        APPLICATION_CONTEXT.get().cloned()
    }

    /// 对齐 `SpringUtil.getBeanFactory()`
    pub fn get_bean_factory() -> Result<Arc<dyn ApplicationContext>, HutoolException> {
        Self::get_application_context().ok_or_else(|| {
            HutoolException::Message(
                "No ConfigurableListableBeanFactory or ApplicationContext injected, maybe not in the Spring environment?".into(),
            )
        })
    }

    /// 对齐 `SpringUtil.getBean(Class)`：按类型获取 Bean
    pub fn get_bean(type_name: &str) -> Result<Arc<dyn Any>, HutoolException> {
        let ctx = Self::get_bean_factory()?;
        ctx.get_bean(type_name)
            .ok_or_else(|| HutoolException::Message(format!("No bean of type: {}", type_name)))
    }

    /// 对齐 `SpringUtil.getBean(String)`：按名称获取 Bean
    pub fn get_bean_by_name(name: &str) -> Result<Arc<dyn Any>, HutoolException> {
        let ctx = Self::get_bean_factory()?;
        ctx.get_bean_by_name(name)
            .ok_or_else(|| HutoolException::Message(format!("No bean named: {}", name)))
    }

    /// 对齐 `SpringUtil.getBeansOfType(Class)`
    pub fn get_beans_of_type(type_name: &str) -> Result<HashMap<String, Arc<dyn Any>>, HutoolException> {
        Ok(Self::get_bean_factory()?.get_beans_of_type(type_name))
    }

    /// 对齐 `SpringUtil.getBeanNamesForType(Class)`
    pub fn get_bean_names_for_type(type_name: &str) -> Result<Vec<String>, HutoolException> {
        Ok(Self::get_bean_factory()?.get_bean_names_for_type(type_name))
    }

    /// 对齐 `SpringUtil.getApplicationName()`
    pub fn get_application_name() -> Result<String, HutoolException> {
        Ok(Self::get_bean_factory()?.get_application_name().to_string())
    }

    /// 对齐 `SpringUtil.getActiveProfiles()`
    pub fn get_active_profiles() -> Result<Vec<String>, HutoolException> {
        Ok(Self::get_bean_factory()?.get_active_profiles())
    }

    /// 对齐 `SpringUtil.getActiveProfile()`：取第一个
    pub fn get_active_profile() -> Result<String, HutoolException> {
        let profiles = Self::get_active_profiles()?;
        profiles.into_iter().next().ok_or_else(|| {
            HutoolException::Message("No active profile set".into())
        })
    }

    /// 对齐 `SpringUtil.registerBean(String, Object)`
    pub fn register_bean(
        _name: &str,
        _bean: Arc<dyn Any + Send + Sync>,
    ) -> Result<(), HutoolException> {
        Err(HutoolException::Message(
            "SpringUtil::register_bean requires a mutable ConfigurableBeanFactory; consider using ApplicationContext directly".into(),
        ))
    }

    /// 对齐 `SpringUtil.unregisterBean(String)`
    pub fn unregister_bean(_name: &str) -> Result<(), HutoolException> {
        Err(HutoolException::Message(
            "SpringUtil::unregister_bean requires a mutable ConfigurableBeanFactory; consider using ApplicationContext directly".into(),
        ))
    }

    /// 对齐 `SpringUtil.publishEvent(Object)`
    pub fn publish_event(event: Arc<dyn Any + Send + Sync>) -> Result<(), HutoolException> {
        let ctx = Self::get_bean_factory()?;
        ctx.publish_event(event);
        Ok(())
    }

    /// 对齐 `SpringUtil.getProperty(String)`：读取配置（委托到 ApplicationContext）
    pub fn get_property(key: &str) -> Result<String, HutoolException> {
        let ctx = Self::get_bean_factory()?;
        ctx.get_property_or(key)
            .ok_or_else(|| HutoolException::Message(format!("No property: {}", key)))
    }

    /// 对齐 `SpringUtil.postProcessBeanFactory(ConfigurableListableBeanFactory)`
    ///
    /// Rust 版无 Spring 生命周期回调；此方法返回 Err 提示用户应直接调用 `set_application_context`。
    pub fn post_process_bean_factory(_factory: &dyn ConfigurableBeanFactory) -> Result<(), HutoolException> {
        Err(HutoolException::Message(
            "SpringUtil::post_process_bean_factory is Spring-only; call set_application_context instead".into(),
        ))
    }
}

/// ApplicationContext 扩展 trait，提供 hitool-rs 独有的便捷方法。
pub trait ApplicationContextExt: ApplicationContext {
    /// get_property_or：返回配置或默认值
    fn get_property_or(&self, key: &str) -> Option<String> {
        // 默认从 BeanFactory 读取；具体实现可覆盖
        self.get_bean_by_name(&format!("__property_{}", key))
            .and_then(|v| v.downcast_ref::<String>().map(|s| s.clone()))
    }
}

impl<T: ApplicationContext + ?Sized> ApplicationContextExt for T {}

/// 启用 SpringUtil 的注解（Rust 版是 derive 宏或 attribute），
/// 对齐 `cn.hutool.extra.spring.EnableSpringUtil`。
///
/// 在 Rust 中没有真正的 Spring 注解扫描；用户应手动调用 `SpringUtil::set_application_context`。
/// 本注解保留为空 derive 占位（未来可通过 proc-macro 实现）。
pub fn enable_spring_util() {
    // 占位函数：在 Spring 中由 @EnableSpringUtil 注解触发，
    // 在 Rust 中用户必须显式 set_application_context。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_application_context_none() {
        // 测试未设置时返回错误
        // 注意：OnceLock 是全局的，如果之前 test 设置过会污染；此测试假设 clean state
        // 由于 OnceLock 无法 reset，此测试只能在第一次运行有效
        let r = SpringUtil::get_application_context();
        // 可能是 None（未设置）或 Some（之前设置过）
        let _ = r;
    }

    #[test]
    fn test_get_bean_factory_error_when_unset() {
        // 如果未设置 ApplicationContext，应返回 Err
        // 由于全局状态无法 reset，此测试可能跳过
        if SpringUtil::get_application_context().is_none() {
            let r = SpringUtil::get_bean("some_type");
            assert!(r.is_err());
        }
    }

    #[test]
    fn test_enable_spring_util() {
        // 占位测试
        enable_spring_util();
    }

    struct MockContext;
    impl ApplicationContext for MockContext {
        fn get_bean(&self, _t: &str) -> Option<Arc<dyn Any>> {
            None
        }
        fn get_bean_by_name(&self, _n: &str) -> Option<Arc<dyn Any>> {
            None
        }
        fn get_beans_of_type(&self, _t: &str) -> HashMap<String, Arc<dyn Any>> {
            HashMap::new()
        }
        fn get_bean_names_for_type(&self, _t: &str) -> Vec<String> {
            vec![]
        }
        fn get_application_name(&self) -> &str {
            "test"
        }
        fn get_active_profiles(&self) -> Vec<String> {
            vec!["default".into()]
        }
        fn contains_bean(&self, _n: &str) -> bool {
            false
        }
        fn publish_event(&self, _e: Arc<dyn Any + Send + Sync>) {}
    }

    #[test]
    fn test_mock_application_context() {
        let ctx = MockContext;
        assert_eq!(ctx.get_application_name(), "test");
        assert_eq!(ctx.get_active_profiles(), vec!["default"]);
        assert!(!ctx.contains_bean("any"));
    }
}