use RustTrade::cli::*;
use std::time::SystemTime;

#[test]
fn explain_displays_decisions() {
    let mut state = ControlState {
        lifecycle: LifecycleState::Running,
        mode: Mode::DryRun,
        execution_lock: ExecutionLock::Unlocked,
        killed: false,
        last_transition: SystemTime::now(),
        last_decisions: vec![
            DecisionLog {
                symbol: "BTCUSDT".into(),
                action: "BUY".into(),
                confidence: 0.85,
                outcome: DecisionOutcome::Accepted,
                reason: "Trend confirmed".into(),
                timestamp: SystemTime::now(),
            },
            DecisionLog {
                symbol: "ETHUSDT".into(),
                action: "SELL".into(),
                confidence: 0.42,
                outcome: DecisionOutcome::Rejected,
                reason: "Confidence too low".into(),
                timestamp: SystemTime::now(),
            },
        ],
    };

    display_explain(&state); // should print colored output, visually check
}
