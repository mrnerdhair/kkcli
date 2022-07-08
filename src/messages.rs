// use kkcli_derive::{KKMessage};

pub mod protos {
    include!(concat!(env!("OUT_DIR"), "/_.rs"));
}

pub use protos::*;

use prost::{DecodeError, Message as _Message};
use thiserror::Error;

#[derive(Error, Debug)]
pub struct EncodeError {
    required: usize,
    remaining: usize,
}

impl core::fmt::Display for EncodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "failed to encode message; insufficient buffer capacity (required: {}, remaining: {})",
            self.required, self.remaining
        )
    }
}

impl EncodeError {
    pub const fn new(required: usize, remaining: usize) -> Self {
        Self {
            required,
            remaining,
        }
    }
}

macro_rules! kk_message {
    ($($x:ident),*) => {
        #[derive(Debug, Clone)]
        pub enum Message {
            $($x(protos::$x)),*
        }

        impl ::prost::Message for Message {
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::bytes::buf::BufMut,
                Self: Sized {
                match self {
                    $(Self::$x(x) => x.encode_raw(buf)),*
                }
            }

            fn merge_field<B>(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::prost::encoding::DecodeContext,
            ) -> Result<(), ::prost::DecodeError>
            where
                B: ::bytes::buf::Buf,
                Self: Sized
            {
                match self {
                    $(Self::$x(x) => x.merge_field(tag, wire_type, buf, ctx)),*
                }
            }

            fn encoded_len(&self) -> usize {
                match self {
                    $(Self::$x(x) => x.encoded_len()),*
                }
            }

            fn clear(&mut self) {
                match self {
                    $(Self::$x(x) => x.clear()),*
                }
            }
        }

        impl Message {
            pub const fn message_type(&self) -> protos::MessageType {
                match self {
                    $(Message::$x(_) => protos::MessageType::$x),*
                }
            }
            
            fn decode_as_type<B: bytes::Buf>(buf: &mut B, message_type: protos::MessageType) -> Result<Self, DecodeError> {
                Ok(match message_type {
                    $(protos::MessageType::$x => Message::$x(protos::$x::decode(buf)?)),*
                })
            }
        }

        $(impl From<protos::$x> for Message {
            fn from(x: protos::$x) -> Self {
                Self::$x(x)
            }
        })*

        $(impl TryFrom<Message> for protos::$x {
            type Error = ();
            fn try_from(x: Message) -> Result<Self, Self::Error> {
                match x {
                    Message::$x(y) => Ok(y),
                    _ => Err(())
                }
            }
        })*
    };
}

