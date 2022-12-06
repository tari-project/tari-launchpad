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

pub mod checker;
mod task;

use std::fmt;

use checker::{ContainerChecker, ReadyIfStarted};
use tari_launchpad_protocol::container::TaskId;
pub(crate) use task::ImageTask;

use crate::config::ManagedProtocol;

pub trait ManagedContainer: fmt::Debug + Send + 'static {
    type Protocol: ManagedProtocol;

    fn checker(&mut self) -> Box<dyn ContainerChecker<Self::Protocol>> {
        Box::new(ReadyIfStarted::default())
    }

    /// Reconfigures the task and return a flag should the container be active?
    fn reconfigure(&mut self, config: Option<&<Self::Protocol as ManagedProtocol>::Config>) -> Option<bool> {
        // Start if config exists
        config.as_ref().map(|_| true)
    }

    fn on_event(&mut self, _event: <Self::Protocol as ManagedProtocol>::Inner) {}

    fn registry(&self) -> &str;

    fn image_name(&self) -> &str;

    fn tag(&self) -> &str {
        "latest"
    }

    fn args(&self, _args: &mut Args) {}

    fn envs(&self, _envs: &mut Envs) {}

    fn ports(&self, _ports: &mut Ports) {}

    fn networks(&self, _networks: &mut Networks) {}

    fn volumes(&self, _volumes: &mut Volumes) {}

    fn mounts(&self, _mounts: &mut Mounts) {}
}

#[derive(Default)]
pub struct Args(Vec<String>);

impl Args {
    pub fn set(&mut self, key: &str, value: impl fmt::Display) {
        let item = format!("{}={}", key, value);
        self.0.push(item);
    }

    pub fn set_pair(&mut self, key: &str, value: impl fmt::Display) {
        self.0.push(key.to_string());
        self.0.push(value.to_string());
    }

    pub fn flag(&mut self, flag: &str) {
        self.0.push(flag.to_string());
    }

    pub fn build(self) -> Vec<String> {
        self.0
    }
}

#[derive(Default)]
pub struct Envs(Vec<String>);

impl Envs {
    pub fn set(&mut self, name: &str, value: impl fmt::Display) {
        let item = format!("{}={}", name, value);
        self.0.push(item);
    }

    pub fn build(self) -> Vec<String> {
        self.0
    }
}

#[derive(Default)]
pub struct Ports(Vec<u16>);

impl Ports {
    pub fn add(&mut self, port: u16) {
        // let item = (port, port);
        self.0.push(port);
    }

    // pub fn forward(&mut self, from: u16, to: u16) {
    // let item = (from, to);
    // self.0.push(item);
    // }

    pub fn build(self) -> Vec<u16> {
        self.0
    }
}

#[derive(Default)]
pub struct Networks(Vec<(String, TaskId)>);

impl Networks {
    pub fn add(&mut self, hostname: impl ToString, id: TaskId) {
        self.0.push((hostname.to_string(), id));
    }

    pub fn build(self) -> Vec<(String, TaskId)> {
        self.0
    }
}

#[derive(Default)]
pub struct Volumes(Vec<String>);

impl Volumes {
    pub fn add(&mut self, volume: impl ToString) {
        self.0.push(volume.to_string());
    }

    pub fn build(self) -> Vec<String> {
        self.0
    }
}

pub enum Mount {
    ToVolume { volume: TaskId, target: String },
    BindTo { source: String, target: String },
}

#[derive(Default)]
pub struct Mounts(Vec<Mount>);

impl Mounts {
    pub fn add_volume(&mut self, volume: TaskId, target: impl ToString) {
        let mount = Mount::ToVolume {
            volume,
            target: target.to_string(),
        };
        self.0.push(mount);
    }

    pub fn bind_path(&mut self, source: impl ToString, target: impl ToString) {
        let mount = Mount::BindTo {
            source: source.to_string(),
            target: target.to_string(),
        };
        self.0.push(mount);
    }

    pub fn build(self) -> Vec<Mount> {
        self.0
    }
}
