use std::sync::OnceLock;

use jsonwebtoken::{DecodingKey, EncodingKey};
use rand::distributions::{Alphanumeric, DistString};

use lib_utils::envs::get_env;

pub fn auth_config() -> &'static AuthConfig {
	static INSTANCE: OnceLock<AuthConfig> = OnceLock::new();

	INSTANCE.get_or_init(|| {
		AuthConfig::load_from_env().unwrap_or_else(|ex| {
			panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
		})
	})
}

pub(crate) struct Keys {
	pub(crate) encoding: EncodingKey,
	pub(crate) decoding: DecodingKey,
}

impl Keys {
	fn new(secret: &[u8]) -> Self {
		Self {
			encoding: EncodingKey::from_secret(secret),
			decoding: DecodingKey::from_secret(secret),
		}
	}
}

#[allow(non_snake_case)]
pub struct AuthConfig {
	pub CLIENT_ID: String,
	pub CLIENT_SECRET: String,
	pub REDIRECT_URL: String,
	pub JWT_KEYS: Keys,
}

impl AuthConfig {
	fn load_from_env() -> lib_utils::envs::Result<AuthConfig> {
		Ok(AuthConfig {
			CLIENT_ID: get_env("CLIENT_ID")?,
			CLIENT_SECRET: get_env("CLIENT_SECRET")?,
			REDIRECT_URL: get_env("REDIRECT_URL")?,
			JWT_KEYS: Keys::new(Alphanumeric.sample_string(&mut rand::thread_rng(), 60).as_bytes()),
		})
	}
}