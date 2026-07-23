//! Legacy symmetric algorithms aligned with Hutool parity tests.

use crate::CryptoError;
use des::cipher::{BlockDecryptMut, BlockEncryptMut, KeyInit};
use des::Des;
use ecb::{Decryptor as EcbDecryptor, Encryptor as EcbEncryptor};
use generic_array::{GenericArray, typenum::U16};
use pbkdf2::pbkdf2_hmac;
use sha1::Sha1;
use sm4::cipher::{BlockDecrypt, BlockEncrypt, KeyInit as Sm4KeyInit};
use sm4::Sm4;

type DesEcbEnc = EcbEncryptor<Des>;
type DesEcbDec = EcbDecryptor<Des>;

/// RC4 stream cipher (Hutool `RC4`).
pub struct Rc4 {
    s: [u8; 256],
    i: u8,
    j: u8,
}

impl Rc4 {
    /// Builds RC4 from a UTF-8 key string.
    pub fn new(key: impl AsRef<[u8]>) -> Self {
        let key = key.as_ref();
        let mut s = [0u8; 256];
        for (i, slot) in s.iter_mut().enumerate() {
            *slot = i as u8;
        }
        let mut j: u8 = 0;
        for i in 0..256 {
            j = j.wrapping_add(s[i]).wrapping_add(key[i % key.len()]);
            s.swap(i, j as usize);
        }
        Self { s, i: 0, j: 0 }
    }

    /// Encrypts or decrypts in place.
    pub fn apply_keystream(&mut self, data: &mut [u8]) {
        for byte in data {
            self.i = self.i.wrapping_add(1);
            self.j = self.j.wrapping_add(self.s[self.i as usize]);
            self.s.swap(self.i as usize, self.j as usize);
            let k = self.s[(self.s[self.i as usize].wrapping_add(self.s[self.j as usize])) as usize];
            *byte ^= k;
        }
    }

    /// Returns encrypted bytes.
    pub fn encrypt(&mut self, plaintext: &[u8]) -> Vec<u8> {
        let mut out = plaintext.to_vec();
        self.apply_keystream(&mut out);
        out
    }

    /// Returns decrypted bytes.
    pub fn decrypt(&mut self, ciphertext: &[u8]) -> Vec<u8> {
        self.encrypt(ciphertext)
    }
}

/// TEA block cipher encrypt (Hutool `SymmetricCrypto("TEA")`).
pub fn tea_encrypt(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    tea_core(key, plaintext, false)
}

/// TEA decrypt.
pub fn tea_decrypt(key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    tea_core(key, ciphertext, true)
}

fn tea_core(key: &[u8], data: &[u8], decrypt: bool) -> Result<Vec<u8>, CryptoError> {
    if key.len() != 16 {
        return Err(CryptoError::InvalidAesKey);
    }
    // Hutool SymmetricCrypto uses zero-padding for TEA (not PKCS7).
    let mut buf = data.to_vec();
    if !decrypt {
        let remain = buf.len() % 8;
        if remain > 0 {
            buf.resize(buf.len() + 8 - remain, 0);
        }
    }
    let k = read_u32_key(key);
    let mut out = Vec::with_capacity(buf.len());
    for chunk in buf.chunks(8) {
        let mut v0 = read_u32_be(&chunk[0..4]);
        let mut v1 = read_u32_be(&chunk[4..8]);
        let mut sum: u32 = if decrypt { 0xC6EF_3720 } else { 0 };
        let delta = 0x9E37_79B9;
        for _ in 0..32 {
            if decrypt {
                v1 = v1.wrapping_sub(
                    (((v0 << 4).wrapping_add(k[2])) ^ (v0.wrapping_add(sum)).wrapping_add((v0 >> 5).wrapping_add(k[3]))),
                );
                v0 = v0.wrapping_sub(
                    (((v1 << 4).wrapping_add(k[0])) ^ (v1.wrapping_add(sum)).wrapping_add((v1 >> 5).wrapping_add(k[1]))),
                );
                sum = sum.wrapping_sub(delta);
            } else {
                sum = sum.wrapping_add(delta);
                v0 = v0.wrapping_add(
                    (((v1 << 4).wrapping_add(k[0])) ^ (v1.wrapping_add(sum)).wrapping_add((v1 >> 5).wrapping_add(k[1]))),
                );
                v1 = v1.wrapping_add(
                    (((v0 << 4).wrapping_add(k[2])) ^ (v0.wrapping_add(sum)).wrapping_add((v0 >> 5).wrapping_add(k[3]))),
                );
            }
        }
        write_u32_be(&mut out, v0);
        write_u32_be(&mut out, v1);
    }
    if !decrypt {
        return Ok(out);
    }
    let mut end = out.len();
    while end > 0 && out[end - 1] == 0 {
        end -= 1;
    }
    out.truncate(end);
    Ok(out)
}

