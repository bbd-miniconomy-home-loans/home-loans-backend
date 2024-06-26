use time::{Duration, OffsetDateTime};
use time::format_description::well_known::Rfc3339;

pub fn now_utc() -> OffsetDateTime {
	OffsetDateTime::now_utc()
}

pub fn format_time(time: OffsetDateTime) -> String {
	time.format(&Rfc3339).unwrap() // TODO: need to check if safe.
}

pub fn now_utc_plus_sec_str(sec: f64) -> String {
	let new_time = now_utc() + Duration::seconds_f64(sec);
	format_time(new_time)
}

pub fn parse_utc(moment: &str) -> Result<OffsetDateTime> {
	OffsetDateTime::parse(moment, &Rfc3339)
		.map_err(|_| TimeError::FailToDateParse(moment.to_string()))
}

pub type Result<T> = core::result::Result<T, TimeError>;

#[derive(Debug)]
pub enum TimeError {
	FailToDateParse(String),
}