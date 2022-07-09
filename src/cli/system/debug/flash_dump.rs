use crate::{
    cli::{expect_field, expect_message, CliDebugCommand},
    messages::{self, Message},
    transport::ProtocolAdapter,
};
use anyhow::{anyhow, Result};
use clap::Args;

/// On DEBUG_LINK firmware, dumps a section of flash memory.
#[derive(Debug, Clone, Args)]
pub struct DebugLinkFlashDump {
    address: u32,
    length: u32,
}

impl CliDebugCommand for DebugLinkFlashDump {
    fn handle_debug(
        self,
        _: &dyn ProtocolAdapter,
        debug_protocol_adapter: Option<&dyn ProtocolAdapter>,
    ) -> Result<()> {
        let debug_protocol_adapter = debug_protocol_adapter
            .ok_or_else(|| anyhow!("this command requires a DEBUG_LINK connection"))?;

        let resp = expect_message!(
            Message::DebugLinkFlashDumpResponse,
            debug_protocol_adapter.send_and_handle(
                messages::DebugLinkFlashDump {
                    address: Some(self.address),
                    length: Some(self.length),
                }
                .into()
            )
        )?;

        println!("{}", hex::encode(expect_field!(resp.data)?));

        Ok(())
    }
}
