use crate::{
    cli::{expect_field, expect_message, parsers::Bip32PathParser, types::Bip32Path, CliCommand},
    messages::{self, Message},
    state_machine::StateMachine,
};
use anyhow::Result;
use clap::{ArgAction::SetTrue, Args};

/// Get Ethereum address in hex encoding
#[derive(Debug, Clone, Args)]
pub struct EthereumGetAddress {
    /// BIP-32 path to key
    #[clap(value_parser = Bip32PathParser)]
    address: Bip32Path,
    #[clap(short = 'd', long, action = SetTrue)]
    show_display: Option<bool>,
}

impl CliCommand for EthereumGetAddress {
    fn handle(self, state_machine: &dyn StateMachine) -> Result<()> {
        let resp = expect_message!(
            Message::EthereumAddress,
            state_machine.send_and_handle(
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
