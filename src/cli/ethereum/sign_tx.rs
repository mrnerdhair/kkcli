use crate::{
    cli::{
        expect_field, expect_message,
        parsers::{Bip32PathParser, HexParser, HexParser20, U256Parser},
        types::{Bip32Path, ByteVec},
        types::{IntoBigEndian, OutputAddressType},
        CliCommand,
    },
    messages::{self, Message},
    transport::ProtocolAdapter,
};
use anyhow::Result;
use clap::{builder::ArgGroup, Args};
use core::cmp::min;
use primitive_types::U256;

/// Sign an Ethereum transaction
#[derive(Debug, Clone, Args)]
#[clap(
    group(ArgGroup::new("gas").required(true)),
    group(ArgGroup::new("payload").required(true).multiple(true)),
    group(ArgGroup::new("target").required(true))
)]
pub struct EthereumSignTx {
    /// BIP-32 path to signing key
    #[clap(short = 'n', long, value_parser = Bip32PathParser, default_value = "m/44'/60'/0'/0/0")]
    address: Bip32Path,
    /// EIP-155 chain id (specify 0 to disable replay protection)
    #[clap(short, long, default_value_t = 1)]
    chain_id: u32,
    #[clap(long, value_parser = U256Parser)]
    nonce: U256,
    /// value to transfer, in wei
    #[clap(short, long, value_parser = U256Parser, group = "payload")]
    value: Option<U256>,
    /// transaction data, hex-encoded
    #[clap(
        short,
        long,
        value_parser = HexParser,
        group = "payload",
    )]
    data: Option<ByteVec>,
    /// send transaction to this address
    #[clap(short, long, value_parser = HexParser20, group = "target")]
    to: Option<[u8; 20]>,
    /// send transaction to another address on the same wallet, at this BIP32 path
    #[clap(short = 'p', long, value_parser = Bip32PathParser, group = "target")]
    to_path: Option<Bip32Path>,
    /// transaction gas limit
    #[clap(short = 'l', long, value_parser = U256Parser)]
    gas_limit: U256,
    /// legacy gas price; prefer EIP-1559 if possible
    #[clap(short = 'g', long, value_parser = U256Parser, group = "gas")]
    gas_price: Option<U256>,
    /// EIP-1559 - The maximum total fee per gas, in wei, that the sender is willing to pay.
    #[clap(short = 'f', long, value_parser = U256Parser, group = "gas")]
    max_fee_per_gas: Option<U256>,
    /// EIP-1559 - Maximum fee per gas, in wei, the sender is willing to pay to miners.
    #[clap(short = 'r', long, value_parser = U256Parser, requires("max-fee-per-gas"))]
    max_priority_fee_per_gas: Option<U256>,
}

impl CliCommand for EthereumSignTx {
    fn handle(self, protocol_adapter: &dyn ProtocolAdapter) -> Result<()> {
        let data_length = self.data.as_ref().map(|x| x.len().try_into().unwrap());
        let mut data = self.data.as_ref().map(|x| x.split_at(min(x.len(), 1024)));

        let resp = expect_message!(
            Message::EthereumTxRequest,
            protocol_adapter.send_and_handle_or(
                messages::EthereumSignTx {
                    address_n: self.address.into(),
                    nonce: Some(self.nonce.into_big_endian()),
                    gas_price: self.gas_price.map(|x| x.into_big_endian()),
                    gas_limit: Some(self.gas_limit.into_big_endian()),
                    to: self.to.map(|x| x.to_vec()),
                    value: self.value.map(|x| x.into_big_endian()),
                    max_fee_per_gas: self.max_fee_per_gas.map(|x| x.into_big_endian()),
                    max_priority_fee_per_gas: self
                        .max_priority_fee_per_gas
                        .map(|x| x.into_big_endian()),
                    chain_id: if self.chain_id == 0 {
                        None
                    } else {
                        Some(self.chain_id)
                    },
                    address_type: self
                        .to_path
                        .as_ref()
                        .map_or_else(|| None, |_| Some(OutputAddressType::Transfer as i32)),
                    r#type: self
                        .max_priority_fee_per_gas
                        .map_or_else(|| Some(0), |_| Some(2)),
                    data_length,
                    data_initial_chunk: data.map(|x| x.0.to_owned()),
                    to_address_n: self.to_path.unwrap_or_default().into(),
                    token_value: None,
                    token_to: None,
                    token_shortcut: None,
                    tx_type: None,
                }
                .into(),
                &mut |msg| {
                    Ok(match msg {
                        Message::EthereumTxRequest(messages::EthereumTxRequest {
                            data_length: Some(data_length),
                            ..
                        }) => {
                            data = data.map(|(_, x)| {
                                x.split_at(min(x.len(), (*data_length).try_into().unwrap()))
                            });
                            Some(
                                messages::EthereumTxAck {
                                    data_chunk: data.map(|x| x.0.to_owned()),
                                }
                                .into(),
                            )
                        }
                        _ => None,
                    })
                },
            )
        )?;

        let v = *expect_field!(resp.signature_v)?;
        let r = expect_field!(resp.signature_r)?;
        let s = expect_field!(resp.signature_s)?;

        assert_eq!(data.as_ref().map(|x| x.1.len()).unwrap_or(0), 0);
        let v: u8 = v.try_into()?;
        assert_eq!(r.len(), 32);
        assert_eq!(s.len(), 32);
        println!("{}{}{}", hex::encode(r), hex::encode(s), hex::encode(&[v]));

        Ok(())
    }
}
