use std::sync::Arc;
use async_trait::async_trait;
use reqwest::Client;


const TAX_REPO_URL: &str = "https://api.tax.projects.bbdgrad.com";

pub enum TaxRepoEnum {
    TaxInMemoryRepoE(TaxInMemoryRepo),
    TaxRepoR(TaxRepo),
}

#[async_trait]
impl TaxRepoTrait for TaxRepoEnum {
    async fn pay_tax(&self) -> bool {
        match self {
            TaxRepoEnum::TaxInMemoryRepoE(r) => r.pay_tax().await,
            TaxRepoEnum::TaxRepoR(r) => r.pay_tax().await
        }
    }
}

#[async_trait]
pub trait TaxRepoTrait: Send + Sync {
    async fn pay_tax(&self) -> bool;
}

pub struct TaxInMemoryRepo {}

#[async_trait]
impl TaxRepoTrait for TaxInMemoryRepo {
    async fn pay_tax(&self) -> bool {
        true
    }
}

pub struct TaxRepo {
    pub client: Arc<Client>,
}
#[async_trait]
impl TaxRepoTrait for TaxRepo {
    async fn pay_tax(&self) -> bool {
        let result = self.client.post(TAX_REPO_URL).send().await;
        return result.is_err();
    }
}
