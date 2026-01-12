use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq)]
pub enum DecisionAction {
    Buy,
    Sell,
    Hold,
}

#[derive(Debug, Clone)]
pub struct StrategyDecision {
    pub symbol: String,
    pub action: DecisionAction,
    pub confidence: f64, // 0.0 → 1.0
    pub reason: String,
    pub timestamp: SystemTime,
}

impl StrategyDecision {
    pub fn hold(symbol: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            action: DecisionAction::Hold,
            confidence: 0.0,
            reason: reason.into(),
            timestamp: SystemTime::now(),
        }
    }
}
