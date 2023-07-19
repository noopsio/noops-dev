use crate::errors::Error;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Jwt {
    iss: String,
    pub sub: String,
    iat: u64,
    exp: u64,
}

impl Jwt {
    pub fn new(iss: String, sub: String, iat: u64, exp_delta: u64) -> Self {
        Jwt {
            iss,
            sub,
            iat,
            exp: iat + exp_delta,
        }
    }

    pub fn encode(&self, encoding_key: &EncodingKey) -> anyhow::Result<String> {
        let jwt = jsonwebtoken::encode(&Header::default(), &self, encoding_key)?;
        Ok(jwt)
    }

    pub fn decode(
        jwt: &str,
        issuer: &str,
        decoding_key: &DecodingKey,
    ) -> Result<(Header, Self), Error> {
        let mut validation = Validation::default();
        validation.set_issuer(&[issuer]);
        let token_data = jsonwebtoken::decode::<Jwt>(jwt, decoding_key, &validation)?;
        Ok((token_data.header, token_data.claims))
    }
    pub fn create_issued_at() -> u64 {
        let start = SystemTime::now();
        start.duration_since(UNIX_EPOCH).unwrap().as_secs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    const JWT_SECRET: &str = "JWT_SECRET";
    const EXPIRED_DELTA: u64 = 600;
    const SUBJECT: &str = "TEST_SUBJECT";
    const ISSUER: &str = "TEST_ISSUER";

    lazy_static! {
        static ref ENCODING_KEY: EncodingKey = EncodingKey::from_secret(JWT_SECRET.as_bytes());
        static ref DECODING_KEY: DecodingKey = DecodingKey::from_secret(JWT_SECRET.as_bytes());
    }

    #[test]
    fn encode_jwt_decocde_success() -> anyhow::Result<()> {
        let issued_at = Jwt::create_issued_at();

        let jwt = Jwt::new(
            ISSUER.to_string(),
            SUBJECT.to_string(),
            issued_at,
            EXPIRED_DELTA,
        )
        .encode(&ENCODING_KEY)?;
        let _ = Jwt::decode(&jwt, ISSUER, &DECODING_KEY)?;
        Ok(())
    }

    #[test]
    fn decode_invalid_token() -> anyhow::Result<()> {
        let jwt = "INVALID_TOKEN";
        let decode_result = Jwt::decode(jwt, ISSUER, &DECODING_KEY);
        assert!(decode_result.is_err());

        if let Error::Token(err) = decode_result.unwrap_err() {
            assert_eq!(err.kind(), &jsonwebtoken::errors::ErrorKind::InvalidToken);
            return Ok(());
        }
        anyhow::bail!("No error ocurred while expecting InvalidToken");
    }

    #[test]
    fn decode_invalid_signature() -> anyhow::Result<()> {
        let issued_at = Jwt::create_issued_at();

        let jwt = Jwt::new(
            ISSUER.to_string(),
            SUBJECT.to_string(),
            issued_at,
            EXPIRED_DELTA,
        )
        .encode(&EncodingKey::from_secret("INVALID_SECRET".as_bytes()))?;

        let decode_result = Jwt::decode(&jwt, ISSUER, &DECODING_KEY);
        assert!(decode_result.is_err());

        if let Error::Token(err) = decode_result.unwrap_err() {
            assert_eq!(
                err.kind(),
                &jsonwebtoken::errors::ErrorKind::InvalidSignature
            );
            return Ok(());
        }
        anyhow::bail!("No error ocurred while expecting InvalidSignature");
    }

    #[test]
    fn decode_invalid_issuer() -> anyhow::Result<()> {
        let issued_at = Jwt::create_issued_at();

        let jwt = Jwt::new(
            "INVALID_ISSUER".to_string(),
            SUBJECT.to_string(),
            issued_at,
            EXPIRED_DELTA,
        )
        .encode(&ENCODING_KEY)?;

        let decode_result = Jwt::decode(&jwt, ISSUER, &DECODING_KEY);

        assert!(decode_result.is_err());

        if let Error::Token(err) = decode_result.unwrap_err() {
            assert_eq!(err.kind(), &jsonwebtoken::errors::ErrorKind::InvalidIssuer);
            return Ok(());
        }
        anyhow::bail!("No error ocurred while expecting InvalidIssuer");
    }

    #[test]
    fn decode_token_expired() -> anyhow::Result<()> {
        let issued_at = Jwt::create_issued_at() - 900;

        let jwt = Jwt::new(
            "INVALID_ISSUER".to_string(),
            SUBJECT.to_string(),
            issued_at,
            EXPIRED_DELTA,
        )
        .encode(&ENCODING_KEY)?;

        let decode_result = Jwt::decode(&jwt, ISSUER, &DECODING_KEY);

        assert!(decode_result.is_err());

        if let Error::Token(err) = decode_result.unwrap_err() {
            assert_eq!(
                err.kind(),
                &jsonwebtoken::errors::ErrorKind::ExpiredSignature
            );
            return Ok(());
        }
        anyhow::bail!("No error ocurred while expecting ExpiredSignature");
    }
}
