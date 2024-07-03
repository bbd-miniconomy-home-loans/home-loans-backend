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
    PersonaDetails,
    PersonaResult,
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

#[api_route(GET "/persona" {})]
pub async fn get_personas_handler(
    State(state): State<AppState>
) -> Json<PersonaResult> {
    let query = "
		SELECT
			p.persona_id,
			l.loan_status,
			l.installment_amount_cents,
			l.interest_rate::FLOAT4 AS interest_rate
		FROM persona p
		JOIN loan l ON p.persona_id = l.persona_id
		JOIN property pr ON p.persona_id = pr.persona_id
	";

    let res: Result<Vec<PersonaDetails>, sqlx::Error> = sqlx::query_as::<_, PersonaDetails>(query)
        .fetch_all(&state.db)
        .await;

    let result = match res {
        Ok(result) => {
            PersonaResult {
                success: true,
                data: Some(result),
                errors: None,
                flex: "Made in Rust :)".to_string(),
            }
        }
        Err(error) => {
            PersonaResult {
                success: false,
                data: None,
                errors: Some(format!("Error: {}", error)),
                flex: "Made in Rust :)".to_string(),
            }
        }
    };

    Json(result)
}
