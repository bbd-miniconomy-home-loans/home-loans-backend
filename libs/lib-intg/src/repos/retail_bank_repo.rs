use std::convert::identity;
use std::fs;
use std::sync::Arc;
use async_trait::async_trait;
use reqwest::{Body, Certificate, Client, ClientBuilder, Identity, RequestBuilder};
use serde_with::serde_as;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use uuid::Uuid;
use log::trace;

use crate::repos::error;

pub enum RetailBankRepoEnum {
    RetailInMemoryRepoE(RetailBankInMemoryRepo),
    RetailSalesRepoE(RetailBankSalesRepo),
}

#[async_trait]
impl RetailBankRepoTrait for RetailBankRepoEnum {
    async fn create_debit_order(&self, home_loan_id: Uuid, approved: bool) -> error::Result<String> {
        match self {
            RetailBankRepoEnum::RetailInMemoryRepoE(repo) => repo.create_debit_order(home_loan_id, approved).await,
            RetailBankRepoEnum::RetailSalesRepoE(repo) => repo.create_debit_order(home_loan_id, approved).await,
        }
    }
}


#[async_trait]
pub trait RetailBankRepoTrait: Send + Sync {
    async fn create_debit_order(&self, home_loan_id: Uuid, approved: bool) -> error::Result<String>;
}

pub struct RetailBankInMemoryRepo {}

#[async_trait]
impl RetailBankRepoTrait for RetailBankInMemoryRepo {
    async fn create_debit_order(&self, home_loan_id: Uuid, approved: bool) -> error::Result<String> {
        Ok("Ok".to_string())
    }
}

pub struct RetailBankSalesRepo {
    pub client: Arc<Client>,
}

const RETAIL_BANK_REPO_URL: &str = "https://api.retailbank.projects.bbdgrad.com/";

#[async_trait]
impl RetailBankRepoTrait for RetailBankSalesRepo {
    async fn create_debit_order(&self, home_loan_id: Uuid, approved: bool) -> error::Result<String> {
        let data = serde_json::json!({
		    "loanId": home_loan_id,
		    "approved": approved,
		});
        let response_data = self.client
            .post(RETAIL_BANK_REPO_URL)
            .body(Body::from(data.to_string()))
            .send().await?
            .text().await?;
        trace!("We have got data from {}" ,response_data);
        Ok(response_data)
    }
}