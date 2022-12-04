pub mod binance;
pub mod cosmos;
pub mod eos;
pub mod ethereum;
pub mod list;
mod macros;
pub mod nano;
pub mod parsers;
pub mod ripple;
pub mod system;
pub mod tendermint;
pub mod thorchain;
pub mod types;
pub mod utxo;

use binance::*;
use cosmos::*;
use eos::*;
use ethereum::*;
use list::*;
pub(crate) use macros::*;
use nano::*;
use ripple::*;
use system::*;
use tendermint::*;
use thorchain::*;
use utxo::*;

use crate::transport::ProtocolAdapter;
use anyhow::Result;
use clap::{ArgAction::SetTrue, Parser};

pub trait CliCommand {
    fn handle(self, protocol_adapter: &mut dyn ProtocolAdapter) -> Result<()>;
}

pub trait CliDebugCommand {
    fn handle_debug(
        self,
        protocol_adapter: &mut dyn ProtocolAdapter,
        debug_protocol_adapter: Option<&mut dyn ProtocolAdapter>,
    ) -> Result<()>;
}

impl<T: CliCommand> CliDebugCommand for T {
    fn handle_debug(
        self,
        protocol_adapter: &mut dyn ProtocolAdapter,
        _: Option<&mut dyn ProtocolAdapter>,
    ) -> Result<()> {
        self.handle(protocol_adapter)
    }
}

/// Command line tool for working with KeepKey devices
#[derive(Parser, Debug, Clone)]
pub struct Cli {
    /// show communication with device
    #[clap(short, long, default_value_t = false, action = SetTrue)]
    pub verbose: bool,
    /// transport used for talking with the device
    /*#[clap(short, long, value_enum, default_value_t = TransportType::Usb)]
    pub transport: TransportType,
    /// path used by the transport (usually serial port)
    #[clap(short, long, requires = "transport")]
    pub path: Option<String>,
    /// DEBUG_LINK transport
    #[clap(long, value_enum, default_value_t = TransportType::Usb)]
    pub debuglink_transport: TransportType,
    /// path used by the DEBUG_LINK transport (usually serial port)
    #[clap(long, requires = "debuglink-transport")]
    pub debuglink_path: Option<String>,
    /// print result as json object
    #[clap(short, long, default_value_t = false)]
    pub json: bool,
    /// enable low-level debugging
    #[clap(short, long, default_value_t = false)]
    pub debug: bool,
    /// automatically press the button during user interaction prompts (on DEBUG_LINK devices only)
    #[clap(short, long, default_value_t = false)]
    pub auto_button: bool,*/
    #[clap(subcommand)]
    pub command: Subcommand,
}

use_cli_subcommands! {
    List,
    Ping,
    GetFeatures,
    ListCoins,
    SetLabel,
    ChangePin,
    ApplyPolicy,
    GetEntropy,
    ClearSession,
    WipeDevice,
    RecoveryDevice,
    LoadDevice,
    ResetDevice,
    FirmwareUpdate,
    CipherKeyValue,
    GetPublicKey,
    GetAddress,
    SignMessage,
    VerifyMessage,
    EthereumGetAddress,
    EthereumSignTx,
    EosGetPublicKey,
    EosSignTx,
    NanoGetAddress,
    NanoSignTx,
    TendermintGetAddress,
    TendermintSignTx,
    CosmosGetAddress,
    CosmosSignTx,
    ThorchainGetAddress,
    ThorchainSignTx,
    EthereumSignMessage,
    EthereumVerifyMessage,
    BinanceGetAddress,
    BinanceSignTx,
    DebugLinkGetState,
    DebugLinkFlashDump,
    DebugLinkFillConfig,
    SignIdentity,
    RippleGetAddress,
    RippleSignTx,
    // SignTx,
    ChangeWipeCode,
    FlashHash,
    FlashWrite,
    SoftReset,
}
