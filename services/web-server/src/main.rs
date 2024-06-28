use std::error::Error;
use std::process;
use std::sync::Arc;

use axum::ServiceExt;
use base64::prelude::BASE64_STANDARD;
use tokio::net::TcpListener;
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::format;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use lib_loki::set_up_loki;
use lib_mq::rabbit;
use lib_mq::rabbit::rabbitmq::RabbitMQ;
use lib_utils::envs::get_env;

use crate::web::routes;

mod error;
mod web;
mod config;
mod log;
// mod log;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	// Setup out dot env environment.
	//update ?
	dotenvy::dotenv().ok();
	/*let (trace_layer, watcher_task) = set_up_loki("home-loans-frontend")
		.expect("Error setting up loki");

	let filter = EnvFilter::builder()
		.with_default_directive(LevelFilter::TRACE.into())
		.parse("")
		.unwrap();
	tracing_subscriber::registry()
		.with(filter)
		.with(trace_layer)
		.with(tracing_subscriber::fmt::Layer::new())
		.init();
*/
	let state = AppState { /*rabbit_mq: RabbitMQ::new().await?*/ };
	let app = routes::init_router(state);

	let port = get_env("SERVER_PORT")?;
	let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
	info!("🚀 Server started successfully");
	// info!("{:<12} - http://{:?}\n", "LISTENING", listener.local_addr()?);
	info!("{:<12} - http://localhost:8080\n", "LISTENING");

	// Spawn our watcher.
	// tokio::spawn(watcher_task);
	axum::serve(listener, app.into_make_service()).await?;
	Ok(())
}

#[derive(Clone)]
struct AppState {
	// rabbit_mq: RabbitMQ,
	// user_repo: Arc<dyn >,
}