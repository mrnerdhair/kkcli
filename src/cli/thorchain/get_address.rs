use crate::{
    cli::{expect_field, expect_message, parsers::Bip32PathParser, types::Bip32Path, CliCommand},
    messages::{self, Message},
    state_machine::StateMachine,
};
use anyhow::Result;
use clap::{ArgAction::SetTrue, Args};

/// Get Thorchain address
#[derive(Debug, Clone, Args)]
pub struct ThorchainGetAddress {
        /// BIP-32 path to key
        #[clap(value_parser = Bip32PathParser, default_value = "m/44'/931'/0'/0/0")]
        address: Bip32Path,
        #[clap(short = 'd', long, action = SetTrue)]
        show_display: Option<bool>,
        #[clap(short, long, action = SetTrue)]
        testnet: Option<bool>,
}

impl CliCommand for ThorchainGetAddress {
    fn handle(self, state_machine: &dyn StateMachine) -> Result<()> {
        let resp = expect_message!(
            Message::ThorchainAddress,
            state_machine.send_and_handle(
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
