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