use crate::{cli::types::Bip32Path, messages};
use anyhow::Result;
use eosio::{Asset, Name, PermissionLevel};
use schemars::{
    schema::{
        ArrayValidation, InstanceType, Schema, SchemaObject, SingleOrVec, StringValidation,
        SubschemaValidation,
    },
    JsonSchema,
};
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, FromInto, PickFirst};
use std::str::FromStr;

struct EmptyArrayDef;
impl JsonSchema for EmptyArrayDef {
    fn schema_name() -> String {
        "EmptyArray".to_string()
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> Schema {
        let mut foo3 = ArrayValidation::default();
        foo3.items = Some(SingleOrVec::Single(Box::from(gen.subschema_for::<()>())));
        foo3.max_items = Some(0);

        let mut foo2 = SchemaObject::default();
        foo2.instance_type = Some(SingleOrVec::Single(Box::from(InstanceType::Array)));
        foo2.array = Some(Box::from(foo3));

        Schema::Object(foo2)
    }
}

struct NameDef;
impl JsonSchema for NameDef {
    fn schema_name() -> String {
        "Name".to_string()
    }
    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> Schema {
        let mut foo3 = StringValidation::default();
        foo3.pattern = Some(r"^([a-z1-5.]{1,11}[a-z1-5])|([a-z1-5.]{12}[a-j1-5])$".to_string());

        let mut foo2 = SchemaObject::default();
        foo2.instance_type = Some(SingleOrVec::Single(Box::from(InstanceType::String)));
        foo2.string = Some(Box::from(foo3));

        Schema::Object(foo2)
    }
}

struct AssetDef;
impl JsonSchema for AssetDef {
    fn schema_name() -> String {
        "Asset".to_string()
    }
    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> Schema {
        let mut foo3 = StringValidation::default();
        foo3.pattern = Some(r"^((\d+\.\d+)|(\d+)) +[A-Z]{1,7}$".to_string());

        let mut foo2 = SchemaObject::default();
        foo2.instance_type = Some(SingleOrVec::Single(Box::from(InstanceType::String)));
        foo2.string = Some(Box::from(foo3));

        Schema::Object(foo2)
    }
}

struct PermissionLevelDef;
impl JsonSchema for PermissionLevelDef {
    fn schema_name() -> String {
        "EitherPermissionLevel".to_string()
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> Schema {
        let mut foo3 = StringValidation::default();
        foo3.pattern = Some(
            r"^([a-z1-5.]{1,11}[a-z1-5]$)|(^[a-z1-5.]{12}[a-j1-5])@([a-z1-5.]{1,11}[a-z1-5]$)|(^[a-z1-5.]{12}[a-j1-5])$".to_string(),
        );

        let mut foo2 = SchemaObject::default();
        foo2.instance_type = Some(SingleOrVec::Single(Box::from(InstanceType::String)));
        foo2.string = Some(Box::from(foo3));

        let mut bar = SubschemaValidation::default();
        bar.one_of = Some(vec![
            Schema::Object(foo2),
            gen.subschema_for::<alternate::PermissionLevel>(),
        ]);
        let mut x = SchemaObject::default();
        x.subschemas = Some(Box::from(bar));
        Schema::Object(x)
    }
}

pub mod alternate {
    use super::NameDef;
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};
    use serde_with::DisplayFromStr;

