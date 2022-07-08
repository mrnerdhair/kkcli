use crate::{
    cli::{expect_success, CliCommand},
    messages,
    state_machine::StateMachine,
};
use anyhow::Result;
use clap::{ArgAction::SetTrue, Args};
use sha2::{Digest, Sha256};

/// Upload new firmware to device (must be in bootloader mode)
#[derive(Debug, Clone, Args)]
pub struct FirmwareUpdate {
    /// Don't send the usual firmware erase command before uploading the new firmware.
    #[clap(short, long, action = SetTrue)]
    skip_erase: bool,
    file_path: String,
}

impl CliCommand for FirmwareUpdate {
    fn handle(self, state_machine: &dyn StateMachine) -> Result<()> {
        let payload = std::fs::read(self.file_path)?;

        if !self.skip_erase {
            println!("Erasing firmware...");
            expect_success!(
                state_machine.send_and_handle(messages::FirmwareErase::default().into()),
            )?;
        }

        println!("Uploading firmware...");
        expect_success!(state_machine.send_and_handle(
            messages::FirmwareUpload {
                payload_hash: Sha256::digest(&payload).to_vec(),
                payload,
            }
            .into()
        ),)?;

        Ok(())
    }
}
