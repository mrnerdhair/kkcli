use crate::{
    cli::{
        expect_field, expect_message,
        parsers::Bip32PathParser,
        types::Bip32Path,
        CliCommand,
    },
    messages::{self, Message},
    transport::ProtocolAdapter,
};
use anyhow::Result;
use clap::Args;

/// Sign an message using Ethereum's personal_sign
#[derive(Debug, Clone, Args)]
pub struct EthereumSignMessage {
    /// BIP-32 path to signing key
    #[clap(short = 'n', long, value_parser = Bip32PathParser, default_value = "m/44'/60'/0'/0/0")]
    address: Bip32Path,
    /// Message to sign
    message: String,
}

impl CliCommand for EthereumSignMessage {
    fn handle(self, protocol_adapter: &dyn ProtocolAdapter) -> Result<()> {
        let resp = expect_message!(
            Message::EthereumMessageSignature,
            protocol_adapter.send_and_handle(
                messages::EthereumSignMessage {
                    address_n: self.address.into(),
                    message: self.message.into_bytes(),
                }
                .into()
            )
        )?;

        println!("Address:\t0x{}", hex::encode(expect_field!(resp.address)?));
        println!(
            "Signature:\t0x{}",
            hex::encode(expect_field!(resp.signature)?)
        );

        Ok(())
    }
}
