use crate::models::ExchangeRates;
use async_trait::async_trait;
use reqwest::Client;
use crate::cache::FileCache;

#[async_trait(?Send)]
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

#[async_trait(?Send)]
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

pub struct CachedProvider<P: ExchangeProvider> {
    inner: P,
    cache: FileCache,
}

impl<P: ExchangeProvider> CachedProvider<P> {
    pub fn new(inner: P) -> Self {
        Self { inner, cache: FileCache }
    }
}

#[async_trait(?Send)]
impl<P: ExchangeProvider> ExchangeProvider for CachedProvider<P> {
    async fn fetch_rates(&self, base: &str) -> Result<ExchangeRates, String> {
        // 1. Try to get from cache
        if let Some(rates) = self.cache.get(base) {
            return Ok(rates);
        }

        // 2. If not in cache, use the inner provider (the real API)
        let fresh_rates = self.inner.fetch_rates(base).await?;

        // 3. Save to cache for next time
        self.cache.save(&fresh_rates);

        Ok(fresh_rates)
    }
}