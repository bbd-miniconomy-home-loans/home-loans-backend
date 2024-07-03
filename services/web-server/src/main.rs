use std::error::Error;
use std::sync::Arc;

use reqwest::ClientBuilder;
use reqwest::header::HeaderMap;
use sqlx::{Pool, Postgres, query};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use tokio::net::TcpListener;
use tracing::{error, info};
use tracing::level_filters::LevelFilter;
use tracing::log::debug;
use tracing_loki::Layer;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use lib_intg::repos::commercial_bank_repo::CommercialBankRepoEnum;
use lib_intg::repos::commercial_bank_repo::CommercialRepo;
use lib_intg::repos::commercial_bank_repo::CommercialRepoTrait;
use lib_intg::repos::commercial_bank_repo::CommercialBankRepoEnum::CommercialRepoE;
use lib_intg::repos::property_sales_repo::PropertySalesRepo;
use lib_intg::repos::property_sales_repo::PropertySalesRepoEnum;
use lib_intg::repos::property_sales_repo::PropertySalesRepoTrait;
use lib_intg::repos::property_sales_repo::PropertySalesRepoEnum::PropertySalesRepoE;
use lib_intg::repos::retail_bank_repo::{RetailBankRepoEnum, RetailBankSalesRepo};
use lib_intg::repos::retail_bank_repo::RetailBankRepoEnum::RetailSalesRepoE;
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

    let mut map = HeaderMap::new();
    map.insert("X-Origin", "home_loans".parse()?);
    let client = ClientBuilder::new()
        .gzip(true)
        .default_headers(map)
        .build()?;

    let arc = Arc::new(client);


    let state = AppState {
        sqs: Arc::new(Sqs::new().await),
        prop_repo: Arc::new(PropertySalesRepoE(PropertySalesRepo { client: arc.clone() })),
        _retail_bank_repo: Arc::new(RetailSalesRepoE(RetailBankSalesRepo { client: arc.clone() })),
        commercial_bank_repo: Arc::new(CommercialRepoE(CommercialRepo { client: arc.clone() })),
        db,
    };

    let app = routes::init_router(state.clone());

    let port = get_env("SERVER_PORT")?;
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    info!("🚀 Server started successfully");
    debug!("{:<12} - http://{:?}\n", "LISTENING", listener.local_addr()?);

    setup_sqs_handler(state);
    // Spawn our watcher.
    tokio::spawn(watcher_task);
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
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
                    let our_account = app_state.commercial_bank_repo.request_balance().await;

                    if our_account.is_err() {
                        error!("We have error from com bank ");
                        if app_state.prop_repo.send_status(data_uuid, false).await.is_err() {
                            error!("We have error property");
                        }
                        return;
                    }
                    // Cool to unwrap here sorted above. 
                    let our_account = our_account.unwrap().data;
                    let our_balance = our_account.account_balance;

                    if loan_request.loan_amount_cents > our_balance {
                        error!("Loan amount is too big.");
                        if app_state.prop_repo.send_status(data_uuid, false).await.is_err() {
                            error!("We have error property");
                        }
                        return;
                    }

                    if app_state.commercial_bank_repo.send_transaction(loan_request.loan_amount_cents.clone(), loan_request.candidate_id.clone(), our_account.account_name).await.is_err()
                    {
                        error!("Commercial bank repo failed ");
                    }

                    let insert_query = query("INSERT INTO persona(persona_id, is_active, created_at) VALUES ($1, true, CURRENT_TIMESTAMP)").bind(&loan_request.candidate_id);
                    if insert_query.execute(&app_state.db).await.is_err() {
                        error!("Unable to insert into db");
                    };
                    let insert_query = query("INSERT INTO loan (persona_id, loan_amount_cents, installment_amount_cents, loan_status, interest_rate, approval_date, created_at) VALUES ($1, $2, $3, 'approved', $4, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
                        .bind(&loan_request.candidate_id)
                        .bind(&loan_request.loan_amount_cents)
                        .bind(&loan_request.loan_amount_cents / 24)
                        .bind(5)
                        ;
                    if insert_query.execute(&app_state.db).await.is_err() {
                        error!("Unable to insert loan into db");
                    };

                    app_state.sqs.delete_message_from_queue(&message_queue_url, queue_message_handle).await.unwrap();
                    if app_state.prop_repo.send_status(data_uuid, true).await.is_err() {
                        error!("We have error property");
                    }
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
    pub _retail_bank_repo: Arc<RetailBankRepoEnum>,
    pub commercial_bank_repo: Arc<CommercialBankRepoEnum>,
    pub db: Pool<Postgres>,
}