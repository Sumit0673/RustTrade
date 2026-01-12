use std::time::SystemTime;
use RustTrade::cli::*;


fn main() {
    // ---- Initial state (pretend this came from disk) ----
    let state = ControlState {
        lifecycle: LifecycleState::Stopped,
        mode: Mode::Info,
        execution_lock: ExecutionLock::Unlocked,
        killed: false,
        last_transition: SystemTime::now(),
        last_decisions: Vec::new(),
    };

    println!("Initial State:\n{:#?}\n", state);

    // ---- Simulate a command ----
    let command = Command::Start { mode: Mode::DryRun };

    match apply_command(state, command) {
        Ok(new_state) => {
            println!("New State:\n{:#?}", new_state);
        }
        Err(err) => {
            eprintln!("Command failed: {}", err);
        }
    }
}
