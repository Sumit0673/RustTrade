use crate::cli::LifecycleState;
use crate::cli::ExecutionLock;
use crate::cli::ControlState;
use crate::cli::Mode;
use crate::cli::DecisionOutcome;
use colored::*;
use std::time::SystemTime;

pub fn handle_start(state: ControlState, mode: Mode) -> Result<ControlState, String>{
    if state.killed{
        return Err("System is Killed. Reset required before starting!".into());
    }

    match state.lifecycle {
        LifecycleState::Running => {
            return Err("System is already running.".into());
        }
        _ => {}
    }

    match state.execution_lock {
        ExecutionLock::Locked => {
            return Err("System is Locked. Unlock before starting".into());
        }
        ExecutionLock::Unlocked => {}
    }

    let new_state = ControlState {
        lifecycle: LifecycleState::Running,
        mode,
        execution_lock: state.execution_lock,
        killed: state.killed,
        last_transition: std::time::SystemTime::now(),
        last_decisions: state.last_decisions,
    };

    Ok(new_state)
}


pub fn handle_stop(state: ControlState) -> Result<ControlState, String>{
    if state.killed{
        return Err("System is Killed. Reset required".into());
    }

    match state.lifecycle {
        LifecycleState::Running | LifecycleState::Paused => {}
        LifecycleState::Stopped => {
            return Err("System is already stopped.".into());
        }
    }
    

    let new_state = ControlState {
        lifecycle: LifecycleState::Stopped,
        mode:state.mode,
        execution_lock: state.execution_lock,
        killed: state.killed,
        last_transition: std::time::SystemTime::now(),
        last_decisions: state.last_decisions,
    };

    Ok(new_state)
}

pub fn handle_pause(state: ControlState) -> Result<ControlState, String>{
    if state.killed{
        return Err("System is Killed. No need to pause!".into());
    }

    match state.lifecycle {
    LifecycleState::Running => {}
    LifecycleState::Paused => return Err("System is already paused.".into()),
    LifecycleState::Stopped => return Err("System is not running.".into()),
}

    let new_state = ControlState {
        lifecycle: LifecycleState::Paused,
        mode:state.mode,
        execution_lock: state.execution_lock,
        killed: state.killed,
        last_transition: std::time::SystemTime::now(),
        last_decisions: state.last_decisions,
    };

    Ok(new_state)
}

pub fn handle_resume(state: ControlState) -> Result<ControlState, String>{
    if state.killed{
        return Err("System is Killed. Reset required".into());
    }

    match state.lifecycle {
        LifecycleState::Paused => {}
        LifecycleState::Running => return Err("System is already running.".into()),
        LifecycleState::Stopped => return Err("System is stoped, can't resume.".into()),
    }


    let new_state = ControlState {
        lifecycle: LifecycleState::Running,
        mode:state.mode,
        execution_lock: state.execution_lock,
        killed: state.killed,
        last_transition: std::time::SystemTime::now(),
        last_decisions: state.last_decisions,
    };

    Ok(new_state)
}

pub fn handle_lock(state: ControlState) -> Result<ControlState, String>{
    if state.killed{
        return Err("System is Killed. Reset required".into());
    }

    match state.execution_lock{
        ExecutionLock::Unlocked => {},
        ExecutionLock::Locked => return Err("System is already locked".into()),
    }


    let new_state = ControlState {
        lifecycle: state.lifecycle,
        mode:state.mode,
        execution_lock: ExecutionLock::Locked,
        killed: state.killed,
        last_transition: std::time::SystemTime::now(),
        last_decisions: state.last_decisions,
    };

    Ok(new_state)
}

pub fn handle_unlock(state: ControlState) -> Result<ControlState, String>{
    if state.killed{
        return Err("System is Killed. Reset required".into());
    }

    match state.execution_lock{
        ExecutionLock::Locked => {},
        ExecutionLock::Unlocked => return Err("System is already unlocked".into()),
    }


    let new_state = ControlState {
        lifecycle: state.lifecycle,
        mode:state.mode,
        execution_lock: ExecutionLock::Unlocked,
        killed: state.killed,
        last_transition: std::time::SystemTime::now(),
        last_decisions: state.last_decisions,
    };

    Ok(new_state)
}

pub fn handle_kill(state: ControlState) -> ControlState {
    println!("Killing the process, setting LifecycleState - Stopped, ExecutionLock - Locked ");
    ControlState {
        killed: true,         
        lifecycle: LifecycleState::Stopped,
        execution_lock: ExecutionLock::Locked,
        last_transition: std::time::SystemTime::now(),

        mode: state.mode,
        last_decisions: state.last_decisions,
    }
}


pub fn handle_reset(state: ControlState, user_confirm: &str) -> Result<ControlState, String> {

    if user_confirm != "I UNDERSTAND" {
        return Err("Reset requires confirmation: type 'I UNDERSTAND'.".into());
    }

    if !state.killed {
        return Err("System is not killed; reset is not allowed.".into());
    }

    let new_state = ControlState {
        killed: false,
        lifecycle: LifecycleState::Stopped,
        execution_lock: ExecutionLock::Unlocked,
        last_transition: SystemTime::now(),
        mode: state.mode,
        last_decisions: Vec::new(),
    };

    println!("[AUDIT] Reset executed at {:?}", new_state.last_transition);

    Ok(new_state)
}




pub fn display_status(state: &ControlState) {
    println!("==== Control State ====");

    let lifecycle = match state.lifecycle {
        LifecycleState::Running => "Running".green(),
        LifecycleState::Paused => "Paused".yellow(),
        LifecycleState::Stopped => "Stopped".white(),
    };
    println!("Lifecycle : {}", lifecycle);

    let mode = match state.mode {
        Mode::Info => "Info".blue(),
        Mode::DryRun => "DryRun".cyan(),
        Mode::Live => "Live".magenta(),
    };
    println!("Mode      : {}", mode);

    let exec_lock = match state.execution_lock {
        ExecutionLock::Unlocked => "Unlocked".green(),
        ExecutionLock::Locked => "Locked".red(),
    };
    println!("Exec Lock : {}", exec_lock);

    let killed = if state.killed { "Yes".red() } else { "No".green() };
    println!("Killed    : {}", killed);

    println!("Last Trans: {:?}", state.last_transition);

    println!("======================");
}

pub fn display_explain(state: &ControlState) {
    println!("==== Last Decisions ====");

    if state.last_decisions.is_empty() {
        println!("{}", "No decisions yet.".yellow());
        return;
    }

    for dec in &state.last_decisions {
        let outcome_colored = match dec.outcome {
            DecisionOutcome::Accepted => "ACCEPTED".green(),
            DecisionOutcome::Rejected => "REJECTED".yellow(),
            DecisionOutcome::Failed   => "FAILED".red(),
        };

        println!(
            "{} {} on {} (conf {:.2}) [{}]",
            dec.action.bold(),
            dec.symbol.bold(),
            dec.reason,
            dec.confidence,
            outcome_colored
        );
    }

    println!("========================");
}
