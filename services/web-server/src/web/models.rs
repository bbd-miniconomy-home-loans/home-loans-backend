use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, JsonSchema, Validate, Debug)]
pub struct LoanRequest {
	#[validate(length(min = 1, message = "Can not be empty"))]
	pub candidate_id: String,
	#[validate(length(min = 1, message = "Can not be empty"))]
	pub property_id: String,
	pub loan_amount_cents: u128,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct LoanRequestUuid {
	pub id: Uuid,
	pub loan_request: LoanRequest,
}


#[derive(Deserialize, Serialize, JsonSchema)]
pub struct DataResult<T> where T: Serialize
{
	pub success: bool,
	pub data: Option<T>,
	pub errors: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Persona {
	pub persona_id: u32,
	pub name: String,
}
