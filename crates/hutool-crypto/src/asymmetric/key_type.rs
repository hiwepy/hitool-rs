/// Hutool `KeyType` — selects public or private key material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyType {
    /// Public key.
    PublicKey,
    /// Private key.
    PrivateKey,
}
