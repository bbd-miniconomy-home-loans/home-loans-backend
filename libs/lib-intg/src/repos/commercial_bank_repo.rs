use std::fmt::format;
use std::fs;
use std::sync::Arc;
use async_trait::async_trait;
use reqwest::{Body, Client, ClientBuilder};
use serde_with::serde_as;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use uuid::Uuid;
use log::trace;
use serde_json::json;
use crate::models::BankResponse;

use crate::repos::error;

pub enum CommercialBankRepoEnum {
    CommercialInMemoryRepoE(CommercialInMemoryRepo),
    CommercialRepoE(CommercialRepo),
}

#[async_trait]
impl CommercialRepoTrait for CommercialBankRepoEnum {
    async fn send_transaction(&self, cents: i64, candidate_id: String, our_account: String) -> error::Result<String> {
        match self {
            CommercialBankRepoEnum::CommercialInMemoryRepoE(repo) => repo.send_transaction(cents, candidate_id, our_account).await,
            CommercialBankRepoEnum::CommercialRepoE(repo) => repo.send_transaction(cents, candidate_id, our_account).await,
        }
    }

    async fn request_balance(&self) -> Result<BankResponse, Box<dyn std::error::Error>> {
        match self {
            CommercialBankRepoEnum::CommercialInMemoryRepoE(repo) => repo.request_balance().await,
            CommercialBankRepoEnum::CommercialRepoE(repo) => repo.request_balance().await,
        }
    }
}


#[async_trait]
pub trait CommercialRepoTrait: Send + Sync {
    async fn send_transaction(&self, cents: i64, candidate_id: String, our_account: String) -> error::Result<String>;
    async fn request_balance(&self) -> Result<BankResponse, Box<dyn std::error::Error>>;
}

pub struct CommercialInMemoryRepo {}

#[async_trait]
impl CommercialRepoTrait for CommercialInMemoryRepo {
    async fn send_transaction(&self, home_loan_id: i64, approved: String, string: String) -> error::Result<String> {
        Ok("Ok".to_string())
    }

    async fn request_balance(&self) -> Result<BankResponse, Box<dyn std::error::Error>> {
        Ok(BankResponse {
            status: 0,
            data: Default::default(),
            message: "".to_string(),
        })
    }
}

pub struct CommercialRepo {
    pub client: Arc<Client>,
}

const COMMERCIAL_REPO_URL: &str = "https://api.retailbank.projects.bbdgrad.com";

#[async_trait]
impl CommercialRepoTrait for CommercialRepo {
    async fn send_transaction(&self, cents: i64, candidate_id: String, our_account: String) -> error::Result<String> {
        let data = json!({
                         "transactions": [
                             {
                               "debitAccountName": our_account,
                               "creditAccountName": candidate_id,
                               "amount": cents,
                               "debitRef": format!("home_loans-{}", candidate_id),
                               "creditRef": format!("home_loans-{}", candidate_id)
                             }
                           ]
                     });

        let response_data = self.client
            .post(COMMERCIAL_REPO_URL)
            .body(Body::from(data.to_string()))
            .send().await?
            .text().await?;
        trace!("We have got data from {}" ,response_data);

        Ok(response_data)
    }

    async fn request_balance(&self) -> Result<BankResponse, Box<dyn std::error::Error>> {
        Ok(self.client.post(format!("{COMMERCIAL_REPO_URL}/account/balance")).send().await?.json().await?)
    }
}