    #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
    pub struct PermissionLevel {
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "NameDef")]
        pub actor: eosio::Name,
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "NameDef")]
        pub permission: eosio::Name,
    }

    impl From<eosio::PermissionLevel> for PermissionLevel {
        fn from(x: eosio::PermissionLevel) -> Self {
            Self {
                actor: *x.actor,
                permission: *x.permission,
            }
        }
    }

    impl From<PermissionLevel> for eosio::PermissionLevel {
        fn from(x: PermissionLevel) -> Self {
            Self {
                actor: x.actor.into(),
                permission: x.permission.into(),
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Key {
    //TODO: maybe move to KeyData::raw and flatten struct?
    pub r#type: u32,
    #[serde(flatten)]
    pub data: KeyData,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum KeyData {
    Raw {
        #[serde(with = "serde_with::As::<serde_with::hex::Hex>")]
        #[schemars(with = "String", regex(pattern = r"^(0x)?([0-9a-fA-F]{2})*$"))]
        key: Vec<u8>,
    },
    AddressN {
        address_n: Bip32Path,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct KeyWeight {
    pub key: Key,
    pub weight: u32,
}

impl From<&KeyWeight> for messages::EosAuthorizationKey {
    fn from(x: &KeyWeight) -> Self {
        Self {
            r#type: Some(x.key.r#type),
            key: if let &KeyData::Raw { ref key } = &x.key.data {
                Some(key.clone())
            } else {
                None
            },
            weight: Some(x.weight),
            address_n: if let &KeyData::AddressN { ref address_n } = &x.key.data {
                address_n.clone()
            } else {
                Bip32Path::default()
            }
            .into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PermissionLevelWeight {
    #[serde(
        with = "serde_with::As::<PickFirst::<(DisplayFromStr, FromInto<alternate::PermissionLevel>)>>"
    )]
    #[schemars(with = "PermissionLevelDef")]
    pub permission: PermissionLevel,
    pub weight: u32,
}

impl From<&PermissionLevelWeight> for messages::EosAuthorizationAccount {
    fn from(x: &PermissionLevelWeight) -> Self {
        Self {
            account: Some(messages::EosPermissionLevel {
                actor: Some(x.permission.actor.as_u64()),
                permission: Some(x.permission.permission.as_u64()),
            }),
            weight: Some(x.weight),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WaitWeight {
    pub wait_sec: u32,
    pub weight: u32,
}

impl From<&WaitWeight> for messages::EosAuthorizationWait {
    fn from(x: &WaitWeight) -> Self {
        Self {
            wait_sec: Some(x.wait_sec),
            weight: Some(x.weight),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Authority {
    pub accounts: Vec<PermissionLevelWeight>,
    pub keys: Vec<KeyWeight>,
    pub threshold: u32,
    pub waits: Vec<WaitWeight>,
}

impl From<&Authority> for messages::EosAuthorization {
    fn from(x: &Authority) -> Self {
        Self {
            threshold: Some(x.threshold),
            keys: x
                .keys
                .clone()
                .iter()
                .map(|x| Into::<messages::EosAuthorizationKey>::into(x))
                .collect(),
            accounts: x
                .accounts
                .clone()
                .iter()
                .map(|x| Into::<messages::EosAuthorizationAccount>::into(x))
                .collect(),
            waits: x
                .waits
                .clone()
                .iter()
                .map(|x| Into::<messages::EosAuthorizationWait>::into(x))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Transaction {
    #[serde(flatten)]
    pub header: TransactionHeader,
    #[allow(dead_code)]
    #[schemars(with = "EmptyArrayDef")]
    pub context_free_actions: [(); 0],
    pub actions: Vec<Action>,
    #[allow(dead_code)]
    #[schemars(with = "EmptyArrayDef")]
    pub transaction_extensions: [(); 0],
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TransactionHeader {
    #[schemars(
        with = "String",
        regex(
            pattern = r"^([1-9]\d{3})-(0[1-9]|1[012])-(0[1-9]|[12]\d|3[01])[Tt]([01]\d|2[0-3]):([0-5]\d):([0-5]\d|60)(\.\d{0,8}[1-9])?$"
        )
    )]
    pub expiration: chrono::NaiveDateTime,
    pub ref_block_num: u16,
    pub ref_block_prefix: u32,
    pub max_net_usage_words: u32,
    pub max_cpu_usage_ms: u8,
    pub delay_sec: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Action {
    #[serde(with = "serde_with::As::<DisplayFromStr>")]
    #[schemars(with = "NameDef")]
    pub account: Name,
    #[serde(
        with = "serde_with::As::<Vec::<PickFirst::<(DisplayFromStr, FromInto::<alternate::PermissionLevel>)>>>"
    )]
    #[schemars(with = "Vec<PermissionLevelDef>")]
    pub authorization: Vec<PermissionLevel>,
    #[serde(flatten)]
    pub outer: ActionInnerOuter,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ActionInnerOuter {
    Known {
        #[serde(flatten)]
        inner: ActionInner,
    },
    Unknown {
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "NameDef")]
        name: Name,
        #[serde(with = "serde_with::As::<serde_with::hex::Hex>")]
        #[schemars(with = "String", regex(pattern = r"^(0x)?([0-9a-fA-F]{2})*$"))]
        data: Vec<u8>,
    },
}

impl ActionInnerOuter {
    pub fn name(&self) -> Name {
        match self {
            Self::Known { inner } => Name::from_str(inner.name()).unwrap(),
            Self::Unknown { name, .. } => name.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "name", content = "data", rename_all = "lowercase")]
pub enum ActionInner {
    Transfer {
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "NameDef")]
        from: Name,
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "NameDef")]
        to: Name,
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "AssetDef")]
        quantity: Asset,
        memo: String,
    },
    DelegateBw {
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "NameDef")]
        from: Name,
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "NameDef")]
        receiver: Name,
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "AssetDef")]
        //TODO: check if name should be suffixed in _quantity
        stake_net_quantity: Asset,
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "AssetDef")]
        //TODO: check if name should be suffixed in _quantity
        stake_cpu_quantity: Asset,
        transfer: bool,
    },
    UndelegateBw {
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "NameDef")]
        from: Name,
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "NameDef")]
        receiver: Name,
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "AssetDef")]
        //TODO: check if name should be suffixed in _quantity
        unstake_net_quantity: Asset,
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "AssetDef")]
        //TODO: check if name should be suffixed in _quantity
        unstake_cpu_quantity: Asset,
    },
    Refund {
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "NameDef")]
        owner: Name,
    },
    BuyRam {
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "NameDef")]
        payer: Name,
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "NameDef")]
        receiver: Name,
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "AssetDef")]
        quant: Asset,
    },
    BuyRamBytes {
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "NameDef")]
        payer: Name,
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "NameDef")]
        receiver: Name,
        bytes: u32,
    },
    SellRam {
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "NameDef")]
        account: Name,
        bytes: i64,
    },
    VoteProducer {
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "NameDef")]
        voter: Name,
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "NameDef")]
        proxy: Name,
        #[serde(with = "serde_with::As::<Vec::<DisplayFromStr>>")]
        #[schemars(with = "Vec::<NameDef>")]
        producers: Vec<Name>,
    },
    UpdateAuth {
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "NameDef")]
        account: Name,
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "NameDef")]
        permission: Name,
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "NameDef")]
        parent: Name,
        auth: Authority,
    },
    DeleteAuth {
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "NameDef")]
        account: Name,
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "NameDef")]
        permission: Name,
    },
    LinkAuth {
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "NameDef")]
        account: Name,
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "NameDef")]
        code: Name,
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "NameDef")]
        r#type: Name,
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "NameDef")]
        requirement: Name,
    },
    UnlinkAuth {
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "NameDef")]
        account: Name,
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "NameDef")]
        code: Name,
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "NameDef")]
        r#type: Name,
    },
    NewAccount {
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "NameDef")]
        creator: Name,
        #[serde(with = "serde_with::As::<DisplayFromStr>")]
        #[schemars(with = "NameDef")]
        name: Name,
        owner: Authority,
        active: Authority,
    },
}

