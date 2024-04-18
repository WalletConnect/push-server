use {
    crate::error::{Error, Result},
    base64::Engine as _,
    serde::{Deserialize, Serialize},
};

pub type Flag = u32;
pub const ENCRYPTED_FLAG: Flag = 1 << 0;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct DecryptedPayloadBlob {
    pub title: String,
    pub body: String,
    pub image: Option<String>,
    pub url: Option<String>,
}

impl DecryptedPayloadBlob {
    pub fn from_base64_encoded(blob_string: &str) -> Result<DecryptedPayloadBlob> {
        let blob_decoded = base64::engine::general_purpose::STANDARD
            .decode(blob_string)
            .map_err(Error::DecryptedNotificationDecode)?;
        serde_json::from_slice(&blob_decoded).map_err(Error::DecryptedNotificationParse)
    }
}
