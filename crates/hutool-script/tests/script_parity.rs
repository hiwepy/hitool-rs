//! Hutool `hutool-script` → hutool-script test parity.
//!
//! 对齐: `cn.hutool.script.test.ScriptUtilTest`
//!
//! Language note: Hutool defaults to JSR-223 JavaScript; hutool-script uses sandboxed
//! Rhai under the same `ScriptUtil` facade. Script text below is the Rhai equivalent of
//! the JavaScript fixtures (string quotes, `fn` syntax). Optional Python/Lua/Groovy
//! engines are intentionally not linked — those tests assert the unsupported-engine error.

use hutool_script::{ScriptContext, ScriptEngine, ScriptLimits, ScriptUtil};

/// 对齐 Java: `ScriptUtilTest.compileTest()`
///
/// Java: `ScriptUtil.compile("print('Script test!');")` then `script.eval()`.
/// Rhai uses double-quoted strings (`'` is a char literal).
#[test]
fn compile_test() {
    let script = ScriptUtil::compile(r#"print("Script test!");"#)
        .unwrap_or_else(|e| panic!("compile failed: {e}"));
    let _ = script
        .eval(&ScriptEngine::default(), &mut ScriptContext::new())
        .unwrap_or_else(|e| panic!("compiled eval failed: {e}"));
}

/// 对齐 Java: `ScriptUtilTest.evalTest()`
///
/// Java: `ScriptUtil.eval("print('Script test!');")`.
#[test]
fn eval_test() {
    let _ = ScriptUtil::eval(r#"print("Script test!");"#)
        .unwrap_or_else(|e| panic!("eval failed: {e}"));
}

/// 对齐 Java: `ScriptUtilTest.invokeTest()`
///
/// Java loads `filter1.js` via `ResourceUtil.readUtf8Str("filter1.js")` and invokes
/// `filter1(2, 1)` expecting `true`. Fixture logic is identical; syntax is Rhai.
#[test]
fn invoke_test() {
    // Same control flow as hutool-script/src/test/resources/filter1.js
    let filter1 = r#"
fn filter1(a, b) {
    if a > b {
        return a > b;
    }
    false
}
"#;
    let result = ScriptUtil::invoke(filter1, "filter1", vec![2_i64.into(), 1_i64.into()])
        .unwrap_or_else(|e| panic!("invoke failed: {e}"));
    assert!(
        result.as_bool().expect("filter1 should return bool"),
        "filter1(2, 1) should be true (对齐 Java assertTrue)"
    );
}

/// 对齐 Java: `ScriptUtilTest.pythonTest()`
///
/// Java: `ScriptUtil.getPythonEngine().eval("print('Hello Python')")` (needs Jython).
/// hutool-script does not link an optional Python engine — identical successful eval is
/// unblockable; assert the documented unsupported-engine error instead.
#[test]
fn python_test() {
    match ScriptUtil::get_python_engine() {
        Ok(_) => panic!("python engine must be unsupported"),
        Err(err) => assert!(
            err.to_string().contains("optional engine") || err.to_string().contains("Python"),
            "unexpected error: {err}"
        ),
    }
}

/// 对齐 Java: `ScriptUtilTest.luaTest()`
///
/// Java: `ScriptUtil.getLuaEngine().eval("print('Hello Lua')")` (needs luaj).
/// Optional Lua engine is not linked — identical successful eval is unblockable.
#[test]
fn lua_test() {
    match ScriptUtil::get_lua_engine() {
        Ok(_) => panic!("lua engine must be unsupported"),
        Err(err) => assert!(
            err.to_string().contains("optional engine") || err.to_string().contains("Lua"),
            "unexpected error: {err}"
        ),
    }
}

/// 对齐 Java: `ScriptUtilTest.groovyTest()`
///
/// Java: `ScriptUtil.getGroovyEngine().eval("println 'Hello Groovy'")` (needs Groovy).
/// Optional Groovy engine is not linked — identical successful eval is unblockable.
#[test]
fn groovy_test() {
    match ScriptUtil::get_groovy_engine() {
        Ok(_) => panic!("groovy engine must be unsupported"),
        Err(err) => assert!(
            err.to_string().contains("optional engine") || err.to_string().contains("Groovy"),
            "unexpected error: {err}"
        ),
    }
}

/// Existing smoke: ScriptEngine construction (kept; not a Hutool inventory method).
#[test]
fn script_engine_test() {
    let limits = ScriptLimits::default();
    let _engine = ScriptEngine::new(limits);
    assert!(limits.max_operations > 0);
}

/// Existing smoke: ScriptLimits defaults (kept; not a Hutool inventory method).
#[test]
fn script_limits_test() {
    let limits = ScriptLimits::default();
    assert!(limits.max_operations > 0, "max_operations 应 > 0");
    assert!(limits.max_call_levels > 0, "max_call_levels 应 > 0");
}
