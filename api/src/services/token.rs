use rusty_paseto::prelude::*;
use serde_json::Value;
use std::convert::TryFrom;
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

use crate::errors::CustomError;

const ACCESS_TOKEN_EXPIRATION: i64 = 15; // 15 minutes
const REFRESH_TOKEN_EXPIRATION: i64 = 7 * 24 * 60; // 7 days in minutes

pub struct TokenService {
    key: PasetoSymmetricKey<V4, Local>,
}

impl TokenService {
    pub fn new() -> Result<Self, CustomError> {
        let key =
            PasetoSymmetricKey::<V4, Local>::from(Key::from(b"wubbalubbadubdubwubbalubbadubdub")); // -> 32 bytes -> no production use
        Ok(Self { key })
    }

    pub async fn generate_token_pair(
        &self,
        user_id: Uuid,
    ) -> Result<(String, String, i64, i64), CustomError> {
        let now = OffsetDateTime::now_utc();

        let access_exp = (now + Duration::minutes(ACCESS_TOKEN_EXPIRATION))
            .format(&time::format_description::well_known::Rfc3339)
            .map_err(|_| CustomError::InternalServerError)?;

        let token_id = Uuid::new_v4().to_string();
        let user_id_str = user_id.to_string();

        let access_token = PasetoBuilder::<V4, Local>::default()
            .set_claim(SubjectClaim::from(user_id_str.as_str()))
            .set_claim(TokenIdentifierClaim::from(token_id.as_str()))
            .set_claim(
                ExpirationClaim::try_from(access_exp.as_str())
                    .map_err(|_| CustomError::InternalServerError)?,
            )
            .set_claim(
                CustomClaim::try_from(("type", "access"))
                    .map_err(|_| CustomError::InternalServerError)?,
            )
            .build(&self.key)
            .map_err(|_| CustomError::InternalServerError)?;

        let refresh_exp = (now + Duration::minutes(REFRESH_TOKEN_EXPIRATION))
            .format(&time::format_description::well_known::Rfc3339)
            .map_err(|_| CustomError::InternalServerError)?;

        let user_id_str = user_id.to_string();
        let refresh_token_id = Uuid::new_v4().to_string();
        let refresh_token = PasetoBuilder::<V4, Local>::default()
            .set_claim(SubjectClaim::from(user_id_str.as_str()))
            .set_claim(TokenIdentifierClaim::from(refresh_token_id.as_str()))
            .set_claim(
                ExpirationClaim::try_from(refresh_exp.as_str())
                    .map_err(|_| CustomError::InternalServerError)?,
            )
            .set_claim(
                CustomClaim::try_from(("type", "refresh"))
                    .map_err(|_| CustomError::InternalServerError)?,
            )
            .build(&self.key)
            .map_err(|_| CustomError::InternalServerError)?;

        Ok((
            access_token,
            refresh_token,
            ACCESS_TOKEN_EXPIRATION,
            REFRESH_TOKEN_EXPIRATION,
        ))
    }

    pub async fn verify_access_token(&self, token: &str) -> Result<Value, CustomError> {
        let claims = PasetoParser::<V4, Local>::default()
            .parse(token, &self.key)
            .map_err(|_| CustomError::InvalidToken("Invalid token".to_string()))?;

        if claims["type"].as_str() != Some("access") {
            return Err(CustomError::InvalidToken("Not an access token".to_string()));
        }

        Ok(claims)
    }

    pub async fn verify_refresh_token(&self, token: &str) -> Result<String, CustomError> {
        let claims = PasetoParser::<V4, Local>::default()
            .check_claim(
                CustomClaim::try_from(("type", "refresh"))
                    .map_err(|_| CustomError::InternalServerError)?,
            )
            .parse(token, &self.key)
            .map_err(|_| CustomError::InvalidToken("Invalid refresh token".to_string()))?;

        claims["sub"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or(CustomError::InvalidToken("sub claim not found".to_string()))
    }

    pub async fn extract_user_id(&self, token: &str) -> Result<Uuid, CustomError> {
        let claims = self.verify_access_token(token).await?;

        let user_id_str = claims["sub"]
            .as_str()
            .ok_or(CustomError::InvalidToken("Missing sub claim".to_string()))?;

        Uuid::parse_str(user_id_str)
            .map_err(|_| CustomError::InvalidToken("Invalid user ID".to_string()))
    }
}
