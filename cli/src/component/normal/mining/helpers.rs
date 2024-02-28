// Copyright 2023. The Tari Project
// SPDX-License-Identifier: BSD-3-Clause

use ratatui::style::Color;

use crate::{component::normal::mining::status_badge::StatusGetter, state::AppState};

pub struct MergeMiningStatus;

impl StatusGetter for MergeMiningStatus {
    fn get_status(&self, state: &AppState) -> (&str, Color) {
        if state.state.config.session.merge_layer_active {
            ("⚒️  Press [Ctrl-M] to stop Merge Mining", Color::Green)
        } else {
            ("Press [Ctrl-M] to start Merge mining   ", Color::Gray)
        }
    }
}

pub struct ShaMiningStatus;

impl StatusGetter for ShaMiningStatus {
    fn get_status(&self, state: &AppState) -> (&str, Color) {
        if state.state.config.session.sha3x_layer_active {
            ("⚒️  Press [Ctrl-T] to stop SHA3X mining", Color::Yellow)
        } else {
            ("Press [Ctrl-T] to start SHA3X mining   ", Color::Gray)
        }
    }
}
