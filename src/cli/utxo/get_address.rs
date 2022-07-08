use crate::{
    cli::{
        expect_message,
        parsers::Bip32PathParser,
        types::{Bip32Path, ScriptType},
        CliCommand,
    },
    messages::{self, Message},
    state_machine::StateMachine,
};
use anyhow::Result;
use clap::{ArgAction::SetTrue, Args};

/// Get bitcoin address in base58 encoding
#[derive(Debug, Clone, Args)]
pub struct GetAddress {
    #[clap(short, long)]
    coin_name: Option<String>,
    /// BIP-32 path to key
    #[clap(value_parser = Bip32PathParser)]
    address: Bip32Path,
    #[clap(value_enum, short = 't', long)]
    script_type: Option<ScriptType>,
    #[clap(short = 'd', long, action = SetTrue)]
    show_display: Option<bool>,
}

impl CliCommand for GetAddress {
    fn handle(self, state_machine: &dyn StateMachine) -> Result<()> {
        let resp = expect_message!(
            Message::Address,
            state_machine.send_and_handle(
                messages::GetAddress {
                    coin_name: self.coin_name,
                    address_n: self.address.into(),
                    script_type: self.script_type.map(|x| x.into()),
                    show_display: self.show_display,
                    multisig: None,
                }
                .into()
            ),
        )?;

        println!("{}", resp.address);

        Ok(())
    }
}
