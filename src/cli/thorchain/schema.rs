use crate::{cli::types::OutputAddressType, messages};
use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::DisplayFromStr;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Coin {
    #[serde(with = "serde_with::As::<DisplayFromStr>")]
    #[schemars(with = "String", regex(pattern = r"^\d{1,20}$"))]
    pub amount: u64,
    pub denom: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ThorchainCoin {
    #[serde(with = "serde_with::As::<DisplayFromStr>")]
    #[schemars(with = "String", regex(pattern = r"^\d{1,20}$"))]
    pub amount: u64,
    pub asset: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Fee {
    pub amount: [Coin; 1],
    #[serde(with = "serde_with::As::<DisplayFromStr>")]
    #[schemars(with = "String", regex(pattern = r"^\d{1,20}$"))]
    pub gas: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum Msg {
    #[serde(rename = "thorchain/MsgSend")]
    ThorchainMsgSend {
        amount: [Coin; 1],
        from_address: String,
        to_address: String,
    },
    #[serde(rename = "thorchain/MsgDeposit")]
    ThorchainMsgDeposit {
        coins: [ThorchainCoin; 1],
        memo: String,
        signer: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Transaction {
    #[serde(with = "serde_with::As::<DisplayFromStr>")]
    #[schemars(with = "String", regex(pattern = r"^\d{1,20}$"))]
    pub account_number: u64,
    pub chain_id: String,
    pub fee: Fee,
    pub memo: String,
    pub msg: [Msg; 1],
    #[serde(with = "serde_with::As::<DisplayFromStr>")]
    #[schemars(with = "String", regex(pattern = r"^\d{1,20}$"))]
    pub sequence: u64,
}

impl Msg {
    pub fn as_message(&self) -> Result<messages::ThorchainMsgAck> {
        let mut out = messages::ThorchainMsgAck::default();
        match self {
            Self::ThorchainMsgSend {
                amount,
                from_address,
                to_address,
            } => {
                out.send = Some(messages::ThorchainMsgSend {
                    from_address: Some(from_address.to_string()),
                    to_address: Some(to_address.to_string()),
                    amount: Some(amount[0].amount),
                    address_type: Some(OutputAddressType::Spend as i32),
                });
            }
            Self::ThorchainMsgDeposit {
                coins,
                memo,
                signer,
            } => {
                out.deposit = Some(messages::ThorchainMsgDeposit {
                    asset: Some(coins[0].asset.to_string()),
                    amount: Some(coins[0].amount),
                    memo: Some(memo.to_string()),
                    signer: Some(signer.to_string()),
                });
            }
        };
        Ok(out)
    }
}
