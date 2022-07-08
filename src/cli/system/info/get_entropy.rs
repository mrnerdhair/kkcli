use crate::{
    cli::{expect_message, CliCommand},
    messages::{self, Message},
    state_machine::StateMachine,
};
use anyhow::Result;
use clap::Args;

/// Get entropy from the device
#[derive(Debug, Clone, Args)]
pub struct GetEntropy {
    size: u32,
}

impl CliCommand for GetEntropy {
    fn handle(self, state_machine: &dyn StateMachine) -> Result<()> {
        let resp = expect_message!(
            Message::Entropy,
            state_machine.send_and_handle(messages::GetEntropy { size: self.size }.into())
        )?;
        println!("{}", hex::encode(resp.entropy));

        Ok(())
    }
}
