use crate::{
    cli::{expect_field, expect_message, parsers::Bip32PathParser, types::Bip32Path, CliCommand},
    messages::{self, Message},
    transport::ProtocolAdapter,
};
use anyhow::Result;
use clap::{ArgAction::SetTrue, Args};

/// Get Tendermint address
#[derive(Debug, Clone, Args)]
pub struct TendermintGetAddress {
        /// BIP-32 path to key
        #[clap(short = 'n', long, value_parser = Bip32PathParser, default_value = "m/44'/118'/0'/0/0")]
        address: Bip32Path,
        /// Confirm address on device screen
        #[clap(short = 'd', long, action = SetTrue)]
        show_display: Option<bool>,
        /// Bech32 prefix for generated address
        #[clap(short = 'p', long)]
        address_prefix: String,
        /// Name of chain (i.e. "cosmos")
        #[clap(short, long)]
        chain_name: String,
}

impl CliCommand for TendermintGetAddress {
    fn handle(self, protocol_adapter: &mut dyn ProtocolAdapter) -> Result<()> {
        let resp = expect_message!(
            Message::TendermintAddress,
            protocol_adapter.with_standard_handler().handle(
                messages::TendermintGetAddress {
                    address_n: self.address.into(),
                    show_display: self.show_display,
                    testnet: None,
                    address_prefix: Some(self.address_prefix),
                    chain_name: Some(self.chain_name),
                }
                .into(),
            )
        )?;

        println!("{}", expect_field!(resp.address)?);

        Ok(())
    }
}
