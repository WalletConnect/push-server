use ed25519_dalek::VerifyingKey;

#[derive(Clone)]
pub struct RelayClient {
    public_key: VerifyingKey,
}

impl RelayClient {
    pub fn new(string_public_key: String) -> crate::error::Result<RelayClient> {
        let verifying_key = Self::string_to_verifying_key(&string_public_key)?;
        Ok(RelayClient {
            public_key: verifying_key,
        })
    }

    pub fn get_verifying_key(&self) -> &VerifyingKey {
        &self.public_key
    }

    fn string_to_verifying_key(string_key: &str) -> crate::error::Result<VerifyingKey> {
        let key_bytes = hex::decode(string_key).map_err(crate::error::Error::Hex)?;
        Ok(VerifyingKey::from_bytes(
            <&[u8; 32]>::try_from(key_bytes.as_slice()).unwrap(),
        )?)
    }
}
