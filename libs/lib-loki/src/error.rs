use derive_more::From;
use serde_with::serde_as;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, From)]
pub enum Error {
	#[from]
	Tracing(tracing_loki::Error),
	#[from]
	Url(url::ParseError),

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