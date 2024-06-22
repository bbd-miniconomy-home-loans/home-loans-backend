/*mod routing;

use std::error::Error;

use amqprs::{BasicProperties, DELIVERY_MODE_PERSISTENT};
use amqprs::callbacks::{DefaultChannelCallback, DefaultConnectionCallback};
use amqprs::channel::{BasicAckArguments, BasicConsumeArguments, BasicPublishArguments, QueueDeclareArguments};
use amqprs::connection::{Connection, OpenConnectionArguments};
use serde::{Deserialize, Serialize};
use tokio::sync::Notify;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	dotenvy::dotenv().ok();
	tracing_subscriber::fmt()
		.with_env_filter(EnvFilter::from_default_env())
		.init();
	
	// TODO: with axum(if we dont go lambda) will will most probably have a state that will have these vars.
	
	receive_message().await?;
	Ok(())
}

async fn send_message<T>(data: T) -> Result<(), Box<dyn Error>>
	where T: Serialize {
	let conn = Connection::open(&OpenConnectionArguments::new(
		"localhost",
		5672,
		"user",
		"password",
	))
		.await.unwrap();
	conn.register_callback(DefaultConnectionCallback).await.unwrap();

	let ch = conn.open_channel(None).await.unwrap();
	ch.register_callback(DefaultChannelCallback).await.unwrap();

	let q_args = QueueDeclareArguments::default()
		.queue(String::from("hello"))
		.durable(true)
		.finish();
	let (queue_name, _, _) = ch.queue_declare(q_args).await.unwrap().unwrap();

	let result = serde_json::to_string(&data).unwrap();
	let payload = result.into_bytes();
	let publish_args = BasicPublishArguments::new("", &queue_name);
// publish messages as persistent
	let props = BasicProperties::default().with_delivery_mode(DELIVERY_MODE_PERSISTENT).finish();
	ch.basic_publish(props, payload, publish_args).await.unwrap();

	println!(" [x] Sent \"Hello World!\"");

// in real applications connections are meant to be long lived
	conn.close().await.unwrap();

	Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct RabbitMessageTester1 {
	name: String,
	message: String,
}

#[tracing::instrument(skip_all)]
async fn receive_message() -> Result<(), Box<dyn Error>> {
	let conn = Connection::open(&OpenConnectionArguments::new(
		"localhost",
		5672,
		"user",
		"password",
	))
		.await.unwrap();
	conn.register_callback(DefaultConnectionCallback).await.unwrap();

	let ch = conn.open_channel(None).await.unwrap();
	ch.register_callback(DefaultChannelCallback).await.unwrap();

	let q_args = QueueDeclareArguments::default()
		.queue(String::from("hello"))
		.durable(true)
		.finish();
	let (queue_name, _, _) = ch.queue_declare(q_args).await.unwrap().unwrap();
	let consumer_args = BasicConsumeArguments::new(&queue_name, "receive.rs");
	let (_ctag, mut rx) = ch.basic_consume_rx(consumer_args).await.unwrap();

	
	
	tokio::spawn(async move {
		tracing::info!("Starting the consumer loop...");
		while let Some(msg) = rx.recv().await {
			let Some(payload) = msg.content else {
				tracing::error!("Could not find a payload skipping :(");
				continue;
			};
			let Some(deliver) = msg.deliver else {
				tracing::error!("Could not find a deliver skipping :(");
				continue;
			};
			let message: RabbitMessageTester1 = match serde_json::from_slice(payload.as_slice()) {
				Ok(res) => res,
				Err(e) => {
					// if there is a deserialization error, print an error
					// and go to the next loop iteration
					tracing::error!("Deserialization error: {e}");
					continue;
				}
			};
			tracing::info!("Got payload: {message:?}");
			// ch.tx_commit()
			ch.basic_ack(BasicAckArguments::new(deliver.delivery_tag(), false)).await.unwrap();
		};
	});

	println!(" [*] Waiting for messages. To exit press CTRL+C");

	let guard = Notify::new();
	guard.notified().await;


	Ok(())
}


#[cfg(test)]
mod tests {
	use crate::{RabbitMessageTester1, send_message};

	#[tokio::test]
	async fn test_say_hello() {
		send_message(RabbitMessageTester1 {
			name: "wow this is cool ".to_string(),
			message: "some message data".to_string(),
		}).await.unwrap();
	}
}

*/