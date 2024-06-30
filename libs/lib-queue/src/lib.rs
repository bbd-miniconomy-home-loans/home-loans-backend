use serde::Serialize;

use error::Result;

mod error;
mod sqs;
mod cdc;

pub trait QueueTrait {
	async fn send_message_to_queue<T>(&self, queue_url: &String, data: MessageData<T>) -> Result<String> where T: Serialize;
	async fn delete_message_from_queue(&self, queue_url: &String, queue_item_id: String) -> Result<String>;
	async fn receive_message_from_queue<T>(&self, queue_url: &String) -> Result<MessageData<T>>;
}

enum MessageType {
	ADD,
	DELETE,
	RECEIVE,
}

pub struct MessageData<T>
	where T: Serialize
{
	pub message_type: MessageType,
	pub data: T,
}
