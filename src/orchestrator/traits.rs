use crate::types::{MarketSnapshot, StrategyDecision};
use crate::types::{StrategyDecision, Order};
use crate::types::{ExecutionReport, StrategyDecision};
use crate::types::AuditEvent;

pub trait MarketDataProvider {
    fn snapshot(&self, symbol: &str) -> Result<MarketSnapshot, String>;
}

pub trait Strategy {
    fn decide(&self, snapshot: &MarketSnapshot) -> StrategyDecision;
}

pub trait RiskEngine {
    fn approve(&self, decision: StrategyDecision) -> Result<Order, String>;
}

pub trait ExecutionEngine {
    fn execute(&self, order: Order) -> Result<ExecutionReport, String>;
}

pub trait Storage {
    fn save_decision(&self, decision: &StrategyDecision);
    fn save_execution(&self, report: &ExecutionReport);
    fn save_audit(&self, event: AuditEvent);
}