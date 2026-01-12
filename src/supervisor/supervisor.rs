use std::time::{Duration, Instant};

use crate::cli::{ControlState, LifecycleState};
use crate::orchestrator::{FailurePolicy, Orchestrator};
use crate::types::{RuntimeState, SystemError};
use crate::types::failure::FailureAction;

pub struct Supervisor<O> {
    orchestrator: O,
    tick_interval: Duration,
}


impl<O> Supervisor<O>
where
    O: Fn() -> Result<(), SystemError>,
{
    pub fn run(
        &self,
        control: &mut ControlState,
        runtime: &mut RuntimeState,
    ) {
        let mut last_tick = Instant::now();

        loop {
            // ---- KILL CHECK ----
            if control.killed {
                eprintln!("System killed. Exiting supervisor.");
                break;
            }

            // ---- LIFECYCLE CHECK ----
            if control.lifecycle != LifecycleState::Running {
                std::thread::sleep(Duration::from_secs(1));
                continue;
            }

            // ---- TICK PACING ----
            if last_tick.elapsed() < self.tick_interval {
                std::thread::sleep(Duration::from_millis(200));
                continue;
            }

            last_tick = Instant::now();

            // ---- EXECUTE TICK ----
            match (self.orchestrator)() {
                Ok(_) => {
                    runtime.record_success();
                }
                Err(err) => {
                    runtime.record_failure(&err.to_string());

                    let action = FailurePolicy::decide(&err, runtime);

                    match action {
                        FailureAction::Retry => {
                            // simple backoff
                            std::thread::sleep(Duration::from_secs(1));
                        }
                        FailureAction::Pause => {
                            eprintln!("Supervisor pausing system");
                            control.lifecycle = LifecycleState::Paused;
                        }
                        FailureAction::Lock => {
                            eprintln!("Supervisor locking execution");
                            control.execution_lock.lock();
                        }
                        FailureAction::Kill => {
                            eprintln!("Supervisor killing system");
                            control.killed = true;
                        }
                        FailureAction::Ignore => {}
                    }
                }
            }
        }
    }
}
