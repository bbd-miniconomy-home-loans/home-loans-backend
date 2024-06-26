use thiserror::Error;

pub type Result<T> = core::result::Result<T, RabbitErrorInternal>;

#[derive(Debug, Error)]
pub enum RabbitErrorInternal {
	#[error("Attempted Open RABBIT MQ Error: {0}")]
	MqOpenError(#[from] amqprs::error::Error),
	#[error("Attempted to parse a number to an integer but errored out: {0}")]
	ParseIntError(#[from] std::num::TryFromIntError),
	#[error("Attempted to parse a number to an integer but errored out: ")]
	TEST(),

}