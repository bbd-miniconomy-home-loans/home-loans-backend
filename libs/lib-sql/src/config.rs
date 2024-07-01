use std::sync::OnceLock;

use lib_utils::envs::{get_env, get_env_parse};

pub fn config() -> &'static SqlxConfig {
	static INSTANCE: OnceLock<SqlxConfig> = OnceLock::new();
	INSTANCE.get_or_init(|| SqlxConfig::load_from_env().unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")))
}

#[allow(non_snake_case)]
pub struct SqlxConfig {
	pub DATABASE_URL: String,
	pub MAX_CONNECTIONS: u8,

}

impl SqlxConfig {
	fn load_from_env() -> lib_utils::envs::Result<SqlxConfig> {
		Ok(SqlxConfig {
			DATABASE_URL: get_env("DATABASE_URL")?,
			MAX_CONNECTIONS: get_env_parse("max_connections")?,
		})
	}
}