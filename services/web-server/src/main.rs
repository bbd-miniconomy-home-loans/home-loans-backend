use std::error::Error;
use std::sync::Arc;

use tokio::net::TcpListener;
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing::log::debug;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use lib_loki::set_up_loki;
use lib_queue::{MessageData, QueueTrait};
use lib_queue::sqs::Sqs;
use lib_utils::envs::get_env;

use crate::web::routes;

mod web;
mod log;
// mod log;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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

	let app = routes::init_router(state.clone());

	let port = get_env("SERVER_PORT")?;
	let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
	info!("🚀 Server started successfully");
	debug!("{:<12} - http://{:?}\n", "LISTENING", listener.local_addr()?);
	let message_queue_url = get_env("HOME_LOAN_MESSAGE_QUEUE_URL").expect("We need message queue");
	tokio::spawn({
		// Yay, some hacks for tokio and its instance on 'static
		let app_state = state.clone();
		async move {
			app_state.sqs.receive_message_from_queue(&message_queue_url, |a: (String, MessageData<web::routes_home_loan::LoanRequestUuid>)| async {
				debug!("Receive message from queue");
				let value = a.1.data;
				app_state.sqs.delete_message_from_queue(&message_queue_url, a.0).await.unwrap();
			}).await.unwrap();
		}
	});
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