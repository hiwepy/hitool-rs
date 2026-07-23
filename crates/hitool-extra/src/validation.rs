//! Hutool bean-validation result types (no Jakarta Validator runtime).
//!
//! 对齐: `cn.hutool.extra.validation.BeanValidationResult`
//! 来源: hutool-extra/src/main/java/cn/hutool/extra/validation/BeanValidationResult.java
//!
//! `ValidationUtil` / Hibernate Validator remain planned — Java bean-validation SPI.

/// Single property validation failure (Hutool `BeanValidationResult.ErrorMessage`).
///
/// 对齐 Java 内部类: `cn.hutool.extra.validation.BeanValidationResult.ErrorMessage`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorMessage {
    /// Property path / field name.
    pub property_name: String,
    /// Human-readable message.
    pub message: String,
    /// Optional stringified invalid value.
    pub value: Option<String>,
}

impl ErrorMessage {
    /// Creates an error message (Hutool setters path).
    #[must_use]
    pub fn new(
        property_name: impl Into<String>,
        message: impl Into<String>,
        value: Option<String>,
    ) -> Self {
        Self {
            property_name: property_name.into(),
            message: message.into(),
            value,
        }
    }

    /// Sets the property name (Hutool `setPropertyName`).
    #[must_use]
    pub fn set_property_name(mut self, property_name: impl Into<String>) -> Self {
        self.property_name = property_name.into();
        self
    }

    /// Sets the message (Hutool `setMessage`).
    #[must_use]
    pub fn set_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    /// Sets the invalid value display (Hutool `setValue`).
    #[must_use]
    pub fn set_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
}

/// Aggregate bean validation outcome (Hutool `BeanValidationResult`).
///
/// 对齐 Java 类: `cn.hutool.extra.validation.BeanValidationResult`
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BeanValidationResult {
    success: bool,
    error_messages: Vec<ErrorMessage>,
}

impl BeanValidationResult {
    /// Creates a result with an explicit success flag (Hutool constructor).
    #[must_use]
    pub fn new(success: bool) -> Self {
        Self {
            success,
            error_messages: Vec::new(),
        }
    }

    /// Returns whether validation passed (Hutool `isSuccess`).
    #[must_use]
    pub fn is_success(&self) -> bool {
        self.success
    }

    /// Sets success flag (Hutool `setSuccess`).
    #[must_use]
    pub fn set_success(mut self, success: bool) -> Self {
        self.success = success;
        self
    }

    /// Returns error messages (Hutool `getErrorMessages`).
    #[must_use]
    pub fn error_messages(&self) -> &[ErrorMessage] {
        &self.error_messages
    }

    /// Replaces error messages (Hutool `setErrorMessages`).
    #[must_use]
    pub fn set_error_messages(mut self, error_messages: Vec<ErrorMessage>) -> Self {
        self.error_messages = error_messages;
        self.success = self.error_messages.is_empty();
        self
    }

    /// Appends one error and marks failure (Hutool `addErrorMessage`).
    #[must_use]
    pub fn add_error_message(mut self, error_message: ErrorMessage) -> Self {
        self.error_messages.push(error_message);
        self.success = false;
        self
    }

    /// Builds a result from explicit property checks (Rust-native warp helper).
    ///
    /// Callers supply `(property, message, value)` triples when a field fails;
    /// empty input yields a successful result.
    #[must_use]
    pub fn from_failures(
        failures: impl IntoIterator<Item = (String, String, Option<String>)>,
    ) -> Self {
        let mut result = Self::new(true);
        for (property_name, message, value) in failures {
            result = result.add_error_message(ErrorMessage::new(property_name, message, value));
        }
        result
    }
}

/// Hutool `ValidationUtil` warp helpers without a Jakarta Validator factory.
///
/// 对齐 Java 类: `cn.hutool.extra.validation.ValidationUtil`（结果包装子集）
///
/// `getValidator` / reflective `validate` remain planned.
pub struct ValidationUtil;

impl ValidationUtil {
    /// Wraps precomputed failures into [`BeanValidationResult`] (Hutool `warpValidate` shape).
    #[must_use]
    pub fn warp_validate(
        failures: impl IntoIterator<Item = (String, String, Option<String>)>,
    ) -> BeanValidationResult {
        BeanValidationResult::from_failures(failures)
    }

    /// Wraps a single-property failure list (Hutool `warpValidateProperty` shape).
    #[must_use]
    pub fn warp_validate_property(
        property_name: impl Into<String>,
        failures: impl IntoIterator<Item = (String, Option<String>)>,
    ) -> BeanValidationResult {
        let property = property_name.into();
        BeanValidationResult::from_failures(failures.into_iter().map(|(message, value)| {
            (property.clone(), message, value)
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bean_validation_result_tracks_errors() {
        let result = BeanValidationResult::new(true)
            .add_error_message(ErrorMessage::new("name", "姓名不能为空", None))
            .add_error_message(ErrorMessage::new("address", "地址不能为空", None));
        assert!(!result.is_success());
        assert_eq!(result.error_messages().len(), 2);
    }

    #[test]
    fn validation_util_warp_helpers() {
        let result = ValidationUtil::warp_validate([
            ("name".into(), "姓名不能为空".into(), None),
            ("address".into(), "地址不能为空".into(), None),
        ]);
        assert!(!result.is_success());
        let prop = ValidationUtil::warp_validate_property(
            "name",
            [("姓名不能为空".into(), None)],
        );
        assert!(!prop.is_success());
        assert_eq!(prop.error_messages().len(), 1);
    }
}
