//! SM2 helpers aligned with Hutool `SM2Test` / `BCUtilTest`.

use crate::CryptoError;
use sm2::dsa::signature::{Signer, Verifier};
use sm2::dsa::{Signature, SigningKey, VerifyingKey};
use sm2::{PublicKey, Scalar, SecretKey};

mod sm2_public_params;
mod sm2_private_params;

pub use sm2_public_params::Sm2PublicParams;
pub use sm2_private_params::Sm2PrivateParams;
pub use sm2_public_params::sm2_public_from_xy;
pub use sm2_public_params::sm2_private_from_hex;
pub use sm2_public_params::generate_sm2_keypair;
pub use sm2_public_params::sm2_private_scalar_len;
pub use sm2_public_params::sm2_oid_present_in_hex;
pub use sm2_public_params::sm2_sign;
pub use sm2_public_params::sm2_sign_hex;
pub use sm2_public_params::sm2_verify;
pub use sm2_public_params::sm2_public_from_secret;
pub use sm2_public_params::sm2_public_hex_from_secret;
pub use sm2_public_params::sm2_encrypt_decrypt_roundtrip;
pub use sm2_public_params::sm2_scalar_from_hex;
pub use sm2_public_params::sm2_private_hex_len;
