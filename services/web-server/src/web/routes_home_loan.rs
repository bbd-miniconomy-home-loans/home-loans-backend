use axum::extract::{Json, State};
use axum_typed_routing::api_route;
use tracing::{debug, error};
use uuid::Uuid;
use validator::Validate;

use lib_queue::{MessageData, MessageType, QueueTrait};
use lib_utils::envs::get_env;

use crate::AppState;

use crate::web::models::{
	LoanRequestUuid, 
	LoanRequest, 
	DataResult, 
	Persona,
};

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
	// In a perfect world, I would handle errors correctly.
	let message_queue_url = get_env("HOME_LOAN_MESSAGE_QUEUE_URL").expect("We need message queue");
	let result = match state.sqs.send_message_to_queue(&message_queue_url, data).await {
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

// TODO: GET all personas
// TODO: POST add persona
// TODO: DELETE persona (update is_active)
// TODO: PUT persona (update)
// TODO: GET: JOIN personas JOIN loan JOIN property  

#[api_route(GET "/persona" {})]
pub async fn get_personas_handler(
	State(state): State<AppState>
) -> Json<Persona> {

	let result = Persona {
			persona_id: 1, 
			name: "we".to_string()
	};

	Json(result)
}

// TODO: get endpoints for SARS
// pub async fn sars_handler() {}

// TODO: get endpoints for Labour Broker
// TODO: GET persona salary
// pub async fn labour_broker_handler() {}

// TODO: get endpoints for Stock Exchange
// TODO: POST list our stocks
// TODO: GET, PUT, DELETE our stocks 
// pub async fn stock_exchange_handler() {}

// TODO: get endpoints for Commercial Bank
// TODO: POST send money from <home_loans> to <persona>
// TODO: GET account balance
// TODO: GET statements/Transactions
// pub async fn commercial_bank_handler() {}

// TODO: get endpoints for Retail Bank
// TODO: POST send money from <persona> to <home_loans>
// pub async fn retail_bank_handler() {}
