use anyhow::Result;
use crate::{messages, cli::types::OutputAddressType};
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
pub struct Fee {
    pub amount: [Coin; 1],
    #[serde(with = "serde_with::As::<DisplayFromStr>")]
    #[schemars(with = "String", regex(pattern = r"^\d{1,20}$"))]
    pub gas: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum Msg {
    #[serde(rename = "cosmos-sdk/MsgSend")]
    CosmosSdkMsgSend {
        amount: [Coin; 1],
        from_address: String,
        to_address: String,
    },
    #[serde(rename = "thorchain/MsgSend")]
    ThorchainMsgSend {
        amount: [Coin; 1],
        from_address: String,
        to_address: String,
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

fn bech32_hrp(x: &str) -> Option<&'_ str> {
    Some(x.split_once('1')?.0)
}

impl Msg {
    pub fn type_prefix(&self) -> &str {
        match self {
            &Self::CosmosSdkMsgSend { .. } => "cosmos-sdk",
            &Self::ThorchainMsgSend { .. } => "thorchain",
        }
    }
    pub fn bech32_hrp(&self) -> Option<String> {
        match self {
            &Self::CosmosSdkMsgSend { ref to_address, ref from_address, .. } => {
                let out = bech32_hrp(from_address)?;
                assert_eq!(out, bech32_hrp(to_address)?);
                Some(out.to_string())
            },
            &Self::ThorchainMsgSend { ref to_address, ref from_address, .. } => {
                let out = bech32_hrp(from_address)?;
                assert_eq!(out, bech32_hrp(to_address)?);
                Some(out.to_string())
            },
        }
    }
    pub fn as_message(&self) -> Result<messages::TendermintMsgAck> {
        let mut out = messages::TendermintMsgAck::default();
        out.chain_name = self.bech32_hrp();
        out.message_type_prefix = Some(self.type_prefix().to_string());
        match self {
            Self::CosmosSdkMsgSend {
                amount,
                from_address,
                to_address,
            } => {
                out.denom = Some(amount[0].denom.clone());
                out.send = Some(messages::TendermintMsgSend {
                    from_address: Some(from_address.to_string()),
                    to_address: Some(to_address.to_string()),
                    amount: Some(amount[0].amount),
                    address_type: Some(OutputAddressType::Spend as i32),
                });
            }
            Self::ThorchainMsgSend {
                amount,
                from_address,
                to_address,
            } => {
                out.denom = Some(amount[0].denom.clone());
                out.send = Some(messages::TendermintMsgSend {
                    from_address: Some(from_address.to_string()),
                    to_address: Some(to_address.to_string()),
                    amount: Some(amount[0].amount),
                    address_type: Some(OutputAddressType::Spend as i32),
                });
            }
        };
        Ok(out)
    }
}
