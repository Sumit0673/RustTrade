use RustTrade::cli::*;
use std::time::SystemTime;

fn base_state() -> ControlState {
    ControlState {
        lifecycle: LifecycleState::Running,
        mode: Mode::Info,
        execution_lock: ExecutionLock::Unlocked,
        killed: false,
        last_transition: SystemTime::now(),
        last_decisions: Vec::new(),
    }
}

#[test]
fn kill_sets_safe_state() {
    let state = base_state();
    let cmd = Command::Kill;
    let new_state = apply_command(state, cmd).unwrap();
    assert!(new_state.killed);
    assert!(matches!(new_state.lifecycle, LifecycleState::Stopped));
    assert!(matches!(new_state.execution_lock, ExecutionLock::Locked));
}

#[test]
fn reset_without_kill_fails() {
    let state = base_state();
    let cmd = Command::Reset { confirm: "I UNDERSTAND".into() };
    let err = apply_command(state, cmd).unwrap_err();
    assert!(err.contains("not killed"));
}

#[test]
fn reset_with_kill_succeeds() {
    let state = base_state();
    let killed_state = apply_command(state, Command::Kill).unwrap();
    let cmd = Command::Reset { confirm: "I UNDERSTAND".into() };
    let new_state = apply_command(killed_state, cmd).unwrap();
    assert!(!new_state.killed);
    assert!(matches!(new_state.lifecycle, LifecycleState::Stopped));
    assert!(matches!(new_state.execution_lock, ExecutionLock::Unlocked));
}
