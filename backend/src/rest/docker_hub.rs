// Copyright 2021. The Tari Project
//
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

use serde::{Deserialize, Serialize};

use crate::docker::{ImageType};
use crate::rest::service_registry::ServiceRegistry;
use crate::rest::TagInfo;
use tari_comms::async_trait;

pub const REGISTRY_URL: &str = "https://registry.hub.docker.com/v2/repositories/";
pub const GRAFANA_REPO_NAME: &str = "grafana";

#[derive(Serialize, Debug, Clone, Deserialize)]
struct ImageTag {
    architecture: String,
    features: String,
    variant: Option<String>,
    digest: String,
    os: String,
    os_features: String,
    os_version: Option<String>,
    size: i32,
    status: String,
    last_pulled: String,
    last_pushed: String,
}

#[derive(Serialize, Debug, Clone, Deserialize)]
struct Tag {
    images: Vec<ImageTag>,
    creator: i32,
    id: i32,
    last_updated: String,
    last_updater: i32,
    last_updater_username: String,
    name: String,
    repository: i32,
    full_size: i32,
    v2: bool,
    tag_status: String,
    tag_last_pulled: String,
    tag_last_pushed: String,
    media_type: String,
    digest: String,
}

impl From<Tag> for TagInfo {
    fn from(source: Tag) -> Self {
        TagInfo {
            latest: true,
            created_on: source.last_updated,
            digest: source.digest,
        }
    }
}


pub struct DockerHubRegistry;

#[async_trait]
impl ServiceRegistry for DockerHubRegistry {
    async fn get_tag_info(image: ImageType) -> Result<TagInfo, String> {
        let image_tag = get_tag(image).await?;

        Ok(TagInfo::from(image_tag))
    }
}

async fn get_tag(image: ImageType) -> Result<Tag, String> {
    if let ImageType::Tor | ImageType::BaseNode | ImageType::Wallet | ImageType::XmRig | ImageType::Sha3Miner | ImageType::MmProxy | ImageType::Monerod = image {
        return Err(format!("image {} is not hosted on docker hub", image));
    }

    let url = format!("{}/{}/{}/tags/latest", REGISTRY_URL, GRAFANA_REPO_NAME, image.image_name());

    let tag = reqwest::get(&url)
        .await
        .map_err(|_| format!("Can't connect to: {}", &url))?
        .json::<Tag>()
        .await
        .map_err(|_| format!("Can't read data from: {}", &url))?;
    Ok(tag)
}
