use crate::error::AppError;
use crate::models::User;
use jwt_simple::prelude::*;

const JWT_DURATION: u64 = 60 * 60 * 24 * 7;
const JWT_ISS: &str = "chat_server";

const JWT_AUD: &str = "chat_web";

pub struct EncodingKey(Ed25519KeyPair);

#[allow(unused)]
pub struct DecodingKey(Ed25519PublicKey);

impl EncodingKey {
    pub fn load(pem: &str) -> Result<Self, AppError> {
        Ok(Self(Ed25519KeyPair::from_pem(pem)?))
    }

    pub fn sign(&self, user: impl Into<User>) -> Result<String, AppError> {
        let claims = Claims::with_custom_claims(user.into(), Duration::from_secs(JWT_DURATION));
        let claims = claims.with_issuer(JWT_ISS).with_audience(JWT_AUD);
        Ok(self.0.sign(claims)?)
    }
}

impl DecodingKey {
    pub fn load(pem: &str) -> Result<Self, AppError> {
        Ok(Self(Ed25519PublicKey::from_pem(pem)?))
    }

    #[allow(unused)]
    pub fn verify(&self, token: &str) -> Result<User, AppError> {
        let opts = VerificationOptions {
            allowed_issuers: Some(HashSet::from_strings(&[JWT_ISS])),
            allowed_audiences: Some(HashSet::from_strings(&[JWT_AUD])),
            ..Default::default()
        };

        let claims = self.0.verify_token::<User>(token, Some(opts))?;
        Ok(claims.custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn jwt_sign_verify_should_work() -> Result<()> {
        let encoding_pem = include_str!("../../fixtures/encoding.pem");
        let decoding_pem = include_str!("../../fixtures/decoding.pem");
        let ek = EncodingKey::load(encoding_pem)?;
        let dk = DecodingKey::load(decoding_pem)?;

        // let user = User::new(1, "Tyr Chen", "tchen@acme.org");
        let user = User {
            id: 1,
            ws_id: 1,
            fullname: "zhangSan".to_string(),
            email: "test@mail.com".to_string(),
            password_hash: Some("111".parse()?),
            created_at: chrono::Utc::now(),
        };

        let token = ek.sign(user.clone())?;
        let user2 = dk.verify(&token)?;

        assert_eq!(user, user2);
        Ok(())
    }
}
