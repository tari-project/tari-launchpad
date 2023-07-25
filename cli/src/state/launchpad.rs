use derive_more::From;
use rust_decimal::Decimal;

use super::{
    mining::{MergedMiningInfo, TariMiningInfo},
    onboarding::{Onboarding, OnboardingAction, OnboardingDelta},
};

#[derive(Debug)]
pub struct LaunchpadState {
    pub onboarding: Onboarding,
    pub tari_mining: TariMiningInfo,
    pub merged_mining: MergedMiningInfo,
}

impl LaunchpadState {
    pub fn new() -> Self {
        let tari_mining = TariMiningInfo {
            mining_started: None,
            tari_amount: 123_456.into(),
        };
        let merged_mining = MergedMiningInfo {
            mining_started: None,
            tari_amount: 45_000.into(),
            monero_amount: Decimal::new(35, 1),
        };
        let onboarding = Onboarding::default();
        Self {
            onboarding,
            tari_mining,
            merged_mining,
        }
    }

    pub fn update(&mut self, delta: LaunchpadDelta) {
        match delta {
            LaunchpadDelta::Onboarding(delta) => {
                self.onboarding.update(delta);
            },
        }
    }
}

#[derive(Debug, Clone, From)]
pub enum LaunchpadAction {
    Onboarding(OnboardingAction),
}

impl From<LaunchpadAction> for Option<OnboardingAction> {
    fn from(value: LaunchpadAction) -> Self {
        if let LaunchpadAction::Onboarding(value) = value {
            Some(value)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, From)]
pub enum LaunchpadDelta {
    Onboarding(OnboardingDelta),
}
