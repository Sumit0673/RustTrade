use crate::cli::control_state;
pub enum Command{
    
    /// Shows current control state (read-only)

    Status,
    Logs {tail: usize},
    Explain,

    // Mutable
    Start {mode: control_state::Mode},
    Stop,
    Pause,
    Resume,
    Lock,
    Unlock,

    //Emergency
    Kill,
    Reset {confirm: String}

}