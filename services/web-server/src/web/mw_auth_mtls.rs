use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::{header, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use rustls::RootCertStore;
use serde::Serialize;
use tracing::debug;
use lib_auth::oauth::AuthErrorInternal::Unauthorized;

use crate::AppState;
use crate::web::mw_auth::AuthError::TokenNotInHeaders;

// could do this in extractor as well
pub async fn mw_auth_mtls(
	req: Request<Body>,
	next: Next,
) -> impl IntoResponse {


	// Extract the access token from headers
	let cert = req.headers()
		.get("X-Amzn-Mtls-Clientcert")
		.and_then(|auth_header| auth_header.to_str().ok());
	if let Some(cert) = cert {
		let store = RootCertStore::empty();
		
		
		
		let response = next.run(req).await;
		StatusCode::OK
	} else {
		StatusCode::UNAUTHORIZED
	}



}


type AuthResult<T> = core::result::Result<T, AuthError>;

#[derive(Clone, Serialize, Debug)]
pub enum AuthError {
	TokenNotInHeaders,
	TokenWrongFormat,
	FailValidate,
}