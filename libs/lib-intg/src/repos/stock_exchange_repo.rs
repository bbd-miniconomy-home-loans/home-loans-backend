use std::{fmt::format, sync::Arc};
use async_trait::async_trait;
use reqwest::{Body, Client, ClientBuilder};
use serde_json::json;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use serde::{Deserialize, Serialize};

use crate::repos::error;

const STOCK_EXCHANGE_REPO_URL: &str = "https://api.mese.projects.bbdgrad.com/stocks/sell";

pub enum StockExchangeRepoEnum {
    StockExchangeInMemoryRepoE(StockExchangeRepoInMemoryRepo),
    StockExchangeRepoE(StockExchangeRepo),
}

#[async_trait]
impl StockExchangeRepoTrait for StockExchangeRepoEnum {
    async fn sell_shares(&self,  seller_id: String, company_id: String, quantity: i64) -> error::Result<String> {
        match self {
            StockExchangeRepoEnum::StockExchangeInMemoryRepoE(r) => r.sell_shares(seller_id, company_id, quantity).await,
            StockExchangeRepoEnum::StockExchangeRepoE(r) => r.sell_shares(seller_id, company_id, quantity).await
        }
    }
}

#[async_trait]
pub trait StockExchangeRepoTrait: Send + Sync {
    async fn sell_shares(&self,  seller_id: String, company_id: String, quantity: i64) -> error::Result<String>;
}

pub struct StockExchangeRepoInMemoryRepo {}

#[async_trait]
impl StockExchangeRepoTrait for StockExchangeRepoInMemoryRepo {
    async fn sell_shares(&self,  seller_id: String, company_id: String, quantity: i64) -> error::Result<String> {

        Ok(format!("Ok"))
    }
}

pub struct StockExchangeRepo {
    pub client: Arc<Client>,
}
#[async_trait]
impl StockExchangeRepoTrait for StockExchangeRepo {
    async fn sell_shares(&self, seller_id: String, company_id: String, quantity: i64) -> error::Result<String>  {

        let data = json!({
                "sellerId": format!("{}-{}", seller_id, generate_random_id()),
                "companyId": company_id,
                "quantity": quantity
            });

        let response = self.client
        .post(STOCK_EXCHANGE_REPO_URL)
        .body(Body::from(data.to_string()))
        .send().await?
        .text().await?;

        println!("{:?}", response);
        println!("{:?}", data.to_string());

        Ok(response)
    }
}

#[derive(Deserialize, Serialize)]
pub struct PersonaResult {
	pub success: bool,
	pub data: Option<String>,
    pub errors: Option<String>,
    pub flex: String
}

fn generate_random_id() -> String {
    let mut rng = thread_rng();
    let id: String = (&mut rng)
        .sample_iter(Alphanumeric)
        .take(16)  // Adjust the length as needed
        .map(char::from)
        .collect();
    id
}