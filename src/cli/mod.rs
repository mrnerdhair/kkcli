pub mod cosmos;
pub mod eos;
pub mod ethereum;
pub mod list;
pub mod nano;
pub mod parsers;
pub mod system;
pub mod tendermint;
pub mod thorchain;
pub mod types;
pub mod utxo;

use self::{
    cosmos::{CosmosGetAddress, CosmosSignTx},
    eos::{EosGetPublicKey, EosSignTx},
    ethereum::{EthereumGetAddress, EthereumSignTx},
    list::List,
    nano::{NanoGetAddress, NanoSignTx},
    system::{
        info::{GetEntropy, GetFeatures, GetPublicKey, ListCoins, Ping},
        initialize::{LoadDevice, RecoveryDevice, ResetDevice},
        ApplyPolicy, ChangePin, CipherKeyValue, ClearSession, FirmwareUpdate, SetLabel, WipeDevice,
    },
    tendermint::{TendermintGetAddress, TendermintSignTx},
    thorchain::{ThorchainGetAddress, ThorchainSignTx},
    utxo::{GetAddress, SignMessage, VerifyMessage},
};
use crate::{messages, state_machine::StateMachine};
use anyhow::Result;
use clap::{ArgAction::SetTrue, Parser, Subcommand};

macro_rules! expect_message {
    ($path:path, $target:expr$(,)*) => {
        match $target {
            Ok(x) => match x {
                $path(y) => Ok(y),
                y => Err(::anyhow::anyhow!("unexpected message ({:?})", y)),
            },
            Err(x) => Err(x),
        }
    };
}
pub(crate) use expect_message;

macro_rules! expect_success {
    ($target:expr$(,)*) => {
        crate::cli::expect_message!(crate::messages::Message::Success, $target).map(|x| {
            println!("Success: {}", x.message());
            x
        })
    };
}
pub(crate) use expect_success;

macro_rules! expect_field {
    ($target:ident.$field:ident) => {{
        #[derive(Clone, Copy, Default)]
        struct TypeRef<T>(::core::marker::PhantomData<*const T>);
        impl<T> TypeRef<T> {
            const fn from_ref(_: &T) -> Self {
                Self(::core::marker::PhantomData)
            }
            fn type_name(&self) -> &'static str {
                ::core::any::type_name::<T>()
            }
            fn type_ident(&self) -> &'static str {
                let type_name = self.type_name();
                // using konst here means we're ready when https://github.com/rust-lang/rust/issues/63084 is fixed
                ::konst::option::unwrap_or!(::konst::string::rfind_skip(type_name, "::"), type_name)
            }
        }

        let type_ref = TypeRef::from_ref(&$target);
        $target.$field.as_ref().ok_or_else(|| {
            ::anyhow::anyhow!(
                "expected {} field in {} message",
                "$field",
                type_ref.type_ident(),
            )
        })
    }};
}
pub(crate) use expect_field;

pub trait CliCommand {
    fn handle(self, state_machine: &dyn StateMachine) -> Result<()>;
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
    pub command: Command,
}

#[derive(Subcommand, Debug, Clone)]
#[clap(disable_help_subcommand = true, dont_collapse_args_in_usage = true)]
pub enum Command {
    List(List),
    Ping(Ping),
    GetFeatures(GetFeatures),
    ListCoins(ListCoins),
    SetLabel(SetLabel),
    ChangePin(ChangePin),
    ApplyPolicy(ApplyPolicy),
    GetEntropy(GetEntropy),
    ClearSession(ClearSession),
    WipeDevice(WipeDevice),
    RecoveryDevice(RecoveryDevice),
    LoadDevice(LoadDevice),
    ResetDevice(ResetDevice),
    FirmwareUpdate(FirmwareUpdate),
    CipherKeyValue(CipherKeyValue),
    GetPublicKey(GetPublicKey),
    GetAddress(GetAddress),
    SignMessage(SignMessage),
    VerifyMessage(VerifyMessage),
    EthereumGetAddress(EthereumGetAddress),
    EthereumSignTx(EthereumSignTx),
    EosGetPublicKey(EosGetPublicKey),
    EosSignTx(EosSignTx),
    NanoGetAddress(NanoGetAddress),
    NanoSignTx(NanoSignTx),
    TendermintGetAddress(TendermintGetAddress),
    TendermintSignTx(TendermintSignTx),
    CosmosGetAddress(CosmosGetAddress),
    CosmosSignTx(CosmosSignTx),
    ThorchainGetAddress(ThorchainGetAddress),
    ThorchainSignTx(ThorchainSignTx),
}

impl CliCommand for Cli {
    fn handle(self, state_machine: &dyn StateMachine) -> Result<()> {
        state_machine.send(messages::Initialize::default().into())?;

        match self.command {
            Command::List(cmd) => cmd.handle(state_machine)?,
            Command::GetEntropy(cmd) => cmd.handle(state_machine)?,
            Command::ListCoins(cmd) => cmd.handle(state_machine)?,
            Command::Ping(cmd) => cmd.handle(state_machine)?,
            Command::SetLabel(cmd) => cmd.handle(state_machine)?,
            Command::ChangePin(cmd) => cmd.handle(state_machine)?,
            Command::ApplyPolicy(cmd) => cmd.handle(state_machine)?,
            Command::WipeDevice(cmd) => cmd.handle(state_machine)?,
            Command::ClearSession(cmd) => cmd.handle(state_machine)?,
            Command::RecoveryDevice(cmd) => cmd.handle(state_machine)?,
            Command::FirmwareUpdate(cmd) => cmd.handle(state_machine)?,
            Command::LoadDevice(cmd) => cmd.handle(state_machine)?,
            Command::ResetDevice(cmd) => cmd.handle(state_machine)?,
            Command::CipherKeyValue(cmd) => cmd.handle(state_machine)?,
            Command::GetPublicKey(cmd) => cmd.handle(state_machine)?,
            Command::GetFeatures(cmd) => cmd.handle(state_machine)?,
            Command::GetAddress(cmd) => cmd.handle(state_machine)?,
            Command::SignMessage(cmd) => cmd.handle(state_machine)?,
            Command::VerifyMessage(cmd) => cmd.handle(state_machine)?,
            Command::EthereumGetAddress(cmd) => cmd.handle(state_machine)?,
            Command::EthereumSignTx(cmd) => cmd.handle(state_machine)?,
            Command::EosGetPublicKey(cmd) => cmd.handle(state_machine)?,
            Command::EosSignTx(cmd) => cmd.handle(state_machine)?,
            Command::NanoGetAddress(cmd) => cmd.handle(state_machine)?,
            Command::NanoSignTx(cmd) => cmd.handle(state_machine)?,
            Command::TendermintGetAddress(cmd) => cmd.handle(state_machine)?,
            Command::TendermintSignTx(cmd) => cmd.handle(state_machine)?,
            Command::CosmosGetAddress(cmd) => cmd.handle(state_machine)?,
            Command::CosmosSignTx(cmd) => cmd.handle(state_machine)?,
            Command::ThorchainGetAddress(cmd) => cmd.handle(state_machine)?,
            Command::ThorchainSignTx(cmd) => cmd.handle(state_machine)?,
        }

        Ok(())
    }
}
