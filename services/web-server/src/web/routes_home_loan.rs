use axum::extract::{Json, State};
use axum_typed_routing::api_route;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::{debug, error};
use tracing::field::debug;
use uuid::Uuid;
use validator::Validate;

use lib_queue::{MessageData, MessageType, QueueTrait};

use crate::AppState;

#[derive(Serialize, Deserialize, JsonSchema, Validate)]
struct LoanRequest {
	#[validate(length(min = 1, message = "Can not be empty"))]
	candidate_id: String,
	#[validate(length(min = 1, message = "Can not be empty"))]
	property_id: String,
	loan_amount_cents: u128,
}

#[derive(Serialize, Deserialize)]
struct LoanRequestUuid {
	id: Uuid,
	loan_request: LoanRequest,
}


#[derive(Deserialize, Serialize, JsonSchema)]
struct DataResult<T> where T: Serialize
{
	success: bool,
	data: Option<T>,
	errors: Option<Vec<String>>,
}

#[api_route(POST "/apply"  {
summary: "Requests a new home loan",
description: "Requests a new home loan",
id: "post-apply",
tags: ["home loan"],
responses: { 403: Json < String >}
})]
pub async fn apply_request_handler(
	State(state): State<AppState>,
	Json(loan_request): Json<LoanRequest>,
) -> Json<DataResult<String>> {
	if let Err(validation_errors) = loan_request.validate() {
		let errors: Vec<String> = validation_errors
			.field_errors()
			.iter()
			.flat_map(|(_, errors)| errors.iter().map(|e| e.message.as_deref().unwrap_or("Invalid input").to_string()))
			.collect();
		return Json(DataResult {
			success: false,
			data: None,
			errors: Some(errors),
		});
	}


	let uuid = Uuid::new_v4();
	let data = MessageData { message_type: MessageType::ADD, data: LoanRequestUuid { id: uuid, loan_request } };
	// TODO: pull from env
	let result = match state.sqs.send_message_to_queue(&"https://sqs.eu-west-1.amazonaws.com/434468814231/home_loan_queue".to_string(), data).await {
		Ok(queue_id) => {
			debug!("Added to queue {}", queue_id);
			DataResult {
				success: true,
				data: Some(uuid.to_string()),
				errors: None,
			}
		}
		Err(error) => {
			error!("Error sending message: {}",error);
			DataResult {
				success: false,
				data: None,
				errors: Some(vec!["Unable to send to queue".to_string()]),
			}
		}
	};

	Json(result)
}
/*
Tests became super difficult with traits etc. 
#[cfg(test)]
mod tests {
	use axum::body::Body;
	use axum::http;
	use axum::http::{Request, StatusCode};
	use dotenvy::dotenv;
	use http_body_util::BodyExt;
	use serde_json::json;
	use tower::ServiceExt;

	use crate::AppState;
	use crate::web::routes;

	#[tokio::test]
	async fn test_valid_loan_request() {
		dotenv().ok();
		let state = AppState { /*rabbit_mq: RabbitMQ::new().await.unwrap()*/ };
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
		let state = AppState { /*rabbit_mq: RabbitMQ::new().await.unwrap()*/ sqs: Arc::new(()) };
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
	}
}
*/