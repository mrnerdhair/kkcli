use super::schema::Transaction;
use crate::{
    cli::{
        expect_field, expect_message,
        parsers::{Bip32PathParser, HexParser32, SerdeJsonFileOrLiteralParser},
        types::Bip32Path,
        CliCommand,
    },
    messages::{self, Message},
    state_machine::StateMachine,
};
use anyhow::{anyhow, Result};
use clap::Args;
use schemars::schema_for;

/// Sign EOS transaction
#[derive(Debug, Clone, Args)]
pub struct EosSignTx {
    /// BIP-32 path to source address
    #[clap(value_parser = Bip32PathParser, default_value = "m/44'/194'/0'/0/0")]
    address: Bip32Path,
    #[clap(short, long, value_parser = HexParser32)]
    chain_id: [u8; 32],
    /// JSON-encoded EOS tx to sign, or the path to a file containing one
    #[clap(long, value_parser = SerdeJsonFileOrLiteralParser::<Transaction>::new(), long_help(Some(&*Box::leak(serde_json::to_string_pretty(&schema_for!(Transaction)).unwrap().into_boxed_str()))))]
    tx: Transaction,
}

impl CliCommand for EosSignTx {
    fn handle(self, state_machine: &dyn StateMachine) -> Result<()> {
        println!("{:#?}", self.tx);
        println!("{}", serde_json::to_string_pretty(&self.tx)?);

        let mut actions = self.tx.actions;
        actions.reverse(); // reverse the list so pop() will happen in order
        let resp = expect_message!(
            Message::EosSignedTx,
            state_machine.send_and_handle_or(
                messages::EosSignTx {
                    address_n: self.address.into(),
                    chain_id: Some(self.chain_id.to_vec()),
                    header: Some(messages::EosTxHeader {
                        expiration: self.tx.header.expiration.timestamp().try_into()?,
                        ref_block_num: self.tx.header.ref_block_num.into(),
                        ref_block_prefix: self.tx.header.ref_block_prefix,
                        max_net_usage_words: self.tx.header.max_net_usage_words,
                        max_cpu_usage_ms: self.tx.header.max_cpu_usage_ms.into(),
                        delay_sec: self.tx.header.delay_sec,
                    }),
                    num_actions: Some(actions.len().try_into()?),
                }
                .into(),
                &mut |msg| {
                    Ok(match msg {
                        Message::EosTxActionRequest(_) => Some(
                            actions
                                .pop()
                                .ok_or_else(|| anyhow!("device requested too many actions"))?
                                .as_tx_action_ack()?
                                .into(),
                        ),
                        _ => None,
                    })
                },
            )
        )?;

        let v = *expect_field!(resp.signature_v)?;
        let r = expect_field!(resp.signature_r)?;
        let s = expect_field!(resp.signature_s)?;

        let v: u8 = v.try_into()?;
        assert_eq!(r.len(), 32);
        assert_eq!(s.len(), 32);
        println!("{}{}{}", hex::encode(r), hex::encode(s), hex::encode(&[v]));

        Ok(())
    }
}
