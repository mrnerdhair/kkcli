use crate::{
    cli::types::OutputAddressType,
    messages::{
        self,
        binance_transfer_msg::{BinanceCoin, BinanceInputOutput},
        Message,
    },
};
use anyhow::Result;
use core::num::TryFromIntError;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::DisplayFromStr;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Coin {
    pub amount: u64,
    pub denom: String,
}

impl TryFrom<Coin> for BinanceCoin {
    type Error = TryFromIntError;
    fn try_from(x: Coin) -> Result<Self, Self::Error> {
        Ok(Self {
            amount: Some(x.amount.try_into()?),
            denom: Some(x.denom),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct InputOutput {
    pub address: String,
    pub coins: Vec<Coin>,
}

impl TryFrom<InputOutput> for BinanceInputOutput {
    type Error = TryFromIntError;
    fn try_from(x: InputOutput) -> Result<Self, Self::Error> {
        Ok(Self {
            address: Some(x.address),
            coins: x
                .coins
                .into_iter()
                .map(TryInto::<BinanceCoin>::try_into)
                .collect::<Result<Vec<BinanceCoin>, _>>()?,
            address_type: Some(OutputAddressType::Spend as i32),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum Msg {
    BinanceTransfer {
        inputs: Vec<InputOutput>,
        outputs: Vec<InputOutput>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Transaction {
    #[serde(with = "serde_with::As::<DisplayFromStr>")]
    #[schemars(with = "String", regex(pattern = r"^\d{1,20}$"))]
    pub account_number: u64,
    pub chain_id: String,
    pub memo: String,
    pub msgs: [Msg; 1],
    #[serde(with = "serde_with::As::<DisplayFromStr>")]
    #[schemars(with = "String", regex(pattern = r"^\d{1,20}$"))]
    pub sequence: u64,
    #[serde(with = "serde_with::As::<DisplayFromStr>")]
    #[schemars(with = "String", regex(pattern = r"^\d{1,20}$"))]
    pub source: u64,
}

impl Msg {
    pub fn as_message(&self) -> Result<Message> {
        Ok(match self {
            Self::BinanceTransfer { inputs, outputs } => messages::BinanceTransferMsg {
                inputs: inputs
                    .iter()
                    .map(|x| TryInto::<BinanceInputOutput>::try_into(x.clone()))
                    .collect::<Result<Vec<BinanceInputOutput>, _>>()?,
                outputs: outputs
                    .iter()
                    .map(|x| TryInto::<BinanceInputOutput>::try_into(x.clone()))
                    .collect::<Result<Vec<BinanceInputOutput>, _>>()?,
            }
            .into(),
        })
    }
}
