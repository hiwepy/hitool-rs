//! `cn.hutool.core.exceptions` 子包对齐
//!
//! 自动生成的模块入口,1:1 镜像 Java 包结构。
//! 每个子模块对应一个 Java 类(`.java` → `.rs`),命名遵循 snake_case。
//! 详细对齐信息见各 `.rs` 文件头注释。

pub mod checked_util;
pub mod dependency_exception;
pub mod exception_util;
pub mod invocation_target_runtime_exception;
pub mod not_inited_exception;
pub mod stateful_exception;
pub mod util_exception;
pub mod validate_exception;

pub use checked_util::{
    CheckedUtil, UncheckedFn0, UncheckedFn1, UncheckedVoidFn0, WrappedRuntime, sleep_checked,
};
pub use dependency_exception::DependencyException;
pub use exception_util::{ExceptionUtil, StackFrame, WrappedError};
pub use invocation_target_runtime_exception::InvocationTargetRuntimeException;
pub use not_inited_exception::NotInitedException;
pub use stateful_exception::StatefulException;
pub use util_exception::UtilException;
pub use validate_exception::ValidateException;

#[cfg(test)]
mod exceptions_idiomatic_parity {
    use super::*;

    /// 对齐 Java exceptions 层次与工具门面的可执行证据。
    #[test]
    fn exception_hierarchy_and_util_facades() {
        let s = StatefulException::with_status(404, "missing");
        assert_eq!(s.get_status(), 404);
        let v = ValidateException::with_status(400, "bad");
        assert_eq!(v.get_status(), 400);
        let _ = UtilException::with_template("x={}", &[&1]);
        let _ = NotInitedException::new("not ready");
        let _ = DependencyException::new("dep");
        let _ = InvocationTargetRuntimeException::new("invoke");
        let f = CheckedUtil::uncheck0(|| 1_i32);
        assert_eq!(f.call(), 1);
        assert!(!ExceptionUtil::get_stack_elements().is_empty());
    }
}
