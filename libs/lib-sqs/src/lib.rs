/*#[derive(Debug)]
struct SQSMessage {
	body: String,
}


async fn send(client: &Client, queue_url: &String, message: &SQSMessage) -> Result<(), Error> {
	println!("Sending message to queue with URL: {}", queue_url);

	let rsp = client
		.send_message()
		.queue_url(queue_url)
		.message_body(&message.body)
		// If the queue is FIFO, you need to set .message_deduplication_id
		// and message_group_id or configure the queue for ContentBasedDeduplication.
		.send()
		.await?;

	println!("Send message to the queue: {:#?}", rsp);

	Ok(())
}*/