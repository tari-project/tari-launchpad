use derive_more::{Display, From, Into};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum ContainerState {
    Running,
    NotRunning,
    NotFound,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskProgress {
    pub pct: u8,
    pub stage: String,
}

impl TaskProgress {
    pub fn new(stage: &str) -> Self {
        Self {
            pct: 0,
            stage: stage.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Inactive,
    /// Waiting for dependencies.
    Pending,
    Progress(TaskProgress),
    Active,
    // TODO: Add failed with a reason?
}

impl TaskStatus {
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }

    pub fn is_started(&self) -> bool {
        !matches!(self, Self::Inactive)
    }

    pub fn is_inactive(&self) -> bool {
        matches!(self, Self::Inactive)
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Inactive => write!(f, "Inactive"),
            Self::Pending => write!(f, "Pending"),
            Self::Progress(value) => write!(f, "Progress({} - {}%)", value.stage, value.pct),
            Self::Active => write!(f, "Active"),
        }
    }
}

pub trait ManagedTask {
    fn id() -> TaskId;

    fn deps() -> Vec<TaskId> {
        Vec::default()
    }
}

#[derive(
    Debug, Clone, From, Into, PartialOrd, Ord, PartialEq, Eq, Hash, Display, Serialize, Deserialize,
)]
pub struct TaskId(String);

impl From<&str> for TaskId {
    fn from(s: &str) -> Self {
        Self(s.into())
    }
}

impl AsRef<str> for TaskId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

pub trait ManagedContainer: fmt::Debug + Sync + Send + 'static {
    /*
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
    */

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

pub trait ManagedNetwork: fmt::Debug + Sync + Send + 'static {
    fn network_name(&self) -> &str;
}
