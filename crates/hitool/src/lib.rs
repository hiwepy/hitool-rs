//! Feature-gated facade for the `HiTool` workspace.
//!
//! Default features include only `core` and `json`. Applications can disable
//! defaults and opt into individual Hutool-aligned capability crates.

#![forbid(unsafe_code)]

#[cfg(feature = "ai")]
pub use hitool_ai as ai;
#[cfg(feature = "aop")]
pub use hitool_aop as aop;
#[cfg(feature = "bloom-filter")]
pub use hitool_bloom_filter as bloom_filter;
#[cfg(feature = "cache")]
pub use hitool_cache as cache;
#[cfg(feature = "captcha")]
pub use hitool_captcha as captcha;
#[cfg(feature = "hutool-compat")]
pub use hitool_compat_hutool as compat;
#[cfg(feature = "core")]
pub use hitool_core as core;
#[cfg(feature = "cron")]
pub use hitool_cron as cron;
#[cfg(feature = "crypto")]
pub use hitool_crypto as crypto;
#[cfg(feature = "db")]
pub use hitool_db as db;
#[cfg(feature = "dfa")]
pub use hitool_dfa as dfa;
#[cfg(feature = "extra")]
pub use hitool_extra as extra;
#[cfg(feature = "http")]
pub use hitool_http as http;
#[cfg(feature = "json")]
pub use hitool_json as json;
#[cfg(feature = "jwt")]
pub use hitool_jwt as jwt;
#[cfg(feature = "log")]
pub use hitool_log as log;
#[cfg(feature = "poi")]
pub use hitool_poi as poi;
#[cfg(feature = "script")]
pub use hitool_script as script;
#[cfg(feature = "setting")]
pub use hitool_setting as setting;
#[cfg(feature = "socket")]
pub use hitool_socket as socket;
#[cfg(feature = "system")]
pub use hitool_system as system;

/// Common extension traits and data types enabled by facade features.
pub mod prelude {
    #[cfg(feature = "core")]
    pub use hitool_core::prelude::*;
    #[cfg(feature = "json")]
    pub use hitool_json::prelude::*;
}

#[cfg(test)]
mod tests {
    #[cfg(all(feature = "core", feature = "json"))]
    #[test]
    fn default_facade_exposes_core_and_json() {
        use crate::prelude::*;

        assert!(" \t".is_blank());
        assert_eq!(crate::json::parse(r#"{"ok":true}"#).unwrap()["ok"], true);
    }
}
