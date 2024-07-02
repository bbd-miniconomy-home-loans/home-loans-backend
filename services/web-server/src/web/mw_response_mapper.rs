use axum::http::{Method, Uri};
use axum::response::Response;
use tracing::debug;

use crate::log::log_request;
use crate::web::mw_request_stamp::RequestStamp;

pub async fn mw_response_mapper(
	uri: Uri,
	req_method: Method,
	req_stamp: RequestStamp,
	res: Response,
) -> Response {
	debug!("{:<12} - mw_reponse_map", "RES_MAPPER");

	let _ = log_request(
		req_method,
		uri,
		req_stamp,
	)
		.await;

	// Maybe we could do something cool with errors here, but for now we skip cool stuff.
	// Show user that request with uuid has failed etc..
	res
}