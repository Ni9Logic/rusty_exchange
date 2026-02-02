use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "A high-performance currency converter")]
pub struct ExchangeArgs {
    /// The amount of currency to convert
    pub amount: f64,

    /// The source currency code (e.g., USD)
    pub from: String,

    /// The target currency code (e.g., EUR)
    pub to: String,
}