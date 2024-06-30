use aws_sdk_sqs::Client;
use serde::Serialize;

use crate::{MessageData, QueueTrait};
use crate::error::Error;

struct Sqs {
	aws_client: Client,
}

impl QueueTrait for Sqs {
	async fn send_message_to_queue<T>(&self, queue_url: &String, data: MessageData<T>) -> crate::error::Result<String> where T: Serialize {
		let send_msg_output = self.aws_client
			.send_message()
			.queue_url(queue_url)
			.message_body(serde_json::to_string(&data)?)
			.send().await?;
		let message_id = send_msg_output.message_id.ok_or(Error::NoMessageId)?;

		Ok(message_id)
	}

	async fn delete_message_from_queue(&self, queue_url: &String, queue_item_id: String) -> crate::error::Result<()> {
		self.aws_client
			.delete_message()
			.queue_url(queue_url)
			.receipt_handle(queue_item_id)
			.send().await?;
		Ok(())
	}

	async fn receive_message_from_queue<T>(&self, queue_url: &String) -> crate::error::Result<MessageData<T>> {
		// Haha, lol we pick up the messages from lambdas
		todo!()
	}
}
