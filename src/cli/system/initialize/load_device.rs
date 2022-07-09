use crate::{
    cli::{expect_success, parsers::XprvParser, CliCommand},
    messages,
    transport::ProtocolAdapter,
};
use anyhow::Result;
use bitcoin::util::bip32::ExtendedPrivKey;
use clap::{builder::ArgGroup, ArgAction::SetTrue, Args};

/// Load keys onto the device by copying them from this computer.
///
/// Initializing a device by copying keys from this computer is fast and convenient, but significantly less secure than
/// going through the on-device key generation or recovery procedure. Don't do this unless you really know what you're doing.
///
#[derive(Debug, Clone, Args)]
#[clap(
        group(ArgGroup::new("key-source").required(true)),
    )]
pub struct LoadDevice {
    /// Load device with a BIP39 mnemonic phrase (i.e. recovery sentence).
    #[clap(short, long, group = "key-source")]
    mnemonic: Option<String>,
    /// Load device with a particular BIP32 extended private key, which can be useful in advanced key-management scenarios.
    #[clap(short, long, group = "key-source", value_parser = XprvParser)]
    xprv: Option<ExtendedPrivKey>,
    /// Set the device's PIN.
    #[clap(short, long)]
    pin: Option<u32>,
    /// Enable BIP39 passphrase protection.
    #[clap(short = 'r', long, action = SetTrue)]
    passphrase_protection: Option<bool>,
    #[clap(short = 'g', long)]
    language: Option<String>,
    #[clap(short, long)]
    label: Option<String>,
    #[clap(short, long, action = SetTrue)]
    skip_checksum: Option<bool>,
    #[clap(short, long)]
    u2f_counter: Option<u32>,
}

impl CliCommand for LoadDevice {
    fn handle(self, protocol_adapter: &dyn ProtocolAdapter) -> Result<()> {
        expect_success!(protocol_adapter.send_and_handle(
            messages::LoadDevice {
                mnemonic: self.mnemonic,
                node: self.xprv.map(|x| messages::HdNodeType {
                    depth: x.depth.into(),
                    fingerprint: u32::from_be_bytes(x.parent_fingerprint.into_bytes()),
                    child_num: x.child_number.into(),
                    chain_code: x.chain_code.as_bytes().to_vec(),
                    public_key: None,
                    private_key: Some(x.private_key.secret_bytes().to_vec()),
                }),
                pin: self.pin.map(|x| x.to_string()),
                passphrase_protection: self.passphrase_protection,
                language: self.language,
                label: self.label,
                skip_checksum: self.skip_checksum,
                u2f_counter: self.u2f_counter,
            }
            .into(),
        ))?;

        Ok(())
    }
}
