pub mod key;
pub mod models;

use api_response::{ApiError, AuthError};
use axum::{extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Header, Validation};
use key::KEYS;
use serde::{Deserialize, Serialize};

pub fn encode_token(email: String) -> Result<String, jsonwebtoken::errors::Error> {
    encode(&Header::default(), &Claims::new(email), &KEYS.encoding)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(email: String) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(24);

        Self {
            sub: email,
            exp: exp.timestamp(),
            iat: iat.timestamp(),
        }
    }
}

#[derive(Deserialize)]
pub struct JWT(pub Claims);

impl<S> FromRequestParts<S> for JWT
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| ApiError::AuthError(AuthError::InvalidToken))?;

        let token_data = decode(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| ApiError::AuthError(AuthError::InvalidToken))?;

        Ok(token_data.claims)
    }
}
