use jsonwebtoken::{decode, encode, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::config::auth_config;
use crate::token::error::Result;
use crate::token::JwtAuthErrorInternal;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
	pub username: String,
	pub exp: usize,
}

pub fn decode_token(token: &str) -> Result<Claims> {
	let config = auth_config();
	let token_data = decode::<Claims>(token, &config.JWT_KEYS.decoding, &Validation::default())
		.map(|token_data| token_data.claims)
		.map_err(|e| JwtAuthErrorInternal::InvalidToken(e))?;
	Ok(token_data)
}

pub fn encode_token(claims: Claims) -> Result<String> {
	let config = auth_config();

	let token = encode(&Header::default(), &claims, &config.JWT_KEYS.encoding)
		.map_err(|e| JwtAuthErrorInternal::TokenCreation(e))?;
	Ok(token)
}