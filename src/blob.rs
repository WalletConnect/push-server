use crate::error::Result;
use serde::{Serialize, Deserialize};

pub type Flag = u32;
pub const ENCRYPTED_FLAG: Flag = 1 << 0;
// pub const SIGN_FLAG: Flag = 1 << 1;
// pub const AUTH_FLAG: Flag = 1 << 2;
// pub const CHAT_FLAG: Flag = 1 << 3;
// pub const PUSH_FLAG: Flag = 1 << 4;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct DecryptedPayloadBlob {
    pub title: String,
    pub body: String,
    pub image: Option<String>,
    pub url: Option<String>,
}

impl DecryptedPayloadBlob {
    pub fn from_json_string(blob_string: String) -> Result<DecryptedPayloadBlob> {
        Ok(serde_json::from_str(&blob_string)?)
    }

    pub fn from_base64_encoded(blob_string: String) -> Result<DecryptedPayloadBlob> {
        let blob_decoded = base64::decode(&blob_string)?;
        Ok(serde_json::from_slice(&blob_decoded)?)
    }
}
