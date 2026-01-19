use sha2::{Sha256, Digest};
use std::{fmt, mem::take};



/// Wrapper for hash with type safety
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hash([u8; 32]);

impl Hash {
    /// Create Hash from data
    pub fn hash(data: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        Hash(hash)
    }

    /// Zero hash (genesis)
    pub fn zero() -> Self {
        Hash([0u8; 32])
    }

    /// Convert to bytes
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        hex::encode(&self.0)
    }

    /// From hex string
    pub fn from_hex(s: &str) -> Result<Self, hex::FromHexError> {
        let bytes = hex::decode(s)?;
        if bytes.len() != 32 {
            return Err(hex::FromHexError::InvalidStringLength);
        }
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&bytes);
        Ok(Hash(hash))
    }

    /// Check if hash meets difficulty (leading zeros)
    pub fn meets_difficulty(&self, difficulty: u32) -> bool {
        let leading_zeros = self.0.iter()
        .take_while(|&&b| b == 0)
        .count() * 8; // 8 bits per byte

        let first_nonzero = self.0.iter()
        .skip_while(|&&b| b == 0)
        .next()
        .unwrap_or(&0);

        let extra_zeros = first_nonzero.leading_zeros();

        (leading_zeros as u32 + extra_zeros) >= difficulty
    }
}