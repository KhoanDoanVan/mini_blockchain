use core::fmt;
use std::fmt::format;



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address([u8; 20]);

/// Hash of public key
impl Address {
    pub fn to_hex(&self) -> String {
        format!("0x{}", hex::encode(&self.0))
    }

    pub fn from_hex(s: &str) -> Result<Self, hex::FromHexError> {
        let s = s.strip_prefix("0x").unwrap_or(s);
        let bytes = hex::decode(s)?;
        if bytes.len() != 20 {
            return Err(hex::FromHexError::InvalidStringLength);
        }
        let mut addr = [0u8; 20];
        addr.copy_from_slice(&bytes);
        Ok(Address(addr))
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}