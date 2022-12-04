use crate::{
    cli::{
        expect_field, expect_message,
        parsers::{Bip32PathParser, HexParser32},
        types::Bip32Path,
        CliCommand,
    },
    messages::{self, Message},
    transport::ProtocolAdapter,
};
use anyhow::Result;
use clap::{builder::ArgGroup, Args};

/// Sign Nano transaction
#[derive(Debug, Clone, Args)]
#[clap(group(ArgGroup::new("recipient").required(true)))]
pub struct NanoSignTx {
    #[clap(short, long)]
    coin_name: Option<String>,
    /// BIP-32 path to source address
    #[clap(short = 'n', long, value_parser = Bip32PathParser, default_value = "m/44'/165'/0'")]
    address: Bip32Path,
    /// Current account top block parent block hash
    #[clap(long, value_parser = HexParser32, requires_all(&["top-parent-link", "top-parent-representative", "top-parent-balance"]))]
    top_parent_hash: Option<[u8; 32]>,
    /// Current account top block link field
    #[clap(long, value_parser = HexParser32, requires("top-parent-hash"))]
    top_parent_link: Option<[u8; 32]>,
    /// Current account top block representative address
    #[clap(long, requires("top-parent-hash"))]
    top_parent_representative: Option<String>,
    /// Current account top block balance in raws
    #[clap(long, requires("top-parent-hash"))]
    top_parent_balance: Option<u128>,
    /// Block hash from which to receive funds
    #[clap(long, value_parser = HexParser32, group = "recipient")]
    link_hash: Option<[u8; 32]>,
    /// Address of the recipient
    #[clap(long, group = "recipient", requires("top-parent-hash"))]
    link_recipient: Option<String>,
    /// BIP-32 path for own account to use as recipient
    #[clap(long, value_parser = Bip32PathParser, group = "recipient", requires("top-parent-hash"))]
    link_recipient_n: Option<Bip32Path>,
    /// Representative for the account
    #[clap(long)]
    representative: String,
    /// New account balance in raws
    #[clap(long)]
    balance: u128,
}

impl CliCommand for NanoSignTx {
    fn handle(self, protocol_adapter: &mut dyn ProtocolAdapter) -> Result<()> {
        let resp = expect_message!(
            Message::NanoSignedTx,
            protocol_adapter.with_standard_handler().handle(
                messages::NanoSignTx {
                    address_n: self.address.into(),
                    coin_name: self.coin_name,
                    parent_block: self.top_parent_hash.map(|x| {
                        messages::nano_sign_tx::ParentBlock {
                            parent_hash: Some(x.to_vec()),
                            link: self.top_parent_link.map(|x| x.to_vec()),
                            representative: self.top_parent_representative,
                            balance: self.top_parent_balance.map(|x| x.to_be_bytes().to_vec()),
                        }
                    }),
                    link_hash: self.link_hash.map(|x| x.to_vec()),
                    link_recipient: self.link_recipient,
                    link_recipient_n: self.link_recipient_n.unwrap_or_default().into(),
                    representative: Some(self.representative),
                    balance: Some(self.balance.to_be_bytes().to_vec()),
                }
                .into(),
            )
        )?;

        println!(
            "Signature:\t{}",
            hex::encode(expect_field!(resp.signature)?)
        );
        println!(
            "Block Hash:\t{}",
            hex::encode(expect_field!(resp.block_hash)?)
        );

        Ok(())
    }
}
