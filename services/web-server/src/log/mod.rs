use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::json;
use serde_with::skip_serializing_none;
use time::Duration;
use tracing::debug;

use lib_utils::time::{format_time, now_utc};

use crate::web::mw_request_stamp::RequestStamp;

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
	uuid: String,
	timestamp: String,
	time_in: String,
	duration_ms: f64,
	http_path: String,
	http_method: String,
}

pub async fn log_request(
	http_method: Method,
	uri: Uri,
	req_stamp: RequestStamp,
) {
	// -- Prep Req Information
	let RequestStamp { uuid, time_in } = req_stamp;
	let now = now_utc();
	let duration: Duration = now - time_in;
	// duration_ms in milliseconds with microseconds precision.
	let duration_ms = (duration.as_seconds_f64() * 1_000_000.).floor() / 1_000.;

	let log_line = RequestLogLine {
		uuid: uuid.to_string(),
		timestamp: format_time(now), // LogLine timestamp ("time_out")
		time_in: format_time(time_in),
		duration_ms,

		http_path: uri.to_string(),
		http_method: http_method.to_string(),
	};

	debug!("REQUEST LOG LINE:\n{}", json!(log_line));
}