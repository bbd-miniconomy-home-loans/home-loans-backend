use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};

use reqwest::{Client, ClientBuilder};

pub trait TaxRepoTrait: Send + Sync {
	fn send_property_tax(&self, user_id: String, property_id: String, amount_of_loan: f64) -> bool;
}


#[derive(Debug, Clone, Default)]
pub struct InMemoryTaxRepo {
	map: Arc<Mutex<HashMap<String, String>>>,
}

impl TaxRepoTrait for InMemoryTaxRepo {
	fn send_property_tax(&self, user_id: String, property_id: String, amount_of_loan: f64) -> bool {
		todo!()
	}
}

#[derive(Debug, Clone)]
pub struct TaxRepo {
	client: Client,
}

impl TaxRepo {
	pub fn new() -> Result<TaxRepo, Box<dyn Error>> {
		let client = ClientBuilder::new().gzip(true).build()?;
		Ok(TaxRepo { client })
	}
}

impl TaxRepoTrait for TaxRepo {
	fn send_property_tax(&self, user_id: String, property_id: String, amount_of_loan: f64) -> bool {
		self.client.post("")
			// .body()
			.build().unwrap();
		return true;
	}
}

