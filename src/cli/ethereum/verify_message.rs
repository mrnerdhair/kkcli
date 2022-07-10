use crate::{
    cli::{
        parsers::{HexParser20, HexParser65},
        CliCommand,
    },
    messages::{self, Message},
    transport::ProtocolAdapter,
};
use anyhow::{bail, Result};
use clap::Args;

/// Verify a message signed using Ethereum's personal_sign
#[derive(Debug, Clone, Args)]
pub struct EthereumVerifyMessage {
    /// Message
    message: String,
    /// Address which signed the message
    #[clap(value_parser = HexParser20)]
    address: [u8; 20],
    /// Signature to verify
    #[clap(value_parser = HexParser65)]
    signature: [u8; 65],
}

impl CliCommand for EthereumVerifyMessage {
    fn handle(self, protocol_adapter: &mut dyn ProtocolAdapter) -> Result<()> {
        match protocol_adapter.with_standard_handler().handle(
            messages::EthereumVerifyMessage {
                address: Some(self.address.to_vec()),
                message: Some(self.message.into_bytes()),
                signature: Some(self.signature.to_vec()),
            }
            .into(),
        )? {
            Message::Success(_) => (),
            Message::Failure(_) => (),
            x => bail!("unexpected message ({:?})", x),
        }

        Ok(())
    }
}
