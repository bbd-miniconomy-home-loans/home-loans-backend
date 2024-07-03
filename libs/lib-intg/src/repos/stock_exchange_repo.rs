use std::sync::Arc;
use async_trait::async_trait;
use reqwest::Client;


const STOCK_EXCHANGE_REPO_URL: &str = "https://api.stock_exchange.projects.bbdgrad.com";

pub enum StockExchangeRepoEnum {
    StockExchangeInMemoryRepoE(StockExchangeRepoInMemoryRepo),
    StockExchangeRepoE(StockExchangeRepo),
}

#[async_trait]
impl StockExchangeRepoTrait for StockExchangeRepoEnum {
    async fn sell_shares(&self) -> bool {
        match self {
            StockExchangeRepoEnum::StockExchangeInMemoryRepoE(r) => r.sell_shares().await,
            StockExchangeRepoEnum::StockExchangeRepoE(r) => r.sell_shares().await
        }
    }
}

#[async_trait]
pub trait StockExchangeRepoTrait: Send + Sync {
    async fn sell_shares(&self) -> bool;
}

pub struct StockExchangeRepoInMemoryRepo {}

#[async_trait]
impl StockExchangeRepoTrait for StockExchangeRepoInMemoryRepo {
    async fn sell_shares(&self) -> bool {
        true
    }
}

pub struct StockExchangeRepo {
    pub client: Arc<Client>,
}
#[async_trait]
impl StockExchangeRepoTrait for StockExchangeRepo {
    async fn sell_shares(&self) -> bool {
        let result = self.client.post(STOCK_EXCHANGE_REPO_URL).send().await;
        return result.is_err();
    }
}
