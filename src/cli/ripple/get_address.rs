use crate::{
    cli::{expect_field, expect_message, parsers::Bip32PathParser, types::Bip32Path, CliCommand},
    messages::{self, Message},
    transport::ProtocolAdapter,
};
use anyhow::Result;
use clap::{ArgAction::SetTrue, Args};

/// Get Ripple address
#[derive(Debug, Clone, Args)]
pub struct RippleGetAddress {
    /// BIP-32 path to key (for compatibility with other wallets, must be m/44'/144'/index')
    #[clap(short = 'n', long, value_parser = Bip32PathParser, default_value = "m/44'/144'/0'")]
    address: Bip32Path,
    /// Confirm address on device screen
    #[clap(short = 'd', long, action = SetTrue)]
    show_display: Option<bool>,
}

impl CliCommand for RippleGetAddress {
    fn handle(self, protocol_adapter: &dyn ProtocolAdapter) -> Result<()> {
        let resp = expect_message!(
            Message::RippleAddress,
            protocol_adapter.send_and_handle(
                messages::RippleGetAddress {
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
