use super::schema::Transaction;
use crate::{
    cli::{
        expect_field, expect_message,
        parsers::{Bip32PathParser, SerdeJsonFileOrLiteralParser},
        types::Bip32Path,
        CliCommand,
    },
    messages::{self, Message},
    state_machine::StateMachine,
};
use anyhow::{anyhow, Result};
use clap::Args;
use schemars::schema_for;

/// Sign Cosmos transactions
#[derive(Debug, Clone, Args)]
pub struct CosmosSignTx {
    /// BIP-32 path to source address
    #[clap(value_parser = Bip32PathParser, default_value = "m/44'/118'/0'/0/0")]
    address: Bip32Path,
    /// JSON-encoded cosmos tx to sign, or the path to a file containing one
    #[clap(long, value_parser = SerdeJsonFileOrLiteralParser::<Transaction>::new(), long_help(Some(&*Box::leak(serde_json::to_string_pretty(&schema_for!(Transaction)).unwrap().into_boxed_str()))))]
    tx: Transaction,
}

impl CliCommand for CosmosSignTx {
    fn handle(self, state_machine: &dyn StateMachine) -> Result<()> {
        println!("{:#?}", self.tx);
        println!("{}", serde_json::to_string_pretty(&self.tx)?);

        let mut msgs = self.tx.msg.to_vec();
        msgs.reverse();
        let resp = expect_message!(
            Message::CosmosSignedTx,
            state_machine.send_and_handle_or(
                messages::CosmosSignTx {
                    address_n: self.address.into(),
                    chain_id: Some(self.tx.chain_id),
                    account_number: Some(self.tx.account_number),
                    fee_amount: Some(self.tx.fee.amount[0].amount.try_into()?),
                    gas: Some(self.tx.fee.gas.try_into()?),
                    memo: Some(self.tx.memo),
                    sequence: Some(self.tx.sequence),
                    msg_count: Some(msgs.len().try_into()?),
                }
                .into(),
                &mut |msg| {
                    Ok(match msg {
                        Message::CosmosMsgRequest(_) => Some(
                            msgs.pop()
                                .ok_or_else(|| anyhow!("device requested too many messages"))?
                                .as_message()?
                                .into(),
                        ),
                        _ => None,
                    })
                },
            )
        )?;

        println!("{:?}", expect_field!(resp.public_key)?);
        println!("{:?}", expect_field!(resp.signature)?);

        Ok(())
    }
}
