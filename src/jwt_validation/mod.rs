use {
    crate::error::Result,
    jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation},
    serde::{Deserialize, Serialize},
};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
}

#[derive(Clone)]
pub struct JwtValidationClient {
    decoding_key: DecodingKey,
    validation: Validation,
}

impl JwtValidationClient {
    pub fn new(jwt_secret: String) -> JwtValidationClient {
        JwtValidationClient {
            decoding_key: DecodingKey::from_secret(jwt_secret.as_bytes()),
            validation: Validation::new(Algorithm::HS256),
        }
    }

    pub fn is_valid_token(&self, jwt: String) -> Result<TokenData<Claims>> {
        Ok(jsonwebtoken::decode::<Claims>(
            &jwt,
            &self.decoding_key,
            &self.validation,
        )?)
    }
}
