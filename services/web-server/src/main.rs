use std::error::Error;
use std::sync::Arc;

use tokio::net::TcpListener;
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing::log::{debug, error};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use lib_intg::repos::property_sales_repo::{PropertyInMemoryRepo, PropertySalesRepoEnum, PropertySalesRepoTrait};
use lib_intg::repos::property_sales_repo::PropertySalesRepoEnum::InMemoryRepo;
use lib_loki::set_up_loki;
use lib_queue::{MessageData, QueueTrait};
use lib_queue::sqs::Sqs;
use lib_utils::envs::get_env;
use routes_home_loan::LoanRequestUuid;
use web::routes_home_loan;

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
        .with_default_directive(LevelFilter::DEBUG.into())
        .parse("")
        .unwrap();
    tracing_subscriber::registry()
        .with(filter)
        .with(trace_layer)
        .with(tracing_subscriber::fmt::Layer::new())
        .init();

    let state = AppState {
        sqs: Arc::new(Sqs::new().await),
        prop_repo: Arc::new(InMemoryRepo(PropertyInMemoryRepo {})),
    };

    let app = routes::init_router(state.clone());

    let port = get_env("SERVER_PORT")?;
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    info!("🚀 Server started successfully");
    debug!("{:<12} - http://{:?}\n", "LISTENING", listener.local_addr()?);
    let message_queue_url = get_env("HOME_LOAN_MESSAGE_QUEUE_URL").expect("We need message queue");
    // Spawn our home loan queue, ideally this would be extracted to different crate/package, and I would have a system for multiple queues.
    tokio::spawn(async move {
        state.sqs.receive_message_from_queue(&message_queue_url, |(queue_message_handle, message_data): (String, MessageData<LoanRequestUuid>)| {
            // In the perfect world this would not need to be cloned, and I could just pass it as reference but...
            // Tokio spawn only provides 'static lifetimes meaning that in order to use an item as a reference it must be either owned(cloning does this) or have a static lifetime its self.
            // There is some performance loss with this, but I keep it at a minimal by using arc
            let app_state = state.clone();
            let message_queue_url = message_queue_url.clone();
            async move {
                debug!("Receive message from queue");
                let message_uuid = message_data.data.id;
                match app_state.prop_repo.send_status(message_uuid, true).await {
                    Ok(_) => {
                        app_state.sqs.delete_message_from_queue(&message_queue_url, queue_message_handle).await.unwrap();
                    }
                    Err(e) => {
                        error!("Error sending request to prop repo: {}",e);
                    }
                };
            }
        }).await.unwrap();
    });
    // Spawn our watcher.
    tokio::spawn(watcher_task);
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

#[derive(Clone)]
struct AppState {
    // Was actual setting this up as be dynamic but the async runtime determines that this will create complex object,
    // If in the future this gets sorted make this Arc<dyn QueueTrait>,
    // The same for our repos have hacked it a bit with enum, so we do have an option of DI.
    // A bit of a pity axum does support complex types, but you need to give it type generics
    // The issue with this is we will have super AppState where we pass all the Generics, so we would have AppState<T1,T2,T3,T4,T5> where T1: SomeTrait, T2: SomeTrait2 ... everywhere AppState is used.
    pub sqs: Arc<Sqs>,
    pub prop_repo: Arc<PropertySalesRepoEnum>,

    // user_repo: Arc<dyn >,
}