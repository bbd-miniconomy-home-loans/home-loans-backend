use thiserror::Error;

pub type Result<T> = core::result::Result<T, AuthErrorInternal>;

#[derive(Debug, Error)]
pub enum AuthErrorInternal {
	#[error("HTTP request error: {0}")]
	Request(#[from] reqwest::Error),
	#[error("OAuth token error: {0}")]
	TokenError(
		#[from]
		oauth2::RequestTokenError<
			oauth2::reqwest::Error<reqwest::Error>,
			oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
		>,
	),
	#[error("You're not authorized!")]
	Unauthorized,
	#[error("Attempted to get a non-none value but found none")]
	OptionError,
	#[error("Attempted to parse a number to an integer but errored out: {0}")]
	ParseIntError(#[from] std::num::TryFromIntError),
	#[error("Encountered an error trying to convert an infallible value: {0}")]
	FromRequestPartsError(#[from] std::convert::Infallible),
}