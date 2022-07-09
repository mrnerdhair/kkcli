use crate::{
    cli::{
        expect_field, expect_message,
        parsers::Bip32PathParser,
        types::{Bip32Path, EosPublicKeyKind},
        CliCommand,
    },
    messages::{self, Message},
    transport::ProtocolAdapter,
};
use anyhow::Result;
use clap::{ArgAction::SetTrue, Args};

/// Get EOS public key
#[derive(Debug, Clone, Args)]
pub struct EosGetPublicKey {
    /// BIP-32 path to key
    #[clap(short = 'n', long, value_parser = Bip32PathParser, default_value = "m/44'/194'/0'/0/0")]
    address: Bip32Path,
    #[clap(short = 'd', long, action = SetTrue)]
    /// Confirm address on device screen
    show_display: Option<bool>,
    #[clap(short, long, value_enum)]
    kind: Option<EosPublicKeyKind>,
}

impl CliCommand for EosGetPublicKey {
    fn handle(self, protocol_adapter: &dyn ProtocolAdapter) -> Result<()> {
        let resp = expect_message!(
            Message::EosPublicKey,
            protocol_adapter.send_and_handle(
                messages::EosGetPublicKey {
                    address_n: self.address.into(),
                    show_display: self.show_display,
                    kind: self.kind.map(|x| x.into()),
                }
                .into(),
            )
        )?;

        println!("{}", expect_field!(resp.wif_public_key)?);

        Ok(())
    }
}
