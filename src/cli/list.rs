use crate::{cli::CliCommand, state_machine::StateMachine};
use anyhow::Result;
use clap::Args;

/// List connected KeepKey USB devices
#[derive(Debug, Clone, Args)]
pub struct List;

impl CliCommand for List {
    fn handle(self, _state_machine: &dyn StateMachine) -> Result<()> {
        unreachable!();
    }
}