kk_message!(
    Initialize,
    Ping,
    Success,
    Failure,
    ChangePin,
    WipeDevice,
    FirmwareErase,
    FirmwareUpload,
    GetEntropy,
    Entropy,
    GetPublicKey,
    PublicKey,
    LoadDevice,
    ResetDevice,
    SignTx,
    Features,
    PinMatrixRequest,
    PinMatrixAck,
    Cancel,
    TxRequest,
    TxAck,
    CipherKeyValue,
    ClearSession,
    ApplySettings,
    ButtonRequest,
    ButtonAck,
    GetAddress,
    Address,
    EntropyRequest,
    EntropyAck,
    SignMessage,
    VerifyMessage,
    MessageSignature,
    PassphraseRequest,
    PassphraseAck,
    RecoveryDevice,
    WordRequest,
    WordAck,
    CipheredKeyValue,
    EncryptMessage,
    EncryptedMessage,
    DecryptMessage,
    DecryptedMessage,
    SignIdentity,
    SignedIdentity,
    GetFeatures,
    EthereumGetAddress,
    EthereumAddress,
    EthereumSignTx,
    EthereumTxRequest,
    EthereumTxAck,
    CharacterRequest,
    CharacterAck,
    RawTxAck,
    ApplyPolicies,
    FlashHash,
    FlashWrite,
    FlashHashResponse,
    DebugLinkFlashDump,
    DebugLinkFlashDumpResponse,
    SoftReset,
    DebugLinkDecision,
    DebugLinkGetState,
    DebugLinkState,
    DebugLinkStop,
    DebugLinkLog,
    DebugLinkFillConfig,
    GetCoinTable,
    CoinTable,
    EthereumSignMessage,
    EthereumVerifyMessage,
    EthereumMessageSignature,
    ChangeWipeCode,
    RippleGetAddress,
    RippleAddress,
    RippleSignTx,
    RippleSignedTx,
    ThorchainGetAddress,
    ThorchainAddress,
    ThorchainSignTx,
    ThorchainMsgRequest,
    ThorchainMsgAck,
    ThorchainSignedTx,
    EosGetPublicKey,
    EosPublicKey,
    EosSignTx,
    EosTxActionRequest,
    EosTxActionAck,
    EosSignedTx,
    NanoGetAddress,
    NanoAddress,
    NanoSignTx,
    NanoSignedTx,
    BinanceGetAddress,
    BinanceAddress,
    BinanceGetPublicKey,
    BinancePublicKey,
    BinanceSignTx,
    BinanceTxRequest,
    BinanceTransferMsg,
    BinanceOrderMsg,
    BinanceCancelMsg,
    BinanceSignedTx,
    CosmosGetAddress,
    CosmosAddress,
    CosmosSignTx,
    CosmosMsgRequest,
    CosmosMsgAck,
    CosmosSignedTx,
    CosmosMsgDelegate,
    CosmosMsgUndelegate,
    CosmosMsgRedelegate,
    CosmosMsgRewards,
    CosmosMsgIbcTransfer,
    TendermintGetAddress,
    TendermintAddress,
    TendermintSignTx,
    TendermintMsgRequest,
    TendermintMsgAck,
    TendermintMsgSend,
    TendermintSignedTx,
    TendermintMsgDelegate,
    TendermintMsgUndelegate,
    TendermintMsgRedelegate,
    TendermintMsgRewards,
    TendermintMsgIbcTransfer,
    OsmosisGetAddress,
    OsmosisAddress,
    OsmosisSignTx,
    OsmosisMsgRequest,
    OsmosisMsgAck,
    OsmosisMsgSend,
    OsmosisMsgDelegate,
    OsmosisMsgUndelegate,
    OsmosisMsgRedelegate,
    OsmosisMsgRewards,
    OsmosisMsgLpAdd,
    OsmosisMsgLpRemove,
    OsmosisMsgLpStake,
    OsmosisMsgLpUnstake,
    OsmosisMsgIbcTransfer,
    OsmosisMsgSwap,
    OsmosisSignedTx
);

impl Message {
    pub fn encoded_len(&self) -> usize {
        prost::Message::encoded_len(self) + 8
    }

    pub fn encode<B: bytes::BufMut>(&self, buf: &mut B) -> Result<(), EncodeError> {
        let encoded_len = prost::Message::encoded_len(self);
        let required = 8 + encoded_len;
        let remaining = buf.remaining_mut();
        if remaining < required {
            return Err(EncodeError::new(required, remaining));
        }
        buf.put_u8('#' as u8);
        buf.put_u8('#' as u8);
        buf.put_u16(Into::<i32>::into(self.message_type()).try_into().unwrap());
        buf.put_u32(encoded_len.try_into().unwrap());
        prost::Message::encode(self, buf)
            .map_err(|x| EncodeError::new(x.required_capacity(), x.remaining()))?;
        Ok(())
    }

    pub fn decode<B: bytes::Buf>(buf: &mut B) -> Result<Self, DecodeError> {
        if buf.remaining() < 8 {
            return Err(DecodeError::new("buffer too short"));
        }
        if !(buf.get_u8() == '#' as u8 && buf.get_u8() == '#' as u8) {
            return Err(DecodeError::new("bad magic bytes"));
        }
        let msg_type: i32 = buf.get_u16().into();
        let msg_len: usize = buf.get_u32().try_into().unwrap();
        if buf.remaining() < msg_len {
            return Err(DecodeError::new("buffer too short"));
        }

        Ok(Self::decode_as_type(
            buf,
            protos::MessageType::from_i32(msg_type)
                .ok_or_else(|| DecodeError::new("bad message type"))?,
        )?)
    }
}
