use async_trait::async_trait;
use axum::body::Body;
use axum::extract::{FromRequestParts, Request};
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;
use time::OffsetDateTime;
use tracing::debug;
use uuid::Uuid;

use lib_utils::time::now_utc;

use crate::web;
use crate::web::error::Error;

#[derive(Debug, Clone)]
pub struct RequestStamp {
	pub uuid: Uuid,
	pub time_in: OffsetDateTime,
}


pub async fn mw_request_stamp_resolver(
	mut req: Request<Body>,
	next: Next,
) -> Response {
	debug!("{:<12} - mw_req_stamp_resolver", "MIDDLEWARE");
	let time_in = now_utc();
	let uuid = Uuid::new_v4();
	req.extensions_mut().insert(RequestStamp { uuid, time_in });
	next.run(req).await
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for RequestStamp {
	type Rejection = Error;
	async fn from_request_parts(parts: &mut Parts, _state: &S) -> web::error::Result<Self> {
		debug!("{:<12} - ReqStamp", "EXTRACTOR");

		parts
			.extensions
			.get::<RequestStamp>()
			.cloned()
			.ok_or(Error::ReqStampNotInReqExt)
	}
}