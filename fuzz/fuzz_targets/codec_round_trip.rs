#![no_main]

use hitool_core::{
    base64_decode, base64_encode, base64_url_decode, base64_url_encode, hex_decode, hex_encode,
};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    assert_eq!(base64_decode(base64_encode(data)).unwrap(), data);
    assert_eq!(base64_url_decode(base64_url_encode(data)).unwrap(), data);
    assert_eq!(hex_decode(hex_encode(data)).unwrap(), data);
});
