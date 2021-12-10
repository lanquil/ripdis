use std::path::Path;
use std::process::Command;
use tracing::{error, warn};

pub struct InventoryCommand {
    cmd: Command,
}

impl InventoryCommand {
    pub fn new<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let cmd = Command::new(path.as_ref().to_path_buf());
        Self { cmd }
    }

    /// Return raw output string, an empty string if the execution is not successful.
    pub fn output(&mut self) -> String {
        match self.cmd.output() {
            Ok(output) => {
                if !output.stderr.is_empty() {
                    // warn!(?self.cmd, stderr = String::from_utf8_lossy(&output.stderr), "Inventory file wrote on stderr.");
                    warn!(?self.cmd, ?output.stderr, "Inventory file wrote on stderr.");
                }
                match output.status.success() {
                    true => String::from_utf8_lossy(&output.stdout).into(),
                    false => {
                        error!(?self.cmd, ?output.status, "Inventory file: non-0 exit code.");
                        String::new()
                    }
                }
            }
            Err(error) => {
                error!(?self.cmd, ?error, "Failed executing inventory file.");
                String::new()
            }
        }
    }
}
