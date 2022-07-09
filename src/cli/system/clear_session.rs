use crate::{
    cli::{expect_success, CliCommand},
    messages,
    transport::ProtocolAdapter,
};
use anyhow::Result;
use clap::Args;

/// Clear session (remove cached PIN, passphrase, etc.)
#[derive(Debug, Clone, Args)]
pub struct ClearSession;

impl CliCommand for ClearSession {
    fn handle(self, protocol_adapter: &dyn ProtocolAdapter) -> Result<()> {
        expect_success!(
            protocol_adapter.send_and_handle(messages::ClearSession::default().into())
        )?;

        Ok(())
    }
}
