use crate::{
    cli::{expect_success, CliCommand},
    messages,
    transport::ProtocolAdapter,
};
use anyhow::Result;
use clap::{Args, ArgAction::SetTrue};

/// Send ping message
#[derive(Debug, Clone, Args)]
pub struct Ping {
    #[clap(short, long, action = SetTrue)]
    button_protection: Option<bool>,
    #[clap(short, long, action = SetTrue)]
    pin_protection: Option<bool>,
    #[clap(short = 'r', long, action = SetTrue)]
    passphrase_protection: Option<bool>,
    #[clap(short, long, action = SetTrue)]
    wipe_code_protection: Option<bool>,
    #[clap(short, long)]
    message: Option<String>,
}

impl CliCommand for Ping {
    fn handle(self, protocol_adapter: &dyn ProtocolAdapter) -> Result<()> {
        let resp = expect_success!(protocol_adapter.send_and_handle(
            messages::Ping {
                message: self.message.clone(),
                button_protection: self.button_protection,
                pin_protection: self.pin_protection,
                passphrase_protection: self.passphrase_protection,
                wipe_code_protection: self.wipe_code_protection,
            }
            .into()
        ))?;
        assert_eq!(self.message, resp.message);

        Ok(())
    }
}