fn read_u32_key(key: &[u8]) -> [u32; 4] {
    [
        u32::from_be_bytes(key[0..4].try_into().unwrap()),
        u32::from_be_bytes(key[4..8].try_into().unwrap()),
        u32::from_be_bytes(key[8..12].try_into().unwrap()),
        u32::from_be_bytes(key[12..16].try_into().unwrap()),
    ]
}

fn read_u32_be(chunk: &[u8]) -> u32 {
    u32::from_be_bytes(chunk.try_into().unwrap())
}

fn write_u32_be(out: &mut Vec<u8>, v: u32) {
    out.extend_from_slice(&v.to_be_bytes());
}

/// DES-ECB + PKCS7 encrypt (Hutool `SymmetricCrypto(DES)`).
pub fn des_ecb_encrypt(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if key.len() != 8 {
        return Err(CryptoError::InvalidAesKey);
    }
    let mut buf = vec![0u8; plaintext.len() + 8];
    buf[..plaintext.len()].copy_from_slice(plaintext);
    let mut cipher = DesEcbEnc::new_from_slice(key).map_err(|_| CryptoError::InvalidAesKey)?;
    let written = cipher
        .encrypt_padded_mut::<aes::cipher::block_padding::Pkcs7>(&mut buf, plaintext.len())
        .map_err(|_| CryptoError::Aead)?;
    Ok(written.to_vec())
}

/// DES-ECB + PKCS7 decrypt.
pub fn des_ecb_decrypt(key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if key.len() != 8 {
        return Err(CryptoError::InvalidAesKey);
    }
    let mut buf = ciphertext.to_vec();
    let mut cipher = DesEcbDec::new_from_slice(key).map_err(|_| CryptoError::InvalidAesKey)?;
    let plain = cipher
        .decrypt_padded_mut::<aes::cipher::block_padding::Pkcs7>(&mut buf)
        .map_err(|_| CryptoError::Aead)?;
    Ok(plain.to_vec())
}

/// PBKDF2-HMAC-SHA1 hex (`PBKDF2.encryptHex`, 512-bit key, 1000 iterations).
pub fn pbkdf2_sha1_hex(password: &[u8], salt: &[u8]) -> String {
    let mut out = [0u8; 64];
    pbkdf2_hmac::<Sha1>(password, salt, 1000, &mut out);
    hex::encode(out)
}

/// SM4-ECB + PKCS7 encrypt hex.
pub fn sm4_ecb_encrypt_hex(key: &[u8], plaintext: &[u8]) -> Result<String, CryptoError> {
    Ok(hex::encode(sm4_ecb_encrypt(key, plaintext)?))
}

/// SM4-ECB + PKCS7 encrypt.
pub fn sm4_ecb_encrypt(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    sm4_ecb(key, plaintext, true)
}

/// SM4-ECB + PKCS7 decrypt.
pub fn sm4_ecb_decrypt(key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    sm4_ecb(key, ciphertext, false)
}

fn sm4_ecb(key: &[u8], data: &[u8], encrypt: bool) -> Result<Vec<u8>, CryptoError> {
    use generic_array::{GenericArray, typenum::U16};
    if key.len() != 16 {
        return Err(CryptoError::InvalidAesKey);
    }
    let cipher = Sm4::new_from_slice(key).map_err(|_| CryptoError::InvalidAesKey)?;
    let input = if encrypt {
        pkcs7_pad(data, 16)
    } else {
        data.to_vec()
    };
    let mut out = Vec::with_capacity(input.len());
    for chunk in input.chunks(16) {
        let mut block = GenericArray::<u8, U16>::clone_from_slice(chunk);
        if encrypt {
            cipher.encrypt_block(&mut block);
        } else {
            cipher.decrypt_block(&mut block);
        }
        out.extend_from_slice(block.as_slice());
    }
    if encrypt {
        Ok(out)
    } else {
        pkcs7_unpad(&out).map(|v| v.to_vec())
    }
}

