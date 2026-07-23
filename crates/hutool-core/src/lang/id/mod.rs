//! `cn.hutool.core.lang.id` 子包对齐
pub mod id_constants;
pub mod nano_id;
pub use id_constants::IdConstants;
pub use nano_id::{JavaRandom, NanoId, DEFAULT_SIZE};
