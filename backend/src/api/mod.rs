// Copyright 2021. The Tari Project
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
mod base_node_api;
mod wallet_api;
use std::convert::TryFrom;

pub use base_node_api::{base_node_sync_progress, node_identity};
use serde::Serialize;
pub use wallet_api::{
    delete_seed_words,
    get_seed_words,
    transaction_fee,
    transfer,
    wallet_balance,
    wallet_events,
    wallet_identity,
};

use crate::{
    commands::{status, ServiceSettings, DEFAULT_IMAGES},
    docker::{ImageType, TariNetwork, TariWorkspace},
    rest::quay_io::get_tag_info,
};

pub static TARI_NETWORKS: [TariNetwork; 4] = [
    TariNetwork::Esmeralda,
    TariNetwork::Dibbler,
    TariNetwork::Igor,
    TariNetwork::Mainnet,
];

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ImageInfo {
    image_name: String,
    container_name: String,
    display_name: String,
    docker_image: String,
    updated: bool,
    created_on: Option<String>,
}

async fn from_image_type(image: ImageType, registry: Option<&str>) -> ImageInfo {
    let mut info = ImageInfo {
        image_name: image.image_name().to_string(),
        container_name: image.container_name().to_string(),
        display_name: image.display_name().to_string(),
        docker_image: TariWorkspace::fully_qualified_image(image, registry),
        ..Default::default()
    };
    if let Ok(tag) = get_tag_info(image).await {
        info.updated = tag.latest;
        info.created_on = Some(tag.created_on);
    }
    info
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageListDto {
    image_info: Vec<ImageInfo>,
    service_recipes: Vec<Vec<String>>,
}

pub fn enum_to_list<T: Sized + ToString + Clone>(enums: &[T]) -> Vec<String> {
    enums.iter().map(|enum_value| enum_value.to_string()).collect()
}

#[tauri::command]
pub fn network_list() -> Vec<String> {
    enum_to_list::<TariNetwork>(&TARI_NETWORKS)
}

/// Provide information about docker images and service recipes in Tari "ecosystem"
#[tauri::command]
pub async fn image_info(settings: ServiceSettings) -> ImageListDto {
    let registry = settings.docker_registry.as_deref();
    let mut images: Vec<ImageInfo> = vec![];
    for image in DEFAULT_IMAGES {
        images.push(from_image_type(image, registry).await);
    }

    let recipes: Vec<Vec<String>> = [
        [ImageType::BaseNode, ImageType::Tor]
            .iter()
            .map(|image_type| image_type.container_name().to_string())
            .collect(),
        [ImageType::Wallet, ImageType::BaseNode, ImageType::Tor]
            .iter()
            .map(|image_type| image_type.container_name().to_string())
            .collect(),
        [
            ImageType::Sha3Miner,
            ImageType::Wallet,
            ImageType::BaseNode,
            ImageType::Tor,
        ]
        .iter()
        .map(|image_type| image_type.container_name().to_string())
        .collect(),
        [
            ImageType::MmProxy,
            ImageType::XmRig,
            ImageType::Wallet,
            ImageType::BaseNode,
            ImageType::Tor,
        ]
        .iter()
        .map(|image_type| image_type.container_name().to_string())
        .collect(),
    ]
    .to_vec();

    ImageListDto {
        image_info: images,
        service_recipes: recipes,
    }
}

#[tauri::command]
pub async fn health_check(image: String) -> String {
    match ImageType::try_from(image.as_str()) {
        Ok(img) => status(img).await,
        Err(_err) => format!("image {} not found", image),
    }
}
