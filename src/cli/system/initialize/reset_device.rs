use crate::{
    cli::{expect_success, parsers::TypedPossibleValuesParser, CliCommand},
    messages::{self, Message},
    state_machine::StateMachine,
};
use anyhow::Result;
use clap::{ArgAction::SetTrue, Args};
use rand::Rng;

/// Perform device setup and generate new seed
#[derive(Debug, Clone, Args)]
pub struct ResetDevice {
    /// Display entropy generated by the device before asking for additional entropy
    #[clap(short, long, action = SetTrue)]
    display_random: Option<bool>,
    #[clap(short = 't', long, value_parser = TypedPossibleValuesParser::<u32>::new(["128", "192", "256"]))]
    strength: Option<u32>,
    #[clap(short = 'r', long, action = SetTrue)]
    passphrase_protection: Option<bool>,
    #[clap(short, long, action = SetTrue)]
    pin_protection: Option<bool>,
    #[clap(short = 'g', long)]
    language: Option<String>,
    #[clap(short, long)]
    label: Option<String>,
    /// Initialize without ever showing the recovery sentence
    #[clap(short, long, action = SetTrue)]
    no_backup: Option<bool>,
    /// screensaver timeout
    #[clap(short, long)]
    auto_lock_delay_ms: Option<u32>,
    #[clap(short, long)]
    u2f_counter: Option<u32>,
}

impl CliCommand for ResetDevice {
    fn handle(self, state_machine: &dyn StateMachine) -> Result<()> {
        expect_success!(state_machine.send_and_handle_or(
            messages::ResetDevice {
                display_random: self.display_random,
                strength: self.strength,
                passphrase_protection: self.passphrase_protection,
                pin_protection: self.pin_protection,
                language: self.language,
                label: self.label,
                no_backup: self.no_backup,
                auto_lock_delay_ms: self.auto_lock_delay_ms,
                u2f_counter: self.u2f_counter,
            }
            .into(),
            &mut |msg| match msg {
                Message::EntropyRequest(_) => {
                    let mut out = [0; 32];
                    rand::thread_rng().fill(&mut out);
                    Ok(Some(
                        messages::EntropyAck {
                            entropy: Some(out.into()),
                        }
                        .into(),
                    ))
                }
                _ => Ok(None),
            },
        ))?;

        Ok(())
    }
}
