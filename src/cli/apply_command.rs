use std::os::linux::raw::stat;

use crate::cli::Command;
use crate::cli::ControlState;
use crate::cli::handle::*;


pub fn apply_command(
    state: ControlState,
    command: Command,
) -> Result<ControlState, String> {
    match command {
        Command::Start { mode } => handle_start(state, mode),
        Command::Stop => handle_stop(state),
        Command::Pause => handle_pause(state),
        Command::Resume => handle_resume(state),

        Command::Lock => handle_lock(state),
        Command::Unlock => handle_unlock(state),

        Command::Kill => Ok(handle_kill(state)),
        Command::Reset { confirm } => handle_reset(state, &confirm),

        Command::Status => {
            display_status(&state);
            Ok(state)
        }
        Command::Explain => {
            display_explain(&state);
            Ok(state)
        }
        _ => Ok(state),
    }
}