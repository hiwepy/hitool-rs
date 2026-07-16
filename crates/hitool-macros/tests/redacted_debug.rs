//! Integration tests for redacted debug output.

use hitool_macros::RedactedDebug;

#[derive(RedactedDebug)]
struct Credentials<T> {
    user: T,
    #[redact]
    password: String,
}

#[test]
fn redacts_annotated_fields_without_hiding_normal_fields() {
    let credentials = Credentials {
        user: "ada",
        password: "top-secret".to_owned(),
    };
    assert_eq!(
        format!("{credentials:?}"),
        "Credentials { user: \"ada\", password: \"[REDACTED]\" }"
    );
    assert!(!format!("{credentials:?}").contains("top-secret"));
    assert_eq!(credentials.password, "top-secret");
}
