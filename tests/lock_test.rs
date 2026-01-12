use RustTrade::cli::*;
use std::time::SystemTime;

fn base_state() -> ControlState {
    ControlState {
        lifecycle: LifecycleState::Stopped,
        mode: Mode::Info,
        execution_lock: ExecutionLock::Unlocked,
        killed: false,
        last_transition: SystemTime::now(),
        last_decisions: Vec::new(),
    }
}

#[test]
fn lock_ok() {
    let state = base_state();
    let cmd = Command::Lock;
    let new_state = apply_command(state, cmd).unwrap();
    assert!(matches!(new_state.execution_lock, ExecutionLock::Locked));
}

#[test]
fn unlock_ok() {
    let mut state = base_state();
    state.execution_lock = ExecutionLock::Locked;
    let cmd = Command::Unlock;
    let new_state = apply_command(state, cmd).unwrap();
    assert!(matches!(new_state.execution_lock, ExecutionLock::Unlocked));
}

#[test]
fn start_locked_fails() {
    let mut state = base_state();
    state.execution_lock = ExecutionLock::Locked;
    let cmd = Command::Start { mode: Mode::Info };
    let err = apply_command(state, cmd).unwrap_err();
    assert!(err.contains("Locked"));
}

#[test]
    fn status_displays_without_panic() {
        let state = ControlState {
            lifecycle: LifecycleState::Running,
            mode: Mode::DryRun,
            execution_lock: ExecutionLock::Unlocked,
            killed: false,
            last_transition: std::time::SystemTime::now(),
            last_decisions: Vec::new(),
        };

        display_status(&state);
    }
