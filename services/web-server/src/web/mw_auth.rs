use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::header;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use tracing::debug;

use crate::AppState;
use crate::web::mw_auth::AuthError::TokenNotInHeaders;

// could do this in extractor as well
pub async fn mw_auth(
	req: Request<Body>,
	next: Next,
) -> Response {


	// Extract the access token from headers
	let access_token = req.headers()
		.get(header::AUTHORIZATION)
		.and_then(|auth_header| auth_header.to_str().ok())
		.and_then(|auth_str| auth_str.strip_prefix("Bearer ").map(|token| token.to_owned()))
		.ok_or_else(|| TokenNotInHeaders);
	// TODO: Something with the token.
	
	debug!("We should be doing something with auth here");
	if access_token.is_err() {
		// TODO: no auth
	}

	next.run(req).await
}


type AuthResult<T> = core::result::Result<T, AuthError>;

#[derive(Clone, Serialize, Debug)]
pub enum AuthError {
	TokenNotInHeaders,
	TokenWrongFormat,
	FailValidate,
}