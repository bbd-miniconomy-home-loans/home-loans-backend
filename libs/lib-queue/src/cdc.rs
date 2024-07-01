/*
TODO: once we sort bring this back 
Change Data Capture
#[tracing::instrument]
pub async fn create_message(message_id: String, message_name: String, message_data: String, db: &sqlx::PgPool) {
	let _ = sqlx::query!(
        "INSERT INTO MESSAGES
                           (message_id, name, message)
                            VALUES
                            ($1, $2, $3)
                            ON CONFLICT (message_id) DO NOTHING",
       message_id,
        message_name,
        message_data
    )
		.execute(db)
		.await
		.inspect_err(|e| tracing::error!("Error while inserting message: {e}"));
}

#[tracing::instrument]
pub async fn update_message(message_id: String, message_name: String, message_data: String, db: &sqlx::PgPool) {
	let _ = sqlx::query!(
        "UPDATE MESSAGES
                            SET
                            name = $1,
                            message = $2
                            where message_id = $3",
       message_id,
        message_name,
        message_data
    )
		.execute(db)
		.await
		.inspect_err(|e| tracing::error!("Error while updating message: {e}"));
}

#[tracing::instrument]
pub async fn delete_message(message_id: String, db: &sqlx::PgPool) {
	let _ = sqlx::query!(
        "DELETE from messages where message_id = $1",
        message_id
    )
		.execute(db)
		.await
		.inspect_err(|e| tracing::error!("Error while deleting message: {e}"));
}
*/