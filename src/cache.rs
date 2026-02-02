use crate::models::{ExchangeRates, CachedRates};
use std::fs;
use std::time::{SystemTime, Duration};

pub struct FileCache;

impl FileCache {
    const CACHE_FILE: &'static str = ".exchange_cache.json";
    const EXPIRATION: Duration = Duration::from_secs(3600); // 1 hour

    pub fn get(&self, base: &str) -> Option<ExchangeRates> {
        let content = fs::read_to_string(Self::CACHE_FILE).ok()?;
        let cached: CachedRates = serde_json::from_str(&content).ok()?;

        // Check if cache belongs to the same base and is not expired
        if cached.rates.base_code == base {
            if let Ok(elapsed) = cached.timestamp.elapsed() {
                if elapsed < Self::EXPIRATION {
                    println!("Cache hit! Using local data.");
                    return Some(cached.rates);
                }
            }
        }
        eprintln!("Cache expired or mismatched. Fetching fresh data...");
        None
    }

    pub fn save(&self, rates: &ExchangeRates) {
        let data = CachedRates {
            timestamp: SystemTime::now(),
            rates: rates.clone(), // You'll need to add #[derive(Clone)] to ExchangeRates
        };

        if let Ok(json) = serde_json::to_string(&data) {
            let _ = fs::write(Self::CACHE_FILE, json);
        }
    }
}