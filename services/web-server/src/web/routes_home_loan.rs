use axum::extract::{Json, State};
use axum_typed_routing::api_route;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;

#[derive(Serialize, Deserialize, JsonSchema)]
struct LoanRequest {
	#[validate(length(min = 1, message = "Can not be empty"))]
	user_id: String,
	property_id: String,
	amount_of_loan: f64,
	loan_term_years: u8,
}

#[api_route(POST "/home_loan" with AppState {
summary: "Requests a new home loan",
description: "Requests a new home loan",
id: "post-home-loan",
tags: ["home loan"],
})]
pub async fn home_loan_request_handler(
	State(state): State<AppState>,
	Json(loan_request): Json<LoanRequest>,
) -> String {
	// Create a message wrapper here and send on ->
	let message_uuid = Uuid::new_v4();

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

	// Assume we have a way to get stuff back from the queue
	// this will be on the return queue
	//
	// state.rabbit_mq.receive_messages_from_queue(message_uuid)

	"".to_string()
}




