use std::error::Error;

use axum::ServiceExt;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

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
	dotenvy::dotenv().ok();

	// Setup tracing for with environment default filter.
	tracing_subscriber::fmt()
		.with_env_filter(EnvFilter::from_default_env())
		.init();

	let state = AppState { rabbit_mq: RabbitMQ::new().await? };
	let app = routes::init_router(state);

	let url = get_env("URL")?;
	let listener = TcpListener::bind(url).await?;
	info!("🚀 Server started successfully");
	// info!("{:<12} - http://{:?}\n", "LISTENING", listener.local_addr()?);
	info!("{:<12} - http://localhost:8080\n", "LISTENING");

	axum::serve(listener, app.into_make_service()).await?;
	Ok(())
}

#[derive(Clone)]
struct AppState {
	rabbit_mq: RabbitMQ,
}