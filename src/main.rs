mod models;
mod provider;
mod converter;
mod args;
mod cache;

use clap::Parser;
use args::ExchangeArgs;
use provider::{ExchangeRateApi, CachedProvider};
use converter::CurrencyConverter;
use dotenvy::dotenv;
use std::env;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    dotenv().ok();
    
    // 1. Parse CLI arguments
    let args = ExchangeArgs::parse();

    // 2. Setup (Injection)
    let api_key = env::var("EXCHANGE_API_KEY")
        .expect("EXCHANGE_API_KEY must be set");
    
    let api = ExchangeRateApi::new(api_key);
    let cached_provider = CachedProvider::new(api);
    let converter = CurrencyConverter::new(cached_provider);

    // 3. Execute using user input
    match converter.convert(args.amount, &args.from, &args.to).await {
        Ok(result) => {
            println!("{:.2} {} = {:.2} {}", args.amount, args.from, result, args.to);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    eprintln!("Execution time: {:?}", start.elapsed());
    Ok(())
}