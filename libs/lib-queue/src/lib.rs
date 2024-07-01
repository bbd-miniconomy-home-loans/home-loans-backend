use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use error::Result;

mod error;
pub mod sqs;
mod cdc;

#[async_trait]
pub trait QueueTrait: Send + Sync {
	async fn send_message_to_queue<T>(&self, queue_url: &String, data: MessageData<T>) -> Result<String> where T: Serialize + Send;
	async fn delete_message_from_queue(&self, queue_url: &String, queue_item_id: String) -> Result<()>;
	async fn receive_message_from_queue(&self, queue_url: &String) -> Result<()>;
}

#[derive(Serialize, Deserialize)]
pub enum MessageType {
	ADD,
	DELETE,
	RECEIVE,
}

#[derive(Serialize, Deserialize)]
pub struct MessageData<T>
	where T: Serialize + Send
{
	pub message_type: MessageType,
	pub data: T,
}