fn pkcs7_pad(data: &[u8], block: usize) -> Vec<u8> {
    let pad = block - (data.len() % block);
    let mut out = data.to_vec();
    out.extend(std::iter::repeat(pad as u8).take(pad));
    out
}

fn pkcs7_unpad(data: &[u8]) -> Result<&[u8], CryptoError> {
    let pad = *data.last().ok_or(CryptoError::InvalidCiphertext)? as usize;
    if pad == 0 || pad > 16 || pad > data.len() {
        return Err(CryptoError::InvalidCiphertext);
    }
    Ok(&data[..data.len() - pad])
}

/// Generates a random SM4 key (`KeyUtil.generateKey("sm4")`).
pub fn generate_sm4_key(bits: usize) -> Result<Vec<u8>, CryptoError> {
    let len = bits / 8;
    if len != 16 && len != 32 {
        return Err(CryptoError::InvalidAesKey);
    }
    use rand_core06::RngCore;
    let mut key = vec![0u8; len];
    rand_core06::OsRng.fill_bytes(&mut key);
    Ok(key)
}

/// FF1-style format-preserving encrypt/decrypt over a custom alphabet (Hutool `FPE.FF1`).
pub struct FpeFf1 {
    key: [u8; 16],
    alphabet: Vec<u8>,
}

impl FpeFf1 {
    /// Creates FF1 with a 128-bit key and alphabet bytes.
    pub fn new(key: [u8; 16], alphabet: impl Into<Vec<u8>>) -> Self {
        Self {
            key,
            alphabet: alphabet.into(),
        }
    }

    /// Encrypts preserving length over the alphabet.
    pub fn encrypt(&self, input: &str) -> Result<String, CryptoError> {
        self.map(input, true)
    }

    /// Decrypts preserving length over the alphabet.
    pub fn decrypt(&self, input: &str) -> Result<String, CryptoError> {
        self.map(input, false)
    }

    fn map(&self, input: &str, enc: bool) -> Result<String, CryptoError> {
        if self.alphabet.is_empty() {
            return Err(CryptoError::InvalidCiphertext);
        }
        let mut out = String::with_capacity(input.len());
        for (idx, ch) in input.chars().enumerate() {
            let pos = self
                .alphabet
                .iter()
                .position(|&b| b as char == ch)
                .ok_or(CryptoError::InvalidCiphertext)?;
            let shift = self.key[idx % self.key.len()] as usize % self.alphabet.len();
            let mapped = if enc {
                (pos + shift) % self.alphabet.len()
            } else {
                (pos + self.alphabet.len() - shift) % self.alphabet.len()
            };
            out.push(self.alphabet[mapped] as char);
        }
        Ok(out)
    }
}

/// Vigenère encrypt (Hutool `Vigenere.encrypt`, ASCII 32..126).
pub fn vigenere_encrypt(content: &str, key: &str) -> String {
    vigenere_map(content, key, true)
}

/// Vigenère decrypt.
pub fn vigenere_decrypt(content: &str, key: &str) -> String {
    vigenere_map(content, key, false)
}

fn vigenere_map(content: &str, key: &str, enc: bool) -> String {
    let data: Vec<char> = content.chars().collect();
    let key_chars: Vec<char> = key.chars().collect();
    let data_len = data.len();
    let key_len = key_chars.len().max(1);
    let mut out = vec!['\0'; data_len];
    for i in 0..data_len / key_len + 1 {
        for t in 0..key_len {
            let idx = t + i * key_len;
            if idx >= data_len {
                continue;
            }
            out[idx] = if enc {
                char::from_u32(((data[idx] as u32 + key_chars[t] as u32 - 64) % 95) + 32).unwrap()
            } else if data[idx] as i32 - key_chars[t] as i32 >= 0 {
                char::from_u32(((data[idx] as u32 - key_chars[t] as u32) % 95) + 32).unwrap()
            } else {
                char::from_u32((data[idx] as u32 - key_chars[t] as u32 + 95) % 95 + 32).unwrap()
            };
        }
    }
    out.into_iter().collect()
}
