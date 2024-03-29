use super::schema::Transaction;
use crate::{
    cli::{
        expect_field, expect_message,
        parsers::{Bip32PathParser, SerdeJsonFileOrLiteralParser},
        types::Bip32Path,
        CliCommand,
    },
    messages::{self, Message},
    transport::ProtocolAdapter,
};
use anyhow::{anyhow, Result};
use clap::Args;
use schemars::schema_for;

/// Sign Cosmos transaction
#[derive(Debug, Clone, Args)]
pub struct CosmosSignTx {
    /// BIP-32 path to source address
    #[clap(short = 'n', long, value_parser = Bip32PathParser, default_value = "m/44'/118'/0'/0/0")]
    address: Bip32Path,
    /// JSON-encoded cosmos tx to sign, or the path to a file containing one
    #[clap(long, value_parser = SerdeJsonFileOrLiteralParser::<Transaction>::new(), long_help(Some(&*Box::leak(serde_json::to_string_pretty(&schema_for!(Transaction)).unwrap().into_boxed_str()))))]
    tx: Transaction,
}

impl CliCommand for CosmosSignTx {
    fn handle(self, protocol_adapter: &mut dyn ProtocolAdapter) -> Result<()> {
        let mut msgs = self.tx.msg.to_vec();
        let msg_count = msgs.len();
        msgs.reverse();
        let resp = expect_message!(
            Message::CosmosSignedTx,
            protocol_adapter
                .with_standard_handler()
                .with_mut_handler(&mut |msg| {
                    Ok(match msg {
                        Message::CosmosMsgRequest(_) => Some(
                            msgs.pop()
                                .ok_or_else(|| anyhow!("device requested too many messages"))?
                                .as_message()?
                                .into(),
                        ),
                        _ => None,
                    })
                },)
                .handle(
                    messages::CosmosSignTx {
                        address_n: self.address.into(),
                        chain_id: Some(self.tx.chain_id),
                        account_number: Some(self.tx.account_number),
                        fee_amount: Some(self.tx.fee.amount[0].amount.try_into()?),
                        gas: Some(self.tx.fee.gas.try_into()?),
                        memo: Some(self.tx.memo),
                        sequence: Some(self.tx.sequence),
                        msg_count: Some(msg_count.try_into()?),
                    }
                    .into()
                )
        )?;

        println!(
            "Public Key:\t{}",
            hex::encode(expect_field!(resp.public_key)?)
        );
        println!(
            "Signature:\t{}",
            hex::encode(expect_field!(resp.signature)?)
        );

        Ok(())
    }
}
