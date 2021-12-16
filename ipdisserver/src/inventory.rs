use crate::answers::{BeaconInfos, FromCmdOutput};
use crate::exec::InventoryCommand;
use crate::hostname::get_hostname;
use std::path::{Path, PathBuf};

pub struct InternalInventory {
    pub key: String,
    pub source: Box<dyn Fn() -> String>,
}

impl Default for InternalInventory {
    fn default() -> Self {
        Self {
            key: "hostname".into(),
            source: Box::from(get_hostname),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct InventoryFile {
    pub path: PathBuf,
}

impl From<&Path> for InventoryFile {
    fn from(path: &Path) -> Self {
        Self { path: path.into() }
    }
}

pub trait ExecuteInventory {
    fn execute(&self) -> InventoryOutput;
}

impl ExecuteInventory for InternalInventory {
    fn execute(&self) -> InventoryOutput {
        let raw_output = (*self.source)();
        let mut output = BeaconInfos::new();
        output.insert(self.key.clone(), raw_output.clone().into());
        InventoryOutput { raw_output, output }
    }
}

impl ExecuteInventory for InventoryFile {
    fn execute(&self) -> InventoryOutput {
        let mut command = InventoryCommand::new(&self.path);
        let raw_output = command.output();
        let output = BeaconInfos::from_cmd_output(&raw_output).unwrap_or_default();
        InventoryOutput { raw_output, output }
    }
}

#[derive(Debug, Clone, Default)]
pub struct InventoryOutput {
    pub raw_output: String,
    pub output: BeaconInfos,
}
