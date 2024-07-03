use std::{env, fs};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Cursor};
use std::sync::Arc;

use aws_config::{BehaviorVersion, Region, SdkConfig};
use aws_config::meta::region::RegionProviderChain;
use rustls::internal::msgs::handshake::CertificateChain;
// mod log;
use rustls::RootCertStore;
use sqlx::{Pool, Postgres};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use tokio::net::TcpListener;
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing::log::debug;
use tracing_loki::Layer;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use x509_parser::pem::Pem;

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

	let (trace_layer, watcher_task) = set_up_loki("home-loans-frontend")
		.expect("Error setting up loki");
	setup_logging(trace_layer);
	let db = setup_db().await?;

	let config = setup_aws_config().await;
	/*let client = aws_sdk_s3::Client::new(&config);
	let resp = client.get_object().bucket("miniconomy-trust-store-bucket").key("").send().await?;
	let data = resp.body.collect().await?.into_bytes();
	let cert_content = String::from_utf8_lossy(&data);*/


	let x = "";
	let mut cursor = Cursor::new(&x);
	let (pem, _) = Pem::read(&mut cursor)?;
	let result = pem.parse_x509().unwrap();
	let x1 = result.verify_signature(None);
	println!("{}", result.subject().to_string());
	println!("{:?}", x1);

	// let cert_bytes = decode(cert_string)?;


	/*	let query = "
			SELECT table_name
			FROM information_schema.tables;
		";

		let tables: Vec<(String, )> = sqlx::query_as(query)
			.fetch_all(&db)
			.await?;

		println!("{:?}", tables);
	*/

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


	setup_sqs_handler(state);
	// Spawn our watcher.
	tokio::spawn(watcher_task);
	axum::serve(listener, app.into_make_service()).await?;
	Ok(())
}

async fn setup_aws_config() -> SdkConfig {
	let provider = RegionProviderChain::first_try(Region::new("eu-west-1"))
		.or_else(Region::new("eu-west-1"));
	let config = aws_config::defaults(BehaviorVersion::latest())
		.region(provider)
		.load().await;
	config
}


fn setup_sqs_handler(state: AppState) {
	// Spawn our home loan queue, ideally this would be extracted to different package,
	// and we would have a system for multiple queues.
	let message_queue_url = get_env("HOME_LOAN_MESSAGE_QUEUE_URL").expect("We need message queue");

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
}

fn setup_logging(trace_layer: Layer) {
	let filter = EnvFilter::builder()
		.with_default_directive(LevelFilter::DEBUG.into())
		.parse("")
		.unwrap();
	tracing_subscriber::registry()
		.with(filter)
		.with(trace_layer)
		.with(tracing_subscriber::fmt::Layer::new())
		.init();
}

async fn setup_db() -> Result<Pool<Postgres>, Box<dyn Error>> {
	let db_username = get_env("DB_USERNAME")?;
	let db_password = get_env("DB_PASSWORD")?;
	let db_database = get_env("DB_DATABASE")?;
	let db_host = get_env("DB_HOST")?;


	let options = PgConnectOptions::default()
		.username(&db_username)
		.password(&db_password)
		.database(&db_database)
		.host(&db_host)
		.ssl_mode(PgSslMode::Require);
	let db = PgPoolOptions::new()
		.max_connections(20)
		.connect_with(options)
		.await?;
	Ok(db)
}

#[derive(Clone)]
struct AppState {
	pub sqs: Arc<Sqs>,
	pub prop_repo: Arc<PropertySalesRepoEnum>,
	pub db: Arc<Pool<Postgres>>,
	// user_repo: Arc<dyn >,
}