use std::error::Error;
use std::sync::Arc;

use tokio::net::TcpListener;
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use lib_loki::set_up_loki;
use lib_queue::sqs::Sqs;
use lib_utils::envs::get_env;

use crate::web::routes;

mod error;
mod web;
mod config;
mod log;
// mod log;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	// TODO: Secrets, should we use aws to pull them from secrets manager 
	dotenvy::dotenv().ok();

	let (trace_layer, watcher_task) = set_up_loki("home-loans-frontend")
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

	let state = AppState {
		sqs: Arc::new(Sqs::new().await)
	};

	let app = routes::init_router(state);

	let port = get_env("SERVER_PORT")?;
	let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
	info!("🚀 Server started successfully");
	// info!("{:<12} - http://{:?}\n", "LISTENING", listener.local_addr()?);
	// info!("{:<12} - http://localhost:8080\n", "LISTENING");

	// Spawn our watcher.
	tokio::spawn(watcher_task);
	axum::serve(listener, app.into_make_service()).await?;
	Ok(())
}

#[derive(Clone)]
struct AppState {
	pub sqs: Arc<Sqs>,
	// user_repo: Arc<dyn >,
}