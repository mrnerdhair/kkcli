use crate::{
    cli::{expect_success, CliCommand},
    messages,
    state_machine::StateMachine,
};
use anyhow::Result;
use clap::Args;

/// Reset device to factory defaults and remove all private data.
#[derive(Debug, Clone, Args)]
pub struct WipeDevice;

impl CliCommand for WipeDevice {
    fn handle(self, state_machine: &dyn StateMachine) -> Result<()> {
        expect_success!(
            state_machine.send_and_handle(messages::WipeDevice::default().into())
        )?;

        Ok(())
    }
}
