use crate::{
    cli::{expect_field, expect_message, parsers::Bip32PathParser, types::Bip32Path, CliCommand},
    messages::{self, Message},
    transport::ProtocolAdapter,
};
use anyhow::Result;
use clap::{ArgAction::SetTrue, Args};

/// Get Cosmos address
#[derive(Debug, Clone, Args)]
pub struct CosmosGetAddress {
        /// BIP-32 path to key
        #[clap(short = 'n', long, value_parser = Bip32PathParser, default_value = "m/44'/118'/0'/0/0")]
        address: Bip32Path,
        /// Confirm address on device screen
        #[clap(short = 'd', long, action = SetTrue)]
        show_display: Option<bool>,
}

impl CliCommand for CosmosGetAddress {
    fn handle(self, protocol_adapter: &mut dyn ProtocolAdapter) -> Result<()> {
        let resp = expect_message!(
            Message::CosmosAddress,
            protocol_adapter.with_standard_handler().handle(
                messages::CosmosGetAddress {
                    address_n: self.address.into(),
                    show_display: self.show_display,
                }
                .into(),
            )
        )?;

        println!("{}", expect_field!(resp.address)?);

        Ok(())
    }
}
