use std::error;
use derive_more::From;
use serde_with::serde_as;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, From)]
pub enum Error {
	NoMessageId,
	#[from]
	SerdeError(serde_json::Error),
	#[from]
	AwsError(aws_sdk_sqs::Error),
	// #[from]
	//It his here aws_smithy_runtime_api::client::result::SdkError, but it is not good.
	// More for their ported stuff.
	SmithyTypesAWSDoesNotWantUsToKnowAboutCusTheyCheat,

}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate