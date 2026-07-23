//! 表达式引擎 facade，对齐 hutool 的 `cn.hutool.extra.expression.*`。
//!
//! 提供 ExpressionEngine trait + ExpressionUtil 静态门面 + 6 个构造器签名。
//! 各具体引擎（Aviator/JEXL/JfireEL/Mvel/QLExpress/Rhino/SpEL）需要外部 Java crate，
//! 属于 unsafe-to-copy，暂不在 hitool-extra 中实现。

use std::collections::HashMap;

use crate::HutoolException;

/// 表达式引擎抽象，对齐 `cn.hutool.extra.expression.ExpressionEngine`。
pub trait ExpressionEngine: Send + Sync {
    /// 在给定上下文（变量绑定）下执行表达式字符串。
    fn eval(
        &self,
        expression: &str,
        context: &HashMap<String, serde_json::Value>,
        allow_class_set: &[&str],
    ) -> Result<serde_json::Value, HutoolException>;

    /// 获取底层原始引擎（Java 返回 `Object`）
    fn raw_engine(&self) -> Option<&dyn std::any::Any>;
}

/// 表达式工具类，对齐 `cn.hutool.extra.expression.ExpressionUtil`。
pub struct ExpressionUtil;

impl ExpressionUtil {
    /// 对齐 `ExpressionUtil.eval(String expression, Map<String, Object> context)`
    pub fn eval(
        expression: &str,
        context: &HashMap<String, serde_json::Value>,
    ) -> Result<serde_json::Value, HutoolException> {
        Self::eval_with_classes(expression, context, &[])
    }

    /// 对齐 `ExpressionUtil.eval(String, Map, Collection<Class<?>>)`
    pub fn eval_with_classes(
        _expression: &str,
        _context: &HashMap<String, serde_json::Value>,
        _allow_class_set: &[&str],
    ) -> Result<serde_json::Value, HutoolException> {
        // Phase 1.4: 等具体 engine（spel-engine 等）实现后填充
        Err(HutoolException::Message(
            "ExpressionUtil::eval requires a concrete ExpressionEngine implementation".into(),
        ))
    }

    /// 对齐 `ExpressionUtil.getEngine()`：返回默认引擎实例
    pub fn get_engine() -> Result<Box<dyn ExpressionEngine>, HutoolException> {
        Err(HutoolException::Message(
            "ExpressionUtil::get_engine requires a concrete ExpressionEngine implementation".into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression_util_eval_not_implemented() {
        let mut ctx = HashMap::new();
        ctx.insert("x".into(), serde_json::json!(1));
        let r = ExpressionUtil::eval("1 + 1", &ctx);
        assert!(r.is_err());
    }

    #[test]
    fn test_expression_util_get_engine_not_implemented() {
        let r = ExpressionUtil::get_engine();
        assert!(r.is_err());
    }
}