impl ActionInner {
    pub fn name(&self) -> &str {
        match self {
            Self::Transfer { .. } => "transfer",
            Self::DelegateBw { .. } => "delegatebw",
            Self::UndelegateBw { .. } => "undelegatebw",
            Self::Refund { .. } => "refund",
            Self::BuyRam { .. } => "buyram",
            Self::BuyRamBytes { .. } => "buyrambytes",
            Self::SellRam { .. } => "sellram",
            Self::VoteProducer { .. } => "voteproducer",
            Self::UpdateAuth { .. } => "updateauth",
            Self::DeleteAuth { .. } => "deleteauth",
            Self::LinkAuth { .. } => "linkauth",
            Self::UnlinkAuth { .. } => "unlinkauth",
            Self::NewAccount { .. } => "newaccount",
        }
    }
}

impl Action {
    pub fn as_tx_action_ack(&self) -> Result<messages::EosTxActionAck> {
        let mut out = messages::EosTxActionAck::default();
        out.common = Some(messages::EosActionCommon {
            account: Some(self.account.as_u64()),
            name: Some(self.outer.name().as_u64()),
            authorization: self
                .authorization
                .iter()
                .map(|x| messages::EosPermissionLevel {
                    actor: Some(x.actor.as_u64()),
                    permission: Some(x.permission.as_u64()),
                })
                .collect(),
        });
        match &self.outer {
            &ActionInnerOuter::Known { ref inner } => match inner {
                &ActionInner::BuyRam {
                    ref payer,
                    ref receiver,
                    ref quant,
                } => {
                    out.buy_ram = Some(messages::EosActionBuyRam {
                        payer: Some(payer.as_u64()),
                        receiver: Some(receiver.as_u64()),
                        quantity: Some(messages::EosAsset {
                            amount: Some(quant.amount),
                            symbol: Some(quant.symbol.as_u64()),
                        }),
                    })
                }
                &ActionInner::BuyRamBytes {
                    ref payer,
                    ref receiver,
                    bytes,
                } => {
                    out.buy_ram_bytes = Some(messages::EosActionBuyRamBytes {
                        payer: Some(payer.as_u64()),
                        receiver: Some(receiver.as_u64()),
                        bytes: Some(bytes),
                    })
                }
                &ActionInner::DelegateBw {
                    ref from,
                    ref receiver,
                    ref stake_net_quantity,
                    ref stake_cpu_quantity,
                    transfer,
                } => {
                    out.delegate = Some(messages::EosActionDelegate {
                        sender: Some(from.as_u64()),
                        receiver: Some(receiver.as_u64()),
                        net_quantity: Some(messages::EosAsset {
                            amount: Some(stake_net_quantity.amount),
                            symbol: Some(stake_net_quantity.symbol.as_u64()),
                        }),
                        cpu_quantity: Some(messages::EosAsset {
                            amount: Some(stake_cpu_quantity.amount),
                            symbol: Some(stake_cpu_quantity.symbol.as_u64()),
                        }),
                        transfer: Some(transfer),
                    })
                }
                &ActionInner::DeleteAuth {
                    ref account,
                    ref permission,
                } => {
                    out.delete_auth = Some(messages::EosActionDeleteAuth {
                        account: Some(account.as_u64()),
                        permission: Some(permission.as_u64()),
                    })
                }
                &ActionInner::LinkAuth {
                    ref account,
                    ref code,
                    ref r#type,
                    ref requirement,
                } => {
                    out.link_auth = Some(messages::EosActionLinkAuth {
                        account: Some(account.as_u64()),
                        code: Some(code.as_u64()),
                        r#type: Some(r#type.as_u64()),
                        requirement: Some(requirement.as_u64()),
                    })
                }
                &ActionInner::NewAccount {
                    ref creator,
                    ref name,
                    ref owner,
                    ref active,
                } => {
                    out.new_account = Some(messages::EosActionNewAccount {
                        creator: Some(creator.as_u64()),
                        name: Some(name.as_u64()),
                        owner: Some(owner.into()),
                        active: Some(active.into()),
                    })
                }
                &ActionInner::Refund { ref owner } => {
                    out.refund = Some(messages::EosActionRefund {
                        owner: Some(owner.as_u64()),
                    })
                }
                &ActionInner::SellRam { ref account, bytes } => {
                    out.sell_ram = Some(messages::EosActionSellRam {
                        account: Some(account.as_u64()),
                        bytes: Some(bytes),
                    })
                }
                &ActionInner::Transfer {
                    ref from,
                    ref to,
                    ref quantity,
                    ref memo,
                } => {
                    out.transfer = Some(messages::EosActionTransfer {
                        sender: Some(from.as_u64()),
                        receiver: Some(to.as_u64()),
                        quantity: Some(messages::EosAsset {
                            amount: Some(quantity.amount),
                            symbol: Some(quantity.symbol.as_u64()),
                        }),
                        memo: if memo == "" {
                            None
                        } else {
                            Some(memo.to_owned())
                        },
                    });
                }
                &ActionInner::UndelegateBw {
                    ref from,
                    ref receiver,
                    ref unstake_net_quantity,
                    ref unstake_cpu_quantity,
                } => {
                    out.undelegate = Some(messages::EosActionUndelegate {
                        sender: Some(from.as_u64()),
                        receiver: Some(receiver.as_u64()),
                        net_quantity: Some(messages::EosAsset {
                            amount: Some(unstake_net_quantity.amount),
                            symbol: Some(unstake_net_quantity.symbol.as_u64()),
                        }),
                        cpu_quantity: Some(messages::EosAsset {
                            amount: Some(unstake_cpu_quantity.amount),
                            symbol: Some(unstake_cpu_quantity.symbol.as_u64()),
                        }),
                    })
                }
                &ActionInner::UnlinkAuth {
                    ref account,
                    ref code,
                    ref r#type,
                } => {
                    out.unlink_auth = Some(messages::EosActionUnlinkAuth {
                        account: Some(account.as_u64()),
                        code: Some(code.as_u64()),
                        r#type: Some(r#type.as_u64()),
                    })
                }
                &ActionInner::VoteProducer {
                    ref voter,
                    ref proxy,
                    ref producers,
                } => {
                    out.vote_producer = Some(messages::EosActionVoteProducer {
                        voter: Some(voter.as_u64()),
                        proxy: Some(proxy.as_u64()),
                        producers: producers.iter().map(|x| x.as_u64()).collect(),
                    })
                }
                &ActionInner::UpdateAuth {
                    ref account,
                    ref permission,
                    ref parent,
                    ref auth,
                } => {
                    out.update_auth = Some(messages::EosActionUpdateAuth {
                        account: Some(account.as_u64()),
                        permission: Some(permission.as_u64()),
                        parent: Some(parent.as_u64()),
                        auth: Some(auth.into()),
                    })
                }
            },
            &ActionInnerOuter::Unknown { ref data, .. } => {
                out.unknown = Some(messages::EosActionUnknown {
                    data_size: Some(data.len().try_into()?),
                    data_chunk: Some(data.clone()),
                })
            }
        }
        Ok(out)
    }
}
