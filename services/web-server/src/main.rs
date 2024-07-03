use std::error::Error;
use std::sync::Arc;

use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing::log::debug;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use lib_intg::repos::property_sales_repo::{PropertyInMemoryRepo, PropertySalesRepoEnum, PropertySalesRepoTrait};
use lib_intg::repos::property_sales_repo::PropertySalesRepoEnum::InMemoryRepo;
use lib_loki::set_up_loki;
use lib_queue::{MessageData, QueueTrait};
use lib_queue::sqs::Sqs;
use lib_utils::envs::get_env;

use web::models::LoanRequestUuid;

use crate::web::routes;

mod web;
mod log;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	dotenvy::dotenv().ok();

	let (trace_layer, watcher_task) = set_up_loki("home-loans-backend")
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

	let database_url = get_env("DATABASE_URL")?;
	let db = PgPoolOptions::new()
		.max_connections(20)
		.connect(&database_url)
		.await?;
	// sqlx::migrate!().run(&db).await?;


	let state = AppState {
		sqs: Arc::new(Sqs::new().await),
		prop_repo: Arc::new(InMemoryRepo(PropertyInMemoryRepo {})),
		db: Arc::new(db),
	};

	let app = routes::init_router(state.clone());

	let port = get_env("SERVER_PORT")?;
	let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
	
	info!("🚀 Server started successfully");
	debug!("{:<12} - http://{:?}\n", "LISTENING", listener.local_addr()?);
	
	let message_queue_url = get_env("HOME_LOAN_MESSAGE_QUEUE_URL").expect("We need message queue");

	// Spawn our home loan queue, ideally this would be extracted to different package,
	// and we would have a system for multiple queues.
	tokio::spawn({
		// Yay, some hacks for tokio and its instance on 'static
		let app_state = state.clone();

		async move {
			app_state.sqs.receive_message_from_queue(&message_queue_url, |(queue_message_handle, message_data): (String, MessageData<LoanRequestUuid>)| {
				
				let app_state = state.clone();
				let message_queue_url = message_queue_url.clone();
				
				async move {
					debug!("Receive message from queue");
					let x = message_data.data.id;

					app_state.prop_repo.send_status(x, true).await;

					app_state.sqs.delete_message_from_queue(&message_queue_url, queue_message_handle).await.unwrap();
				}
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
	pub prop_repo: Arc<PropertySalesRepoEnum>,
	pub db: Arc<Pool<Postgres>>,
}