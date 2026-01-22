use secp256k1::{Message, PublicKey as Secp256k1PublicKey, Secp256k1, SecretKey, ecdsa::Signature, rand};

use crate::crypto::keypair::{Address, PublicKey};


pub struct KeyPair {
    secret_key: SecretKey,
    public_key: PublicKey
}

impl KeyPair {
    /// Generate new random keypair
    pub fn generate() -> Self {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(
            &mut rand::thread_rng()
        );

        KeyPair { 
            secret_key, 
            public_key: PublicKey(public_key.serialize().to_vec()) 
        }
    }

    /// Get public key
    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    /// Get address
    pub fn address(&self) -> Address {
        self.public_key.to_address()
    }

    /// Sign data
    pub fn sign(&self, data: &[u8]) -> Vec<u8> {
        let secp = Secp256k1::new();
        let message = Message::from_digest_slice(data)
            .expect("32 bytes");
        let signature = secp.sign_ecdsa(&message, &self.secret_key);
        signature.serialize_compact().to_vec()
    }

    /// Verify signature
    pub fn verify(
        public_key: &PublicKey,
        data: &[u8],
        signature: &[u8]
    ) -> bool {
        let secp = Secp256k1::new();

        let pubkey = match Secp256k1PublicKey::from_slice(
            public_key.as_bytes()
        ) {
            Ok(pk) => pk,
            Err(_) => return false
        };

        let message = match Message::from_digest_slice(data) {
            Ok(msg) => msg,
            Err(_) => return false
        };

        let sig = match Signature::from_compact(signature) {
            Ok(s) => s,
            Err(_) => return false
        };

        secp.verify_ecdsa(&message, &sig, &pubkey).is_ok()
    }
}