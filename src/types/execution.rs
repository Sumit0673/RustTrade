use std::time::SystemTime;

#[derive(Debug, Clone)]
pub enum ExecutionStatus {
    Filled,
    Rejected,
    Partial,
}

#[derive(Debug, Clone)]
pub struct ExecutionReport {
    pub order_id: String,
    pub symbol: String,
    pub executed_qty: f64,
    pub avg_price: Option<f64>,
    pub status: ExecutionStatus,
    pub message: String,
    pub timestamp: SystemTime,
}
