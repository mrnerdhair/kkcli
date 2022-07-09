use crate::{
    cli::{expect_success, CliCommand},
    messages,
    transport::ProtocolAdapter,
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
    fn handle(self, protocol_adapter: &dyn ProtocolAdapter) -> Result<()> {
        expect_success!(protocol_adapter.send_and_handle(
            messages::ChangePin {
                remove: self.remove
            }
            .into()
        ))?;

        Ok(())
    }
}
