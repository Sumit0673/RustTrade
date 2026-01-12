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
fn start_from_stopped_ok() {
    let state = base_state();
    let cmd = Command::Start { mode: Mode::DryRun };
    let new_state = apply_command(state, cmd).unwrap();
    assert!(matches!(new_state.lifecycle, LifecycleState::Running));
}

#[test]
fn start_from_running_err() {
    let mut state = base_state();
    state.lifecycle = LifecycleState::Running;
    let cmd = Command::Start { mode: Mode::Info };
    let err = apply_command(state, cmd).unwrap_err();
    assert!(err.contains("already running"));
}

#[test]
fn pause_from_running_ok() {
    let mut state = base_state();
    state.lifecycle = LifecycleState::Running;
    let cmd = Command::Pause;
    let new_state = apply_command(state, cmd).unwrap();
    assert!(matches!(new_state.lifecycle, LifecycleState::Paused));
}

#[test]
fn resume_from_paused_ok() {
    let mut state = base_state();
    state.lifecycle = LifecycleState::Paused;
    let cmd = Command::Resume;
    let new_state = apply_command(state, cmd).unwrap();
    assert!(matches!(new_state.lifecycle, LifecycleState::Running));
}
