// Copyright 2022. The Tari Project
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

use crate::docker::ImageType;
use crate::rest::{TagInfo, quay_io::QuayIoRegistry, docker_hub::DockerHubRegistry};
use tari_comms::async_trait;

#[async_trait]
pub trait ServiceRegistry: Send {
    async fn get_tag_info(&self, image: ImageType) -> Result<TagInfo, String>;
}

pub fn get_registry(image: ImageType) -> Box<dyn ServiceRegistry> {
    match image {
        ImageType::Tor |
        ImageType::BaseNode |
        ImageType::Wallet |
        ImageType::XmRig |
        ImageType::Sha3Miner |
        ImageType::MmProxy |
        ImageType::Monerod => Box::new(QuayIoRegistry),
        ImageType::Loki |
        ImageType::Promtail |
        ImageType::Grafana => Box::new(DockerHubRegistry)
    }
}
