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

/// Sign Cosmos transactions
#[derive(Debug, Clone, Args)]
pub struct BinanceSignTx {
    /// BIP-32 path to source address
    #[clap(short = 'n', long, value_parser = Bip32PathParser, default_value = "m/44'/714'/0'/0/0")]
    address: Bip32Path,
    /// JSON-encoded cosmos tx to sign, or the path to a file containing one
    #[clap(long, value_parser = SerdeJsonFileOrLiteralParser::<Transaction>::new(), long_help(Some(&*Box::leak(serde_json::to_string_pretty(&schema_for!(Transaction)).unwrap().into_boxed_str()))))]
    tx: Transaction,
}

impl CliCommand for BinanceSignTx {
    fn handle(self, protocol_adapter: &dyn ProtocolAdapter) -> Result<()> {
        let mut msgs = self.tx.msgs.to_vec();
        msgs.reverse();
        let resp = expect_message!(
            Message::BinanceSignedTx,
            protocol_adapter.send_and_handle_or(
                messages::BinanceSignTx {
                    address_n: self.address.into(),
                    chain_id: Some(self.tx.chain_id),
                    account_number: Some(self.tx.account_number.try_into()?),
                    memo: Some(self.tx.memo),
                    sequence: Some(self.tx.sequence.try_into()?),
                    msg_count: Some(msgs.len().try_into()?),
                    source: Some(self.tx.source.try_into()?),
                }
                .into(),
                &mut |msg| {
                    Ok(match msg {
                        Message::BinanceTxRequest(_) => Some(
                            msgs.pop()
                                .ok_or_else(|| anyhow!("device requested too many messages"))?
                                .as_message()?,
                        ),
                        _ => None,
                    })
                },
            )
        )?;

        println!("Public Key:\t{}", hex::encode(expect_field!(resp.public_key)?));
        println!("Signature:\t{}", hex::encode(expect_field!(resp.signature)?));

        Ok(())
    }
}
