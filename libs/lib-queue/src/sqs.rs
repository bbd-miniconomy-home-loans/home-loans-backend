use async_trait::async_trait;
use aws_sdk_sqs::Client;
use serde::Serialize;

use crate::{MessageData, QueueTrait};
use crate::error::Error;

pub struct Sqs {
	aws_client: Client,
}

impl Sqs
{
	pub fn new() -> Sqs {
		Sqs {
			aws_client: todo!(),
		}
	}
}

#[async_trait]
impl QueueTrait for Sqs {
	async fn send_message_to_queue<T>(&self, queue_url: &String, data: MessageData<T>) -> crate::error::Result<String> where T: Serialize + Send {
		let send_msg_output = self.aws_client
			.send_message()
			.queue_url(queue_url)
			.message_body(serde_json::to_string(&data)?)
			.send().await.unwrap();
		let message_id = send_msg_output.message_id.ok_or(Error::NoMessageId)?;

		Ok(message_id)
	}

	async fn delete_message_from_queue(&self, queue_url: &String, queue_item_id: String) -> crate::error::Result<()> {
		self.aws_client
			.delete_message()
			.queue_url(queue_url)
			.receipt_handle(queue_item_id)
			.send().await.unwrap();
		Ok(())
	}

	async fn receive_message_from_queue(&self, queue_url: &String) -> crate::error::Result<()> {
		todo!()
	}
}
