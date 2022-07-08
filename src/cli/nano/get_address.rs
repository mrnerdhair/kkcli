use crate::{
    cli::{expect_field, expect_message, parsers::Bip32PathParser, types::Bip32Path, CliCommand},
    messages::{self, Message},
    state_machine::StateMachine,
};
use anyhow::Result;
use clap::{ArgAction::SetTrue, Args};

/// Get Nano address
#[derive(Debug, Clone, Args)]
pub struct NanoGetAddress {
    /// BIP-32 path to key
    #[clap(value_parser = Bip32PathParser, default_value = "m/44'/165'/0'")]
    address: Bip32Path,
    #[clap(short, long)]
    coin_name: Option<String>,
    #[clap(short = 'd', long, action = SetTrue)]
    show_display: Option<bool>,
}

impl CliCommand for NanoGetAddress {
    fn handle(self, state_machine: &dyn StateMachine) -> Result<()> {
        let resp = expect_message!(
            Message::NanoAddress,
            state_machine.send_and_handle(
                messages::NanoGetAddress {
                    address_n: self.address.into(),
                    coin_name: self.coin_name,
                    show_display: self.show_display,
                }
                .into(),
            )
        )?;

        println!("{}", expect_field!(resp.address)?);

        Ok(())
    }
}
