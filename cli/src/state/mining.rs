use std::time::{Duration, Instant};

use rust_decimal::Decimal;

#[derive(Debug)]
pub struct TariMiningInfo {
    pub mining_started: Option<Instant>,
    pub tari_amount: Decimal,
}

impl TariMiningInfo {
    pub fn is_active(&self) -> bool {
        self.mining_started.is_some()
    }

    pub fn mining_duration(&self) -> Option<Duration> {
        self.mining_started.as_ref().map(Instant::elapsed)
    }

    pub fn toggle(&mut self) {
        if self.mining_started.is_some() {
            self.mining_started = None;
        } else {
            self.mining_started = Some(Instant::now());
        }
    }
}

#[derive(Debug)]
pub struct MergedMiningInfo {
    pub mining_started: Option<Instant>,
    pub tari_amount: Decimal,
    pub monero_amount: Decimal,
}

impl MergedMiningInfo {
    pub fn is_active(&self) -> bool {
        self.mining_started.is_some()
    }

    pub fn mining_duration(&self) -> Option<Duration> {
        self.mining_started.as_ref().map(Instant::elapsed)
    }

    pub fn toggle(&mut self) {
        if self.mining_started.is_some() {
            self.mining_started = None;
        } else {
            self.mining_started = Some(Instant::now());
        }
    }
}
