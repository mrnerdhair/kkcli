use crate::{
    cli::{expect_field, expect_message, parsers::Bip32PathParser, types::Bip32Path, CliCommand},
    messages::{self, Message},
    transport::ProtocolAdapter,
};
use anyhow::Result;
use clap::Args;

/// Sign Cosmos transactions
#[derive(Debug, Clone, Args)]
pub struct RippleSignTx {
    /// BIP-32 path to source address (for compatibility with other wallets, must be m/44'/144'/index')
    #[clap(short = 'n', long, value_parser = Bip32PathParser, default_value = "m/44'/144'/0'")]
    address: Bip32Path,
    /// fee (in drops) for the transaction
    #[clap(long, value_parser = clap::value_parser!(u64).range(10..=1000000))]
    fee: u64,
    /// transaction flags
    #[clap(long)]
    flags: Option<u32>,
    /// transaction sequence number
    #[clap(long)]
    sequence: Option<u32>,
    /// see <https://developers.ripple.com/reliable-transaction-submission.html#lastledgersequence>
    #[clap(long)]
    last_ledger_sequence: Option<u32>,
    /// amount of XRP to send
    #[clap(long)]
    amount: Option<u64>,
    /// destination account address
    #[clap(long)]
    destination: Option<String>,
    /// destination tag to identify payments
    #[clap(long)]
    destination_tag: Option<u32>,
}

impl CliCommand for RippleSignTx {
    fn handle(self, protocol_adapter: &dyn ProtocolAdapter) -> Result<()> {
        let resp = expect_message!(
            Message::RippleSignedTx,
            protocol_adapter.send_and_handle(
                messages::RippleSignTx {
                    address_n: self.address.into(),
                    fee: Some(self.fee),
                    flags: self.flags,
                    sequence: self.sequence,
                    last_ledger_sequence: self.last_ledger_sequence,
                    payment: Some(messages::RipplePayment {
                        amount: self.amount,
                        destination: self.destination,
                        destination_tag: self.destination_tag,
                    }),
                }
                .into(),
            )
        )?;

        println!(
            "Serialized Tx:\t{}",
            hex::encode(expect_field!(resp.serialized_tx)?)
        );
        println!(
            "Signature:\t{}",
            hex::encode(expect_field!(resp.signature)?)
        );

        Ok(())
    }
}
