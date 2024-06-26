use std::sync::OnceLock;

use lib_utils::envs::{get_env, get_env_parse};

pub fn config() -> &'static LokiConfig {
	static INSTANCE: OnceLock<LokiConfig> = OnceLock::new();
	INSTANCE.get_or_init(|| LokiConfig::load_from_env().unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")))
}

#[allow(non_snake_case)]
pub struct LokiConfig {
	pub GRAFANA_USER: String,
	pub GRAFANA_TOKEN: String,
	pub GRAFANA_URL: String,
}

impl LokiConfig {
	fn load_from_env() -> lib_utils::envs::Result<LokiConfig> {
		Ok(LokiConfig {
			GRAFANA_USER: get_env("GRAFANA_USER")?,
			GRAFANA_TOKEN: get_env("GRAFANA_TOKEN")?,
			GRAFANA_URL: get_env_parse("GRAFANA_URL")?,
		})
	}
}