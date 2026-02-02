use crate::provider::ExchangeProvider;

pub struct CurrencyConverter<P: ExchangeProvider> {
    provider: P,
}

impl<P: ExchangeProvider> CurrencyConverter<P> {
    // Constructor handles Dependency Injection
    pub fn new(provider: P) -> Self {
        Self { provider }
    }

    pub async fn convert(
        &self,
        amount: f64,
        from: &str,
        to: &str,
    ) -> Result<f64, String> {
        // 1. Fetch rates for the source currency
        let data = self.provider.fetch_rates(from).await?;

        // 2. Look up the target rate
        let rate = data.conversion_rates.get(to).ok_or_else(|| {
            format!("Target currency '{}' not found in exchange data.", to)
        })?;

        // 3. Perform the calculation
        Ok(amount * rate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ExchangeRates;
    use std::collections::HashMap;
    use async_trait::async_trait;

    // A "Mock" provider that doesn't use the internet
    struct MockProvider;

    #[async_trait]
    impl ExchangeProvider for MockProvider {
        async fn fetch_rates(&self, _base: &str) -> Result<ExchangeRates, String> {
            let mut rates = HashMap::new();
            rates.insert("EUR".to_string(), 0.9); // 1 USD = 0.9 EUR
            
            Ok(ExchangeRates {
                base_code: "USD".to_string(),
                conversion_rates: rates,
            })
        }
    }

    #[tokio::test]
    async fn test_conversion_logic() {
        // Arrange
        let mock = MockProvider;
        let converter = CurrencyConverter::new(mock);

        // Act
        let result = converter.convert(100.0, "USD", "EUR").await;

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 90.0);
    }
}