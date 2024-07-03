use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, JsonSchema, sqlx::Type)]
#[sqlx(type_name = "loan_status_enum")]
#[sqlx(rename_all = "lowercase")] 
pub enum LoanStatus {
	Pending, 
	Approved, 
	Rejected, 
	Completed
}

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

#[derive(Deserialize, Serialize, JsonSchema, sqlx::FromRow)]
pub struct PersonaDetails {
	pub persona_id: Option<String>,
	pub loan_status: Option<LoanStatus>,
	pub installment_amount_cents: Option<i32>,
	pub interest_rate: Option<f32>
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct PersonaResult {
	pub success: bool,
	pub data: Option<Vec<PersonaDetails>>,
	pub errors: Option<String>,
	pub flex: String
}