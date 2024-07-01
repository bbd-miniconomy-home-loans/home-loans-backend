use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

use crate::config::config;

pub(crate) mod error;
mod dbx;
mod config;

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> sqlx::Result<Db> {
	// Testing can break this process 
	let max_connections = 5;

	PgPoolOptions::new()
		.max_connections(max_connections)
		.connect(&config().DATABASE_URL)
		.await
}