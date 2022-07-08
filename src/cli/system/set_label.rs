
use crate::{
    cli::{expect_success, CliCommand},
    messages,
    state_machine::StateMachine,
};
use anyhow::Result;
use clap::Args;

/// Set new wallet label
#[derive(Debug, Clone, Args)]
pub struct SetLabel {
    #[clap(short, long)]
    label: String,
}

impl CliCommand for SetLabel {
    fn handle(self, state_machine: &dyn StateMachine) -> Result<()> {
        let mut req = messages::ApplySettings::default();
        req.label = Some(self.label);

        expect_success!(state_machine.send_and_handle(req.into()))?;

        Ok(())
    }
}
