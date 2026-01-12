use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct MarketSnapshot {
    pub symbol: String,
    pub price: f64,
    pub timestamp: SystemTime,
}

impl MarketSnapshot {
    pub fn new(symbol: impl Into<String>, price: f64) -> Self {
        Self {
            symbol: symbol.into(),
            price,
            timestamp: SystemTime::now(),
        }
    }
}
