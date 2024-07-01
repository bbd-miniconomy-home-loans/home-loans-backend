use thiserror::Error;

pub type Result<T> = core::result::Result<T, JwtAuthErrorInternal>;

#[derive(Debug, Error)]
pub enum JwtAuthErrorInternal {
	#[error("Invalid token error: {0}")]
	InvalidToken(jsonwebtoken::errors::Error),
	#[error("Error creating token: {0}")]
	TokenCreation(jsonwebtoken::errors::Error),

	#[error("Wrong credentials")]
	WrongCredentials,
	#[error("Missing credentials")]
	MissingCredentials,
}