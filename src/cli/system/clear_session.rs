use crate::{
    cli::{expect_success, CliCommand},
    messages,
    state_machine::StateMachine,
};
use anyhow::Result;
use clap::Args;

/// Clear session (remove cached PIN, passphrase, etc.)
#[derive(Debug, Clone, Args)]
pub struct ClearSession;

impl CliCommand for ClearSession {
    fn handle(self, state_machine: &dyn StateMachine) -> Result<()> {
        expect_success!(
            state_machine.send_and_handle(messages::ClearSession::default().into())
        )?;

        Ok(())
    }
}
