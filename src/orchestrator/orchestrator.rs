use crate::cli::ControlState;
use crate::types::*;
use super::traits::*;

use std::time::SystemTime;

pub struct Orchestrator<M, S, R, E, T>
where
    M: MarketDataProvider,
    S: Strategy,
    R: RiskEngine,
    E: ExecutionEngine,
    T: Storage,
{
    pub market: M,
    pub strategy: S,
    pub risk: R,
    pub executor: E,
    pub storage: T,
}

impl<M, S, R, E, T> Orchestrator<M, S, R, E, T>
where
    M: MarketDataProvider,
    S: Strategy,
    R: RiskEngine,
    E: ExecutionEngine,
    T: Storage,
{
    pub fn tick(
        &self,
        symbol: &str,
        trace_id: TraceId,
        control: &ControlState,
        runtime: &mut RuntimeState,
    ) -> Result<(), SystemError> {

        let result = (|| {
            // pre check

            if control.killed {
                return Err(SystemError::fatal("System is killed"));
            }

            if control.execution_lock.is_locked() {
                return Err(SystemError::policy("Execution is locked"));
            }

            if !control.lifecycle.is_running() {
                return Err(SystemError::policy("System not running"));
            }

            // Snapshot

            let snapshot = self.market
                .snapshot(symbol)
                .map_err(SystemError::transient)?;

            self.storage.save_audit(AuditEvent::MarketSnapshot {
                trace_id,
                symbol: symbol.to_string(),
                timestamp: SystemTime::now(),
            });

            // Strat Decision

            let decision = self.strategy.decide(&snapshot);
            self.storage.save_decision(&decision);

            self.storage.save_audit(AuditEvent::StrategyDecision {
                trace_id,
                action: format!("{:?}", decision.action),
                confidence: decision.confidence,
                reason: decision.reason.clone(),
                timestamp: SystemTime::now(),
            });

            if decision.action == DecisionAction::Hold {
                return Ok(());
            }

            // Check Risk

            let order = self.risk.approve(decision)?;

            self.storage.save_audit(AuditEvent::RiskApproved {
                trace_id,
                order_id: order.client_order_id.clone(),
                timestamp: SystemTime::now(),
            });

            // Execute
            
            let report = self.executor.execute(order)?;
            self.storage.save_execution(&report);

            self.storage.save_audit(AuditEvent::Execution {
                trace_id,
                exchange_order_id: report.exchange_order_id.clone(),
                status: format!("{:?}", report.status),
                timestamp: SystemTime::now(),
            });

            Ok(())
        })();
        
        // Failure Audit

        if let Err(ref err) = result {
            self.storage.save_audit(AuditEvent::Failure {
                trace_id,
                error: err.to_string(),
                timestamp: SystemTime::now(),
            });
        }

        result
    }
}
