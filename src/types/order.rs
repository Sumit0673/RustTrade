use std::time::SystemTime;

#[derive(Debug, Clone)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub symbol: String,
    pub side: OrderSide,
    pub quantity: f64,
    pub price: Option<f64>,
    pub timestamp: SystemTime,
}

impl Order {
    pub fn market(
        symbol: impl Into<String>,
        side: OrderSide,
        quantity: f64,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            side,
            quantity,
            price: None,
            timestamp: SystemTime::now(),
        }
    }
}
