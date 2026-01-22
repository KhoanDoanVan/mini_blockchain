use std::ops::Add;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

use crate::crypto::keypair::address::Address;


/// Public key wrapper
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicKey(Vec<u8>);


impl PublicKey {
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        PublicKey(bytes)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    // Derive address from public key (Bitcoin-style)
    pub fn to_address(&self) -> Address {
        let mut hasher = Sha256::new();
        hasher.update(&self.0);
        let hash = hasher.finalize();

        // Take first 20 bytes
        let mut addr = [0u8; 20];
        addr.copy_from_slice(&hash[0..20]);
        Address(addr)
    }
}