#![no_main]

use hutool_core::{
    Base16Codec, Base32Decoder, Base32Encoder, Base58Decoder, Base58Encoder, Base62Decoder,
    Base62Encoder, base32_decode, base32_encode, base32_hex_decode, base32_hex_encode,
    base58_decode, base58_decode_checked, base58_encode, base58_encode_checked, base62_decode,
    base62_encode, base62_inverted_decode, base62_inverted_encode, base64_decode,
    base64_decode_tolerant, base64_encode, base64_encode_config, base64_url_decode,
    base64_url_encode, bcd_decode, bcd_encode, hex_decode, hex_encode,
};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    assert_eq!(base64_decode(base64_encode(data)).unwrap(), data);
    assert_eq!(base64_url_decode(base64_url_encode(data)).unwrap(), data);
    assert_eq!(
        base64_decode_tolerant(base64_encode_config(data, true, false)),
        data
    );
    assert_eq!(hex_decode(hex_encode(data)).unwrap(), data);
    assert_eq!(
        Base16Codec::LOWER
            .decode_text(&Base16Codec::LOWER.encode_bytes(data))
            .unwrap(),
        data
    );
    assert_eq!(base32_decode(&base32_encode(data)).unwrap(), data);
    assert_eq!(base32_hex_decode(&base32_hex_encode(data)).unwrap(), data);
    assert_eq!(
        Base32Decoder::standard()
            .decode_text(&Base32Encoder::standard().encode_bytes(data)),
        data
    );
    assert_eq!(base58_decode(&base58_encode(data)).unwrap(), data);
    assert_eq!(
        Base58Decoder::bitcoin()
            .decode_text(&Base58Encoder::bitcoin().encode_bytes(data))
            .unwrap(),
        data
    );
    assert_eq!(base62_decode(&base62_encode(data)).unwrap(), data);
    assert_eq!(
        Base62Decoder::gmp()
            .decode_bytes(&Base62Encoder::gmp().encode_bytes(data))
            .unwrap(),
        data
    );
    assert_eq!(
        base62_inverted_decode(&base62_inverted_encode(data)).unwrap(),
        data
    );
    let checked = base58_encode_checked(Some(0), data);
    assert_eq!(base58_decode_checked(&checked, true).unwrap(), data);
    let hex = hex_encode(data);
    assert_eq!(bcd_decode(&bcd_encode(&hex).unwrap()), hex.to_uppercase());
});
