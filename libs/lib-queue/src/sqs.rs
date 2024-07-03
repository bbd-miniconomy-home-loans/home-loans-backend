use std::env;
use std::fmt::Debug;
use std::future::Future;

use async_trait::async_trait;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sqs::Client;
use aws_sdk_sqs::config::{BehaviorVersion, Region};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use tracing::{debug, error};

use crate::{MessageData, QueueTrait};
use crate::error::Error;
use crate::error::Error::SmithyTypesAWSDoesNotWantUsToKnowAboutCusTheyCheat;

pub struct Sqs {
	aws_client: Client,
}

impl<'a> Sqs
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
	async fn send_message_to_queue<T>(&self, queue_url: &String, data: MessageData<T>) -> crate::error::Result<String> where T: Serialize + DeserializeOwned + Send,
	{
		let send_msg_output = self.aws_client
			.send_message()
			.queue_url(queue_url)
			.message_body(serde_json::to_string(&data)?)
			.send().await
			.map_err(|e| SmithyTypesAWSDoesNotWantUsToKnowAboutCusTheyCheat)?;
		let message_id = send_msg_output.message_id.ok_or(Error::NoMessageId)?;
		Ok(message_id)
	}

	async fn delete_message_from_queue(&self, queue_url: &String, queue_item_id: String) -> crate::error::Result<()> {
		self.aws_client
			.delete_message()
			.queue_url(queue_url)
			.receipt_handle(queue_item_id)
			.send().await
			.map_err(|e| SmithyTypesAWSDoesNotWantUsToKnowAboutCusTheyCheat)?;
		Ok(())
	}
	async fn receive_message_from_queue<T, F, Fut>(&self, queue_url: &String, receive_func: F) -> crate::error::Result<()>
		where
			T: Serialize + DeserializeOwned + Send,
			F: Fn((String, MessageData<T>)) -> Fut + Send,
			Fut: Future
	{
		debug!("Receive message from queue start");
		while let Some(messages) = self.aws_client
			.receive_message()
			.wait_time_seconds(20)
			.queue_url(queue_url)
			.send().await
			.map_err(|e| SmithyTypesAWSDoesNotWantUsToKnowAboutCusTheyCheat)?
			.messages {
			debug!("Receive message from queue messages collect");
			messages.into_iter().filter_map(|message| {
				let packaged_message = message.body()
					.map(|package| serde_json::from_str::<MessageData<T>>(package))
					.map(|x| x.map_err(|err| error!("Failed to deserialize message: {}", err)).ok())
					.flatten();
				let receipt_handle = message.receipt_handle();
				match (packaged_message, receipt_handle) {
					(Some(pm), Some(rh)) => Some((rh.to_string(), pm)),
					_ => {
						error!("Could not find a payload or receipt handle, skipping :(");
						None
					}
				}
			}).for_each(|(receipt_handle, packaged_message)| { receive_func((receipt_handle, packaged_message)); });
		}
		Ok(())
	}
}
