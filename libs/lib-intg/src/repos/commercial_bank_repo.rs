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
use crate::models::BankResponse;

use crate::repos::error;

pub enum CommercialBankRepoEnum {
    CommercialInMemoryRepoE(CommercialInMemoryRepo),
    CommercialRepoE(CommercialRepo),
}

#[async_trait]
impl CommercialRepoTrait for CommercialBankRepoEnum {
    async fn send_debit_order(&self, home_loan_id: Uuid, approved: bool) -> error::Result<String> {
        match self {
            CommercialBankRepoEnum::CommercialInMemoryRepoE(repo) => repo.send_debit_order(home_loan_id, approved).await,
            CommercialBankRepoEnum::CommercialRepoE(repo) => repo.send_debit_order(home_loan_id, approved).await,
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
    async fn send_debit_order(&self, home_loan_id: Uuid, approved: bool) -> error::Result<String>;
    async fn request_balance(&self) -> Result<BankResponse, Box<dyn std::error::Error>>;
}

pub struct CommercialInMemoryRepo {}

#[async_trait]
impl CommercialRepoTrait for CommercialInMemoryRepo {
    async fn send_debit_order(&self, home_loan_id: Uuid, approved: bool) -> error::Result<String> {
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

/*impl CommercialRepo {
    pub async fn new() -> CommercialRepo {
        let client_pem_file_loc = "C:\\Users\\bbdnet3301\\Downloads\\aDownloads\\hl1.crt";
        let client_pem_file_loc_key = "C:\\Users\\bbdnet3301\\Downloads\\aDownloads\\a.key";
        let cert = fs::read(client_pem_file_loc).unwrap();
        let key = fs::read(client_pem_file_loc_key).unwrap();
        let identity = reqwest::Identity::from_pkcs8_pem(&cert, &key).expect("TODO: panic message");

        let client = ClientBuilder::new()
            .gzip(true)
            .use_native_tls()
            .tls_built_in_root_certs(true)  // You may need to adjust this based on your setup
            .identity(identity)
            .https_only(true)
            .build()
            .expect("Failed to build reqwest client");
        CommercialRepo { client }
    }
}
*/
#[async_trait]
impl CommercialRepoTrait for CommercialRepo {
    async fn send_debit_order(&self, home_loan_id: Uuid, approved: bool) -> error::Result<String> {
        let response_data = self.client
            .post(COMMERCIAL_REPO_URL)
            // .body(Body::from(data.to_string()))
            .send().await?
            .text().await?;
        trace!("We have got data from {}" ,response_data);

        Ok(response_data)
    }

    async fn request_balance(&self) -> Result<BankResponse, Box<dyn std::error::Error>> {
        Ok(self.client.post(format!("{COMMERCIAL_REPO_URL}/account/balance")).send().await?.json().await?)
    }
}