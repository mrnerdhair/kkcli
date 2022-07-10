use crate::{
    cli::{expect_message, expect_field, parsers::{Bip32PathParser, HexParser32}, types::Bip32Path, CliCommand},
    messages::{self, Message},
    transport::ProtocolAdapter,
};
use anyhow::Result;
use clap::{builder::ArgGroup, Args};

/*
///
/// Request: Ask device to sign transaction
/// @next TxRequest
/// @next Failure
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignTx {
    /// number of transaction outputs
    #[prost(uint32, required, tag="1")]
    pub outputs_count: u32,
    /// number of transaction inputs
    #[prost(uint32, required, tag="2")]
    pub inputs_count: u32,
    /// coin to use
    #[prost(string, optional, tag="3", default="Bitcoin")]
    pub coin_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="4", default="1")]
    pub version: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="5", default="0")]
    pub lock_time: ::core::option::Option<u32>,


}
///
/// Response: Device asks for information for signing transaction or returns the last result
/// If request_index is set, device awaits TxAck message (with fields filled in according to request_type)
/// If signature_index is set, 'signature' contains signed input of signature_index's input
/// @prev SignTx
/// @prev TxAck
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxRequest {
    /// what should be filled in TxAck message?
    #[prost(enumeration="RequestType", optional, tag="1")]
    pub request_type: ::core::option::Option<i32>,
    /// request for tx details
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<TxRequestDetailsType>,
    /// serialized data and request for next
    #[prost(message, optional, tag="3")]
    pub serialized: ::core::option::Option<TxRequestSerializedType>,
}
///
/// Request: Reported transaction data
/// @prev TxRequest
/// @next TxRequest
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxAck {
    #[prost(message, optional, tag="1")]
    pub tx: ::core::option::Option<TransactionType>,
}
*/

/// Sign UTXO (e.g. Bitcoin) transaction
#[derive(Debug, Clone, Args)]
pub struct SignTx {
    #[clap(short, long)]
    coin_name: Option<String>,
    /// BIP-32 path to source address
    #[clap(short = 'n', long, value_parser = Bip32PathParser, default_value = "m/44'/165'/0'")]
    address: Bip32Path,
    /// transaction version
    #[clap(short, long)]
    tx_version: Option<u32>,
    /// transaction lock_time
    #[clap(short, long)]
    lock_time: Option<u32>,

    inputs: Vec<TxInput>,
    outputs: Vec<TxOutput>,

    /// only for Decred and Zcash
    expiry: ::core::option::Option<u32>,
    /// only for Zcash
    #[clap(long, action = SetTrue)]
    overwintered: Option<bool>,
    /// only for Zcash, nVersionGroupId when overwintered is set
    #[clap(long, requires("overwintered"))]
    version_group_id: Option<u32>,
    /// only for Zcash, BRANCH_ID when overwintered is set
    #[clap(long, requires("overwintered"))]
    branch_id: Option<u32>,
}

impl CliCommand for SignTx {
    fn handle(self, protocol_adapter: &mut dyn ProtocolAdapter) -> Result<()> {
        let resp = expect_message!(
            Message::SignedTx,
            protocol_adapter.with_standard_handler().handle(
                messages::SignTx {
                    inputs_count: self.inputs.len(),
                    outputs_count: self.outputs.len(),
                    version: self.tx_version,
                    address_n: self.address.into(),
                    coin_name: self.coin_name,
                    lock_time: self.lock_time,
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
        
        println!("Signature:\t{}", hex::encode(expect_field!(resp.signature)?));
        println!("Block Hash:\t{}", hex::encode(expect_field!(resp.block_hash)?));

        Ok(())
    }
}
