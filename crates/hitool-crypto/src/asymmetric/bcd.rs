//! Hutool `BCD` helpers (aligned with `cn.hutool.core.codec.BCD`).

/// Converts encrypted bytes to an ASCII BCD string (`BCD.bcdToStr`).
#[must_use]
pub fn bcd_to_str(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        let high = (byte >> 4) & 0x0f;
        let low = byte & 0x0f;
        out.push(nibble_to_char(high));
        out.push(nibble_to_char(low));
    }
    out
}

/// Converts an ASCII BCD string to bytes (`BCD.ascToBcd`).
pub fn asc_to_bcd(data: &str) -> Vec<u8> {
    let bytes = data.as_bytes();
    let mut out = Vec::with_capacity(bytes.len() / 2);
    let mut idx = 0;
    while idx + 1 < bytes.len() {
        let high = char_to_nibble(bytes[idx]);
        let low = char_to_nibble(bytes[idx + 1]);
        out.push((high << 4) | low);
        idx += 2;
    }
    out
}

fn nibble_to_char(n: u8) -> char {
    if n > 9 {
        char::from(b'A' + n - 10)
    } else {
        char::from(b'0' + n)
    }
}

fn char_to_nibble(ch: u8) -> u8 {
    match ch {
        b'0'..=b'9' => ch - b'0',
        b'a'..=b'f' => ch - b'a' + 10,
        b'A'..=b'F' => ch - b'A' + 10,
        _ => ch.saturating_sub(b'0'),
    }
}
