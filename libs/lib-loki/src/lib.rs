use std::process;

use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use tracing_loki::BackgroundTask;
use tracing_loki::Layer;
use url::Url;

use crate::config::config;

mod error;
mod config;

pub fn set_up_loki(application_name: &str) -> error::Result<(Layer, BackgroundTask)> {
	let config = config();
	let basic_auth = format!("{}:{}", config.GRAFANA_USER, config.GRAFANA_TOKEN);
	let encoded_basic_auth = BASE64_STANDARD.encode(basic_auth.as_bytes());
	let url = Url::parse(config.GRAFANA_URL.as_str())?;
	let (layer, task) = tracing_loki::builder()
		.label("application", application_name)?
		.extra_field("pid", format!("{}", process::id()))?
		.http_header("Authorization", format!("Basic {encoded_basic_auth}"))?
		.build_url(url)?;
	Ok((layer, task))
}