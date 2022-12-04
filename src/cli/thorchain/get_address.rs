use crate::{
    cli::{expect_field, expect_message, parsers::Bip32PathParser, types::Bip32Path, CliCommand},
    messages::{self, Message},
    transport::ProtocolAdapter,
};
use anyhow::Result;
use clap::{ArgAction::SetTrue, Args};

/// Get Thorchain address
#[derive(Debug, Clone, Args)]
pub struct ThorchainGetAddress {
    /// BIP-32 path to key
    #[clap(short = 'n', long, value_parser = Bip32PathParser, default_value = "m/44'/931'/0'/0/0")]
    address: Bip32Path,
    /// Confirm address on device screen
    #[clap(short = 'd', long, action = SetTrue)]
    show_display: Option<bool>,
    #[clap(short, long, action = SetTrue)]
    testnet: Option<bool>,
}

impl CliCommand for ThorchainGetAddress {
    fn handle(self, protocol_adapter: &mut dyn ProtocolAdapter) -> Result<()> {
        let resp = expect_message!(
            Message::ThorchainAddress,
            protocol_adapter.with_standard_handler().handle(
                messages::ThorchainGetAddress {
                    address_n: self.address.into(),
                    show_display: self.show_display,
                    testnet: self.testnet,
                }
                .into(),
            )
        )?;

        println!("{}", expect_field!(resp.address)?);

        Ok(())
    }
}
