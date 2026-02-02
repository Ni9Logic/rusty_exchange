use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExchangeRates {
    pub base_code: String,
    pub conversion_rates: HashMap<String, f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CachedRates {
    pub timestamp: SystemTime,
    pub rates: ExchangeRates, // Our existing struct
}
