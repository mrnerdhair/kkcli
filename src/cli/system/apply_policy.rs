use crate::{
    cli::{expect_success, CliCommand},
    messages,
    state_machine::StateMachine,
};
use anyhow::Result;
use clap::Args;

/// Apply a policy
#[derive(Debug, Clone, Args)]
pub struct ApplyPolicy {
    #[clap(short = 'o', long, default_value = "")]
    policy_name: String,
    #[clap(short = 'c', long, default_value_t = true)]
    enabled: bool,
}

impl CliCommand for ApplyPolicy {
    fn handle(self, state_machine: &dyn StateMachine) -> Result<()> {
        expect_success!(state_machine.send_and_handle(
            messages::ApplyPolicies {
                policy: vec![messages::PolicyType {
                    policy_name: Some(self.policy_name),
                    enabled: Some(self.enabled),
                }],
            }
            .into(),
        ))?;

        Ok(())
    }
}
