use crate::models::ExchangeRates;
use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait ExchangeProvider {
    async fn fetch_rates(&self, base: &str) -> Result<ExchangeRates, String>;
}

pub struct ExchangeRateApi {
    api_key: String,
    client: Client,
}

impl ExchangeRateApi {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl ExchangeProvider for ExchangeRateApi {
    async fn fetch_rates(&self, base: &str) -> Result<ExchangeRates, String> {
        let url = format!(
            "https://v6.exchangerate-api.com/v6/{}/latest/{}",
            self.api_key, base
        );

        let response = self.client
            .get(url)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        response
            .json::<ExchangeRates>()
            .await
            .map_err(|e| format!("Parsing error: {}", e))

    }
}