pub use relay_rpc::domain::*;
use {
    relay_rpc::auth::{JwtClaims, JwtHeader, DID_DELIMITER, DID_METHOD, DID_PREFIX, JWT_DELIMITER},
    std::collections::HashSet,
};

#[derive(Debug, ThisError)]
pub enum JwtVerificationError {
    #[error("Invalid format")]
    Format,

    #[error("Invalid encoding")]
    Encoding,

    #[error("Invalid JWT signing algorithm")]
    Header,

    #[error("Invalid claims")]
    Claims,

    #[error("Invalid signature")]
    Signature,

    #[error("Invalid JSON")]
    Serialization,

    #[error("Invalid issuer DID prefix")]
    IssuerPrefix,

    #[error("Invalid issuer DID method")]
    IssuerMethod,

    #[error("Invalid issuer format")]
    IssuerFormat,

    #[error(transparent)]
    PubKey(#[from] ClientIdDecodingError),
}

#[derive(Debug)]
pub struct Jwt(pub String);

impl Jwt {
    pub(crate) fn decode(&self, aud: &HashSet<String>) -> Result<ClientId, JwtVerificationError> {
        let mut parts = self.0.splitn(3, JWT_DELIMITER);

        let (Some(header), Some(claims)) = (parts.next(), parts.next()) else {
            return Err(JwtVerificationError::Format);
        };

        let decoder = &data_encoding::BASE64URL_NOPAD;

        let header_len = decoder
            .decode_len(header.len())
            .map_err(|_| JwtVerificationError::Encoding)?;
        let claims_len = decoder
            .decode_len(claims.len())
            .map_err(|_| JwtVerificationError::Encoding)?;

        let mut output = vec![0u8; header_len.max(claims_len)];

        // Decode header.
        data_encoding::BASE64URL_NOPAD
            .decode_mut(header.as_bytes(), &mut output[..header_len])
            .map_err(|_| JwtVerificationError::Encoding)?;

        {
            let header = serde_json::from_slice::<JwtHeader>(&output[..header_len])
                .map_err(|_| JwtVerificationError::Serialization)?;

            if !header.is_valid() {
                return Err(JwtVerificationError::Header);
            }
        }

        // Decode claims.
        data_encoding::BASE64URL_NOPAD
            .decode_mut(claims.as_bytes(), &mut output[..claims_len])
            .map_err(|_| JwtVerificationError::Encoding)?;

        let claims = serde_json::from_slice::<JwtClaims>(&output[..claims_len])
            .map_err(|_| JwtVerificationError::Serialization)?;

        // Basic token validation: `iat`, `exp` and `aud`.
        if !claims.is_valid(aud, None) {
            return Err(JwtVerificationError::Claims);
        }

        let did_key = claims
            .iss
            .strip_prefix(DID_PREFIX)
            .ok_or(JwtVerificationError::IssuerPrefix)?
            .strip_prefix(DID_DELIMITER)
            .ok_or(JwtVerificationError::IssuerFormat)?
            .strip_prefix(DID_METHOD)
            .ok_or(JwtVerificationError::IssuerMethod)?
            .strip_prefix(DID_DELIMITER)
            .ok_or(JwtVerificationError::IssuerFormat)?;

        let pub_key = did_key.parse::<DecodedClientId>()?;

        let mut parts = self.0.rsplitn(2, JWT_DELIMITER);

        let (Some(signature), Some(message)) = (parts.next(), parts.next()) else {
            return Err(JwtVerificationError::Format);
        };

        let key = jsonwebtoken::DecodingKey::from_ed_der(pub_key.as_ref());

        // Finally, verify signature.
        let sig_result = jsonwebtoken::crypto::verify(
            signature,
            message.as_bytes(),
            &key,
            jsonwebtoken::Algorithm::EdDSA,
        );

        match sig_result {
            Ok(true) => Ok(pub_key.into()),
            _ => Err(JwtVerificationError::Signature),
        }
    }
}
