use std::time::SystemTime;
use crate::types::trace::TraceId;

#[derive(Debug, Clone)]
pub enum AuditEvent {
    MarketSnapshot {
        trace_id: TraceId,
        symbol: String,
        timestamp: SystemTime,
    },
    StrategyDecision {
        trace_id: TraceId,
        action: String,
        confidence: f64,
        reason: String,
        timestamp: SystemTime,
    },
    RiskApproved {
        trace_id: TraceId,
        order_id: String,
        timestamp: SystemTime,
    },
    Execution {
        trace_id: TraceId,
        exchange_order_id: String,
        status: String,
        timestamp: SystemTime,
    },
    Failure {
        trace_id: TraceId,
        error: String,
        timestamp: SystemTime,
    },
}
