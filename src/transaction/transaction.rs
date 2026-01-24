use crate::crypto::hash::{Hash};
use crate::crypto::keypair::{self, Address};
use crate::crypto::keypair::PublicKey;
use serde::{Serialize, Deserialize};
use std::ops::Add;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Transaction ID (hash of transaction)
    pub id: Hash,

    pub from: Address,

    pub to: Address,

    pub amount: u64, // Using u64 for precision (satoshi-style)

    pub fee: u64,

    pub timestamp: u64,

    pub nonce: u64, // None (prevent replay attacks)

    pub public_key: PublicKey,

    pub signature: Vec<u8>
}


impl Transaction {
    /// Create new unsigned transaction
    pub fn new(
        from: Address,
        to: Address,
        amount: u64,
        fee: u64,
        nonce: u64,
        public_key: PublicKey
    ) -> Self {

        let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

        let mut tx = Transaction {
            id: Hash::zero(), // Will calculate after sign
            from,
            to,
            amount,
            fee,
            timestamp,
            nonce,
            public_key,
            signature: Vec::new()
        };

        // Calculate ID from transaction data (without include signature)
        tx.id = tx.calculate_hash();
        tx
    }


    /// Calculate hash of transaction (use for ID)
    pub fn calculate_hash(&self) -> Hash {
        let data = bincode::serialize(&(
            &self.from,
            &self.to,
            self.amount,
            self.fee,
            self.timestamp,
            self.nonce,
        )).unwrap();
        
        Hash::hash(&data)
    }


    /// Sign transaction
    pub fn sign(&mut self, keypair: &crate::crypto::keypair::) {
        let data = self.calculate_hash();
        self.signature = keypair.sign(data.as_bytes());
    }

    pub fn verify(&self) -> bool {
        // 1. Verify address matches public key
        if self.from != self.public_key.to_address() {
            return false;
        }

        // 2. Verify signature
        let data = self.calculate_hash();
        crate::crypto::keypair::KeyPair::verify(
            &self.public_key, 
            data.as_bytes(),
            &self.signature
        )
    }

    pub fn coinbase(to: Address, amount: u64, block_height: u64) -> Self {
        let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

        let mut tx = Transaction {
            id: Hash::zero(),
            from: Address::from_hex("0x0000000000000000000000000000000000000000").unwrap(),
            to,
            amount,
            fee:0,
            timestamp,
            nonce: block_height,
            public_key: PublicKey::from_bytes(vec![0;33]),
            signature: Vec::new(),
        };

        tx.id = tx.calculate_hash();
        tx
    }

    pub fn is_coinbase(&self) -> bool {
        self.from == Address::from_hex("0x0000000000000000000000000000000000000000").unwrap()
    }
}