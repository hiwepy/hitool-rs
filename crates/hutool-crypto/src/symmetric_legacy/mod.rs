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

mod rc4;
mod fpe_ff1;

pub use rc4::Rc4;
pub use fpe_ff1::FpeFf1;
pub use rc4::tea_encrypt;
pub use rc4::tea_decrypt;
pub use rc4::des_ecb_encrypt;
pub use rc4::des_ecb_decrypt;
pub use rc4::pbkdf2_sha1_hex;
pub use rc4::sm4_ecb_encrypt_hex;
pub use rc4::sm4_ecb_encrypt;
pub use rc4::sm4_ecb_decrypt;
pub use rc4::generate_sm4_key;
pub use rc4::vigenere_encrypt;
pub use rc4::vigenere_decrypt;
