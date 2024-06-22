use amqprs::BasicProperties;
use amqprs::channel::BasicPublishArguments;
use amqprs::connection::{Connection, OpenConnectionArguments};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::rabbit::config::rabbit_config;
use crate::rabbit::error::RabbitErrorInternal::MqOpenError;
use crate::rabbit::error::Result;

/*trait Queue {
	fn create_connection();
	fn send_data_to_queue();
	fn receive_data_from_queue();
}
*/
#[derive(Clone)]
pub struct RabbitMQ {
	connection: Connection,
}


#[derive(Serialize, Deserialize)]
pub struct MessageData<T>
{
	uuid: Uuid,
	content: T,
}

impl RabbitMQ {
	pub async fn new() -> Result<RabbitMQ> {
		Ok(RabbitMQ { connection: create_rabbit_connection().await? })
	}

	pub async fn send_message_to_queue<T>(&self, data: MessageData<T>) -> Result<()>
		where T: Serialize {
		let channel = self.connection.open_channel(None).await.unwrap();
		// What do these do?
		let queue_name = "amqprs.example";
		let exchange_name = "amq.topic";
		// create arguments for basic_publish
		let args = BasicPublishArguments::new(exchange_name, queue_name);

		let json_data = serde_json::to_string(&data).unwrap();
		let package = json_data.into_bytes();

		let properties = BasicProperties::default()
			.with_content_type("application/json")
			.finish();

		channel
			.basic_publish(properties, package, args)
			.await
			.unwrap();
		Ok(())
	}

	pub async fn receive_messages_from_queue<T>(&self) -> Result<()> {
		let channel = self.connection.open_channel(None).await.unwrap();
		// What do these do?
		let queue_name = "amqprs.example";
		let exchange_name = "amq.topic";
		// create arguments for basic_publish
		let args = BasicPublishArguments::new(exchange_name, queue_name);

		// Maybe take function that will be used when a message is processed.
		// Will most probably need to be static though?
		// Would actually like to have an async method that could handle getting data back

		/*	let properties = BasicProperties::default()
				.with_content_type("application/json")
				.finish();

			channel
				.basic_publish(properties, package, args)
				.await
				.unwrap();*/
		Ok(())
	}
}

pub async fn create_rabbit_connection() -> Result<Connection> {
	let config = rabbit_config();
	let args = OpenConnectionArguments::new(
		config.HOST.as_str(),
		config.PORT,
		config.USERNAME.as_str(),
		config.PASSWORD.as_str(),
	);
	Connection::open(&args).await.map_err(MqOpenError)
}


