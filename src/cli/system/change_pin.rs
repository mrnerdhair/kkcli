use crate::{
    cli::{expect_success, CliCommand},
    messages,
    state_machine::StateMachine,
};
use anyhow::Result;
use clap::{ArgAction::SetTrue, Args};

/// Change new PIN or remove existing
#[derive(Debug, Clone, Args)]
pub struct ChangePin {
    #[clap(short, long, action = SetTrue)]
    remove: Option<bool>,
}

impl CliCommand for ChangePin {
    fn handle(self, state_machine: &dyn StateMachine) -> Result<()> {
        expect_success!(state_machine.send_and_handle(
            messages::ChangePin {
                remove: self.remove
            }
            .into()
        ))?;

        Ok(())
    }
}
