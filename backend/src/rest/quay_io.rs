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

use crate::docker::ImageType;
use crate::rest::service_registry::ServiceRegistry;
use crate::rest::TagInfo;
use tari_comms::async_trait;

pub const QUAY_IO_URL: &str = "https://quay.io/api/v1/repository";
pub const TARILABS_REPO_NAME: &str = "tarilabs";

#[derive(Serialize, Debug, Clone, Deserialize)]
struct QuayImageTag {
    name: String,
    reversion: bool,
    start_ts: u64,
    end_ts: Option<u64>,
    manifest_digest: String,
    is_manifest_list: bool,
    size: Option<u64>,
    last_modified: String,
    expiration: Option<String>,
}

#[derive(Serialize, Debug, Clone, Deserialize)]
struct QuayTags {
    tags: Vec<QuayImageTag>,
    has_additional: bool,
    page: i32,
}

impl From<QuayImageTag> for TagInfo {
    fn from(source: QuayImageTag) -> Self {
        TagInfo {
            latest: true,
            created_on: source.last_modified,
            digest: source.manifest_digest,
        }
    }
}

pub struct QuayIoRegistry;

#[async_trait]
impl ServiceRegistry for QuayIoRegistry {
    async fn get_tag_info(&self, image: ImageType) -> Result<TagInfo, String> {
        let image_tag = get_image_tags(image).await?;
        let tags = image_tag.tags;
        let mut filtered: Vec<QuayImageTag> = tags
            .iter()
            .filter(|t| t.name.contains("latest") && t.expiration.is_none())
            .cloned()
            .collect();
        if filtered.is_empty() {
            Err("No tags found for tag [latest]".to_string())
        } else {
            if filtered.len() > 1 {
                filtered.sort_by(|t1, t2| t1.start_ts.cmp(&t2.start_ts));
            }
            Ok(TagInfo::from(filtered.pop().unwrap()))
        }
    }
}

fn quay_url(image_name: String, page: Option<i32>) -> String {
    match page {
        Some(p) => format!("{}/{}/tag/?page={}", QUAY_IO_URL, image_name, p),
        None => format!("{}/{}/tag/", QUAY_IO_URL, image_name),
    }
}

async fn get_image_tags(image: ImageType) -> Result<QuayTags, String> {
    if let ImageType::Loki | ImageType::Promtail | ImageType::Grafana = image {
        return Err(format!("image {} is not hosted on quay.io", image));
    }

    let quay_io_url = quay_url(format!("{}/{}", TARILABS_REPO_NAME, image.image_name()), None);
    let tag = reqwest::get(&quay_io_url)
        .await
        .map_err(|_| format!("Can't connect to: {}", &quay_io_url))?
        .json::<QuayTags>()
        .await
        .map_err(|_| format!("Can't read data from: {}", &quay_io_url))?;
    Ok(tag)
}