use {
    crate::error::Result,
    jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation},
};

pub struct GoTrueClaims {
    pub sub: String,
    pub aud: String,
    pub role: String,
}

pub struct GoTrueClient {
    decoding_key: DecodingKey,
    validation: Validation,
}

impl GoTrueClient {
    pub fn new(jwt_secret: String) -> GoTrueClient {
        GoTrueClient {
            decoding_key: DecodingKey::from_secret(jwt_secret.as_bytes()),
            validation: Validation::new(Algorithm::HS256),
        }
    }

    pub fn is_valid_token(&self, jwt: String) -> Result<TokenData<GoTrueClaims>> {
        Ok(jsonwebtoken::decode::<GoTrueClaims>(
            &jwt,
            &self.decoding_key,
            &self.validation,
        )?)
    }
}
