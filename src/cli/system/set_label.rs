
use crate::{
    cli::{expect_success, CliCommand},
    messages,
    transport::ProtocolAdapter,
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
    fn handle(self, protocol_adapter: &mut dyn ProtocolAdapter) -> Result<()> {
        let mut req = messages::ApplySettings::default();
        req.label = Some(self.label);

        expect_success!(protocol_adapter.with_standard_handler().handle(req.into()))?;

        Ok(())
    }
}
