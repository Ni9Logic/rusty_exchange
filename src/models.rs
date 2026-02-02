use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ExchangeRates {
    pub base_code: String,
    pub conversion_rates: HashMap<String, f64>,
}


