use log::*;
use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;
use std::process::Child;

pub struct ChildGuard(pub Child);

/// A guard of a child os process, sends a ctrl-c SIGINT to child process when it is dropped.
impl Drop for ChildGuard {
    fn drop(&mut self) {
        let pid = self.0.id() as i32;
        // send ctrl-c to child process to let it gracefully shut down
        match signal::kill(Pid::from_raw(pid), Signal::SIGINT) {
            Err(e) => debug!("could not kill child process id {}: {}", pid, e),
            Ok(_) => debug!("killed child process id {}", pid),
        }
    }
}
