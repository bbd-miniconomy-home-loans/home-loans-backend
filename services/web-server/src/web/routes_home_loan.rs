use aide::axum::IntoApiResponse;
use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_typed_routing::api_route;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::AppState;

#[derive(Serialize, Deserialize, JsonSchema, Validate)]
struct LoanRequest {
	#[validate(length(min = 1, message = "Can not be empty"))]
	user_id: String,
	#[validate(length(min = 1, message = "Can not be empty"))]
	property_id: String,
	#[validate(range(min = 0.0, message = "Must be positive"))]
	amount_of_loan: f64,
	#[validate(range(min = 1, message = "Must be at least 1 year"))]
	loan_term_years: u8,
}

#[derive(Serialize)]
struct ErrorResponse {
	errors: Vec<String>,
}

#[api_route(POST "/home_loan"  {
summary: "Requests a new home loan",
description: "Requests a new home loan",
id: "post-home-loan",
tags: ["home loan"],
// responses: {200: Json < LoanRequest >, }
})]
pub async fn home_loan_request_handler(
	State(state): State<AppState>,
	Json(loan_request): Json<LoanRequest>,
) -> impl IntoApiResponse {
	// Validate the loan request
	if let Err(validation_errors) = loan_request.validate() {
		let errors: Vec<String> = validation_errors
			.field_errors()
			.iter()
			.flat_map(|(_, errors)| errors.iter().map(|e| e.message.as_deref().unwrap_or("Invalid input").to_string()))
			.collect();
		return (StatusCode::BAD_REQUEST, Json(ErrorResponse { errors }));
		// return format!("Validation error: {:?}", validation_errors);
	}

	// Create a message wrapper here and send on ->
	let message_uuid = Uuid::new_v4();
	// This is a nightmare to figure out.
	// It seems like most people are just like ok,we have got the message.

	// loan_request
	// state.rabbit_mq.send_message_to_queue(loan_request);

	// Assume we make a call to our queue.

	// In the queue handler
	// --
	// 	    Make a call to Central Revenue Service with the total amount of loan.
	// 	    Calculate repayments needed over x time
	// 	    Get the base rate to calculate
	// 	    Store this stuff in the database.
	// --

	// Assume we have a way to get stuff back from the queue - Eish :(
	// this will be on the return queue
	//
	// state.rabbit_mq.receive_messages_from_queue(message_uuid)
	// Maybe create a generic api return type that will have optional data and optional error message so we dont have this error...
	(StatusCode::OK, Json("Loan request received".to_string()))
}


#[cfg(test)]
mod tests {
	use axum::body::Body;
	use axum::http;
	use axum::http::{Request, StatusCode};
	use dotenvy::dotenv;
	use http_body_util::BodyExt;
	use serde_json::json;
	use tower::ServiceExt;

	use lib_mq::rabbit::rabbitmq::RabbitMQ;

	use crate::AppState;
	use crate::web::routes;

	#[tokio::test]
	async fn test_valid_loan_request() {
		dotenv().ok();
		let state = AppState { rabbit_mq: RabbitMQ::new().await.unwrap() };
		let app = routes::init_router(state);
		let valid_request = json!({
			"user_id": "user123",
			"property_id": "property456",
			"amount_of_loan": 150000.0,
			"loan_term_years": 30
		});
		let response = app
			.oneshot(Request::builder()
				.method("POST")
				.header(http::header::CONTENT_TYPE, "application/json")
				.uri("/api/home_loan")
				.body(Body::from(valid_request.to_string()))
				.unwrap())
			.await.unwrap();
		assert_eq!(response.status(), StatusCode::OK);
		let body = response.into_body().collect().await.unwrap().to_bytes();
		assert_eq!(&body[..], b"Loan request received");
	}


	#[tokio::test]
	async fn test_invalid_loan_request() {
		dotenv().ok();
		let state = AppState { rabbit_mq: RabbitMQ::new().await.unwrap() };
		let app = routes::init_router(state);
		let invalid_request = json!({
			"user_id": "",
			"property_id": "",
			"amount_of_loan": -5000.0,
			"loan_term_years": 0
		});
		let response = app
			.oneshot(Request::builder()
				.method("POST")
				.header(http::header::CONTENT_TYPE, "application/json")
				.uri("/api/home_loan")
				.body(Body::from(invalid_request.to_string()))
				.unwrap())
			.await.unwrap();
		assert_eq!(response.status(), StatusCode::OK);
		let body = response.into_body().collect().await.unwrap().to_bytes();
		assert!(body.starts_with(b"Validation error:"));

		println!("{:?}", str::from_utf8(body));
	}
}
