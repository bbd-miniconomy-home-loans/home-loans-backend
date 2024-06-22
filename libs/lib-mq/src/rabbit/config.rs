use std::sync::OnceLock;

use lib_utils::envs::{get_env, get_env_parse};

pub fn rabbit_config() -> &'static RabbitConfig {
	static INSTANCE: OnceLock<RabbitConfig> = OnceLock::new();

	INSTANCE.get_or_init(|| {
		RabbitConfig::load_from_env().unwrap_or_else(|ex| {
			panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
		})
	})
}

#[allow(non_snake_case)]
pub struct RabbitConfig {
	pub HOST: String,
	pub PORT: u16,
	pub USERNAME: String,
	pub PASSWORD: String,
}

impl RabbitConfig {
	fn load_from_env() -> lib_utils::envs::Result<RabbitConfig> {
		Ok(RabbitConfig {
			HOST: get_env("RABBIT_HOST")?,
			PORT: get_env_parse("RABBIT_PORT")?,
			USERNAME: get_env("RABBIT_USERNAME")?,
			PASSWORD: get_env("RABBIT_PASSWORD")?,
		})
	}
}