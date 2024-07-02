use async_trait::async_trait;
use reqwest::{Body, Client, ClientBuilder};
use serde_with::serde_as;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use uuid::Uuid;

use crate::repos::error;

pub enum PropertySalesRepoEnum {
	InMemoryRepo(PropertyInMemoryRepo),
	PropertySalesRepo(PropertySalesRepo),
}
#[async_trait]

impl PropertySalesRepoTrait for PropertySalesRepoEnum {
	async fn send_status(&self, home_loan_id: Uuid, approved: bool) -> error::Result<String> {
		match self {
			PropertySalesRepoEnum::InMemoryRepo(repo) => repo.send_status(home_loan_id, approved).await,
			PropertySalesRepoEnum::PropertySalesRepo(repo) => repo.send_status(home_loan_id, approved).await,
		}
	}
}


#[async_trait]
pub trait PropertySalesRepoTrait: Send + Sync {
	async fn send_status(&self, home_loan_id: Uuid, approved: bool) -> error::Result<String>;
}

pub struct PropertyInMemoryRepo {}
#[async_trait]

impl PropertySalesRepoTrait for PropertyInMemoryRepo {
	async fn send_status(&self, home_loan_id: Uuid, approved: bool) -> error::Result<String> {
		Ok("Ok".to_string())
	}
}

pub struct PropertySalesRepo {
	client: Client,
}

const PROPERTY_SALES_REPO_URL: &str = "hello convenience!";

impl PropertySalesRepo {
	async fn new() -> PropertySalesRepo {
		let client_pem_file_loc = "ca/second_client.pem";
		let mut buf = Vec::new();
		File::open(client_pem_file_loc)
			.await
			.unwrap()
			.read_to_end(&mut buf)
			.await
			.unwrap();
		let cert = reqwest::Certificate::from_pem(&buf).unwrap();

		let client_pem_file_loc = "ca/second_client.pem";
		let mut buf = Vec::new();
		File::open(client_pem_file_loc)
			.await
			.unwrap()
			.read_to_end(&mut buf)
			.await
			.unwrap();
		let identity = reqwest::Identity::from_pem(&buf).unwrap();

		let client = ClientBuilder::new().gzip(true)
			.use_rustls_tls()
			.tls_built_in_root_certs(false)
			.add_root_certificate(cert)
			.identity(identity)
			.https_only(true).build().unwrap();

		PropertySalesRepo { client }
	}
}
#[async_trait]

impl PropertySalesRepoTrait for PropertySalesRepo {
	async fn send_status(&self, home_loan_id: Uuid, approved: bool) -> error::Result<String> {
		let data = serde_json::json!({
		    "loanId": home_loan_id,
		    "approved": approved,
		});
		let response_data = self.client
			.post(PROPERTY_SALES_REPO_URL)
			.body(Body::from(data.to_string()))
			.send().await?
			.text().await?;
		Ok(response_data)
	}
}