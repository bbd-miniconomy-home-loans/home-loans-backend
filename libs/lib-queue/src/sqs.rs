use std::env;

use async_trait::async_trait;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sqs::Client;
use aws_sdk_sqs::config::{BehaviorVersion, Region};
use serde::Serialize;

use crate::{MessageData, QueueTrait};
use crate::error::Error;

pub struct Sqs {
	aws_client: Client,
}

impl Sqs
{
	pub async fn new() -> Sqs {
		let provider = RegionProviderChain::first_try(env::var("REGION")
			.ok().map(Region::new))
			.or_else(Region::new("eu-west-1"));

		let config = aws_config::defaults(BehaviorVersion::latest())
			.region(provider)
			.load().await;

		Sqs {
			aws_client: Client::new(&config),
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
