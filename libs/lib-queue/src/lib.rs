use std::fmt::Debug;
use std::future::Future;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use error::Result;

mod error;
pub mod sqs;
mod cdc;

#[async_trait]
pub trait QueueTrait: Send + Sync {
	async fn send_message_to_queue<T>(&self, queue_url: &String, data: MessageData<T>) -> Result<String> where T: Serialize + DeserializeOwned + Send + Debug;
	async fn delete_message_from_queue(&self, queue_url: &String, queue_item_id: String) -> Result<()>;
	async fn receive_message_from_queue<T, F, Fut>(&self, queue_url: &String, receive_func: F) -> Result<()>
		where T: Serialize + DeserializeOwned + Send,
		      F: Fn((String, MessageData<T>)) -> Fut + Send,
		      Fut: Future<>;
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
	ADD,
	DELETE,
	RECEIVE,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageData<T>
{
	pub message_type: MessageType,
	pub data: T,
}

