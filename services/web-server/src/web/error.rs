/*use lib_auth::oauth::AuthErrorInternal;
// Need to do this magic as we don't have the error in the same crate.
pub struct AuthError(pub AuthErrorInternal);

impl IntoResponse for AuthError {
	fn into_response(self) -> Response {
		// Destructure the new type wrapper to access the inner error
		let AuthError(inner_error) = self;
		debug!("{}",inner_error);
		// Match against the inner error
		let (status, error_message) = match inner_error {
			AuthErrorInternal::Request(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Making request to Oauth Provider: {}", e)),
			AuthErrorInternal::TokenError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
			AuthErrorInternal::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized!".to_string()),
			AuthErrorInternal::OptionError => (StatusCode::INTERNAL_SERVER_ERROR, "Attempted to get a non-none value but found none".to_string(), ),
			AuthErrorInternal::ParseIntError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
			AuthErrorInternal::FromRequestPartsError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
		};

		let body = Json(json!({
            "error": error_message,
        }));
		(status, body).into_response()
	}
}

impl From<AuthErrorInternal> for AuthError {
	fn from(err: AuthErrorInternal) -> Self {
		AuthError(err)
	}
}
*/
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Attempted getting request stamp")]
	// -- Extractors
	ReqStampNotInReqExt,
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		let (status, error_message) = match self {
			Error::ReqStampNotInReqExt => (StatusCode::INTERNAL_SERVER_ERROR, "Attempted getting request stamp".to_string()),
		};
		let body = Json(json!({
            "error": error_message,
        }));
		(status, body).into_response()
	}
}