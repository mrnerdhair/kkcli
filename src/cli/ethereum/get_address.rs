use crate::{
    cli::{expect_field, expect_message, parsers::Bip32PathParser, types::Bip32Path, CliCommand},
    messages::{self, Message},
    transport::ProtocolAdapter,
};
use anyhow::Result;
use clap::{ArgAction::SetTrue, Args};

/// Get Ethereum address in hex encoding
#[derive(Debug, Clone, Args)]
pub struct EthereumGetAddress {
    /// BIP-32 path to key
    #[clap(short = 'n', long, value_parser = Bip32PathParser, default_value = "m/44'/60'/0'/0/0")]
    address: Bip32Path,
    /// Confirm address on device screen
    #[clap(short = 'd', long, action = SetTrue)]
    show_display: Option<bool>,
}

impl CliCommand for EthereumGetAddress {
    fn handle(self, protocol_adapter: &mut dyn ProtocolAdapter) -> Result<()> {
        let resp = expect_message!(
            Message::EthereumAddress,
            protocol_adapter.with_standard_handler().handle(
                messages::EthereumGetAddress {
                    address_n: self.address.into(),
                    show_display: self.show_display,
                }
                .into(),
            )
        )?;

        println!("{}", expect_field!(resp.address_str)?);

        Ok(())
    }
}
