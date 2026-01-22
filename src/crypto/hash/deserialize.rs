use serde::{Deserialize, Deserializer};

use crate::crypto::hash::Hash;

// Storage
impl<'de> Deserialize<'de> for Hash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        Hash::from_hex(&s).map_err(serde::de::Error::custom)
    }
}