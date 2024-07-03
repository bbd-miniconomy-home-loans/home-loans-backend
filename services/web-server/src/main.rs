use std::{env, fs};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Cursor};
use std::os::raw::c_long;
use std::sync::Arc;

use aws_config::{BehaviorVersion, Region, SdkConfig};
use aws_config::meta::region::RegionProviderChain;
use reqwest::ClientBuilder;
use reqwest::header::HeaderMap;
use rustls::internal::msgs::handshake::CertificateChain;
// mod log;
use rustls::RootCertStore;
use sqlx::{Pool, Postgres};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use tokio::net::TcpListener;
use tracing::{error, info};
use tracing::level_filters::LevelFilter;
use tracing::log::debug;
use tracing_loki::Layer;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use uuid::Uuid;
use x509_parser::nom::Parser;
use x509_parser::pem::Pem;
use lib_intg::repos::commercial_bank_repo::{CommercialBankRepoEnum, CommercialInMemoryRepo, CommercialRepo, CommercialRepoTrait};
use lib_intg::repos::commercial_bank_repo::CommercialBankRepoEnum::{CommercialInMemoryRepoE, CommercialRepoE};

use lib_intg::repos::property_sales_repo::{PropertyInMemoryRepo, PropertySalesRepo, PropertySalesRepoEnum, PropertySalesRepoTrait};
use lib_intg::repos::property_sales_repo::PropertySalesRepoEnum::{InMemoryRepo, PropertySalesRepoR};
use lib_intg::repos::retail_bank_repo::{RetailBankInMemoryRepo, RetailBankRepoEnum, RetailBankRepoTrait, RetailBankSalesRepo};
use lib_intg::repos::retail_bank_repo::RetailBankRepoEnum::{RetailInMemoryRepoE, RetailSalesRepoE};
use lib_intg::repos::stock_exchange_repo::{StockExchangeRepo, StockExchangeRepoEnum, StockExchangeRepoInMemoryRepo};
use lib_intg::repos::stock_exchange_repo::StockExchangeRepoEnum::{StockExchangeInMemoryRepoE, StockExchangeRepoE};
use lib_intg::repos::tax_repo::{TaxInMemoryRepo, TaxRepo, TaxRepoEnum};
use lib_intg::repos::tax_repo::TaxRepoEnum::{TaxInMemoryRepoE, TaxRepoR};
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

    /* sqlx::migrate!("migrations")
         .run(&db)
         .await?;*/
    /*let client = aws_sdk_s3::Client::new(&config);
    let resp = client.get_object().bucket("miniconomy-trust-store-bucket").key("").send().await?;
    let data = resp.body.collect().await?.into_bytes();
    let cert_content = String::from_utf8_lossy(&data);*/


    /*   let x = "";
       let mut cursor = Cursor::new(&x);
       let (pem, _) = Pem::read(&mut cursor)?;
       let result = pem.parse_x509().unwrap();
       let x1 = result.verify_signature(None);
       println!("{}", result.subject().to_string());
       println!("{:?}", x1);*/

    // let cert_bytes = decode(cert_string)?;

    let mut map = HeaderMap::new();
    map.insert("X-Origin", "home_loans".parse()?);
    let client = ClientBuilder::new()
        .gzip(true)
        .default_headers(map)
        .build()?;

    let arc = Arc::new(client);


    let state = AppState {
        sqs: Arc::new(Sqs::new().await),
        prop_repo: Arc::new(PropertySalesRepoR(PropertySalesRepo { client: arc.clone() })),
        tax_repo: Arc::new(TaxRepoR(TaxRepo { client: arc.clone() })),
        stock_exchange_repo: Arc::new(StockExchangeRepoE(StockExchangeRepo { client: arc.clone() })),
        retail_bank_repo: Arc::new(RetailSalesRepoE(RetailBankSalesRepo { client: arc.clone() })),
        commercial_bank_repo: Arc::new(CommercialRepoE(CommercialRepo { client: arc.clone() })),
        db,
    };

    let app = routes::init_router(state.clone());

    let port = get_env("SERVER_PORT")?;
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    info!("🚀 Server started successfully");
    debug!("{:<12} - http://{:?}\n", "LISTENING", listener.local_addr()?);
    state.clone().retail_bank_repo.send_status(Uuid::default(), true).await.expect("TODO: panic message");

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
                    let data_uuid = message_data.data.id;
                    let loan_request = message_data.data.loan_request;
                    // -> check if works
                    let result = app_state.commercial_bank_repo.request_balance().await;

                    if result.is_err() {
                        error!("We have error from com bank ");
                        if app_state.prop_repo.send_status(data_uuid, false).await.is_err() {
                            error!("We have error property");
                        }
                        return;
                    }
                    // Cool to unwrap here sorted above. 
                    let our_balance = result.unwrap().data.account_balance;

                    if loan_request.loan_amount_cents > our_balance {
                        error!("Loan amount is too big.");
                        if app_state.prop_repo.send_status(data_uuid, false).await.is_err() {
                            error!("We have error property");
                        }
                        return;
                    }


                    // create debit order
                    
                    
                    
                    // app_state.prop_repo.send_status(loan_reques, true).await.expect("TODO: panic message");
                    // app_state.retail_repo.send_status(loan_reques, true).await.expect("TODO: panic message");
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
    pub tax_repo: Arc<TaxRepoEnum>,
    pub stock_exchange_repo: Arc<StockExchangeRepoEnum>,
    pub retail_bank_repo: Arc<RetailBankRepoEnum>,
    pub commercial_bank_repo: Arc<CommercialBankRepoEnum>,
    pub db: Pool<Postgres>,
    // user_repo: Arc<dyn >,
}