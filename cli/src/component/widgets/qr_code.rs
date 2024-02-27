// Copyright 2023. The Tari Project
// SPDX-License-Identifier: BSD-3-Clause

use qrcode::render::unicode;
use qrcode::QrCode;
use ratatui::widgets::Block;
use ratatui::{backend::Backend, layout::Rect, widgets::Paragraph};
use tari_common_types::types::PublicKey;
use tari_launchpad_protocol::settings::TariNetwork;
use tari_launchpad_protocol::wallet::InvalidPublicKey;
use tari_utilities::hex::Hex;
use tari_utilities::ByteArray;

use crate::{
    component::{Component, ComponentEvent, Frame, Input},
    state::AppState,
};

pub struct QrCodePreview {}

impl QrCodePreview {
    pub fn new() -> Self {
        Self {}
    }
}

impl Input for QrCodePreview {
    type Output = ();

    fn on_event(&mut self, _event: ComponentEvent, _state: &mut AppState) -> Option<Self::Output> {
        None
    }
}

impl<B: Backend> Component<B> for QrCodePreview {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let node_status = &state.state.node;
        if let (_, Some(identity)) = (
            state.state.config.session.is_base_node_active(),
            node_status.identity.clone(),
        ) {
            let network = if let Some(settings) = state.state.config.settings.as_ref() {
                settings.saved_settings.tari_network
            } else {
                TariNetwork::default()
            };

            let public_key =
                match PublicKey::from_vec(&identity.public_key).map_err(|e| InvalidPublicKey(e.to_string())) {
                    Ok(public_key) => public_key,
                    Err(e) => {
                        log::warn!("Couldn't convert public key: {}", e);
                        return;
                    },
                };

            let peer = format!("{}::{}", public_key, identity.public_addresses.join("::"));
            let qr_link = format!(
                "tari://{}/base_nodes/add?name={}&peer={}",
                network.lower_case(),
                identity.node_id.to_hex(),
                peer
            );
            if let Ok(code) = QrCode::new(qr_link.clone()) {
                let image = code
                    .render::<unicode::Dense1x2>()
                    .dark_color(unicode::Dense1x2::Dark)
                    .light_color(unicode::Dense1x2::Light)
                    .max_dimensions(40, 20)
                    .module_dimensions(1, 1)
                    .build()
                    .lines()
                    .skip(1)
                    .fold("".to_string(), |acc, l| format!("{}{}\n", acc, l));

                let qr_code = Paragraph::new(image).block(Block::default());

                f.render_widget(qr_code, rect);
            }
        }
    }
}
