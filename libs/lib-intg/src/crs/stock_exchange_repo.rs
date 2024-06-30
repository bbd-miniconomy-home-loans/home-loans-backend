pub trait StockExchangeRepoTrait: Send + Sync {
	async fn buy_shares(&self, user_id: String, property_id: String, amount_of_loan: f64) -> bool;
	async fn sell_shares(&self, user_id: String, property_id: String, amount_of_loan: f64) -> bool;
}


