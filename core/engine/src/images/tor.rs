use crate::types::{Args, Envs, ManagedContainer, ManagedTask, Networks, Ports, TaskId};

pub static DEFAULT_REGISTRY: &str = "quay.io/tarilabs";
pub static GRAFANA_REGISTRY: &str = "grafana";

#[derive(Debug, Default)]
pub struct LocalNet {}

impl ManagedTask for LocalNet {
    fn id() -> TaskId {
        "LocalNet".into()
    }
}

#[derive(Debug, Default)]
pub struct Tor;

impl ManagedContainer for Tor {
    fn registry(&self) -> &str {
        DEFAULT_REGISTRY
    }

    fn image_name(&self) -> &str {
        "tor"
    }

    fn args(&self, args: &mut Args) {
        args.set_pair("--SocksPort", "0.0.0.0:9050");
        args.set_pair("--ControlPort", "0.0.0.0:9051");
        args.set_pair("--CookieAuthentication", 0);
        args.set_pair("--ClientOnly", 1);
        args.set_pair("--ClientUseIPv6", 1);
        /* TODO: Repair
        if let Some(settings) = self.settings.as_ref() {
            let hashed = EncryptedKey::hash_password(settings.tor_password.deref());
            args.set_pair("--HashedControlPassword", hashed);
        }
        */
        args.flag("--allow-missing-torrc");
    }

    fn envs(&self, envs: &mut Envs) {
        /* TODO: Repair
        if let Some(settings) = self.settings.as_ref() {
            settings.add_common(envs);
        }
        */
    }

    fn ports(&self, ports: &mut Ports) {
        ports.add(9050);
        ports.add(9051);
    }

    fn networks(&self, networks: &mut Networks) {
        networks.add("tor", LocalNet::id());
    }
}
