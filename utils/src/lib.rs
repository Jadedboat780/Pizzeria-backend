use crate::jwt::secret_key::KEYS;
use crate::jwt::Claims;
use jsonwebtoken::{encode, Header};

pub mod api_response;
pub mod encryption;
pub mod jwt;

pub fn encode_token(email: String) -> Result<String, jsonwebtoken::errors::Error> {
    encode(&Header::default(), &Claims::new(email), &KEYS.encoding)
}
