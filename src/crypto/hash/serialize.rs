use serde::{Serialize, Serializer};

use crate::crypto::hash::Hash;

// Storage
impl Serialize for Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        serializer.serialize_str(&self.to_hex())
    }
}