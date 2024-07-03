use std::sync::Arc;
use async_trait::async_trait;
use reqwest::{Body, Client, ClientBuilder};
use serde_with::serde_as;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use uuid::Uuid;

use crate::repos::error;

pub enum PropertySalesRepoEnum {
    InMemoryRepo(PropertyInMemoryRepo),
    PropertySalesRepoE(PropertySalesRepo),
}

#[async_trait]
impl PropertySalesRepoTrait for PropertySalesRepoEnum {
    async fn send_status(&self, home_loan_id: Uuid, approved: bool) -> error::Result<String> {
        match self {
            PropertySalesRepoEnum::InMemoryRepo(repo) => repo.send_status(home_loan_id, approved).await,
            PropertySalesRepoEnum::PropertySalesRepoE(repo) => repo.send_status(home_loan_id, approved).await,
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
    pub client: Arc<Client>,
}

const PROPERTY_SALES_REPO_URL: &str = "https://api.sales.projects.bbdgrad.com";


#[async_trait]
impl PropertySalesRepoTrait for PropertySalesRepo {
    async fn send_status(&self, home_loan_id: Uuid, approved: bool) -> error::Result<String> {
        let data = serde_json::json!({
		    "loanId": home_loan_id,
		    "isApproved": approved,
		});
        let response_data = self.client
            .post(format!("{PROPERTY_SALES_REPO_URL}/api/loan/update"))
            .body(Body::from(data.to_string()))
            .send().await?
            .text().await?;
        Ok(response_data)
    }
}