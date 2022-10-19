use anyhow::Error;
use tari_sdm::SdmScope;
use tari_sdm_launchpad::resources::{
    config::{LaunchpadConfig, WalletConfig},
    files::Configurator,
    images,
    networks,
    volumes,
};
use tokio::signal::ctrl_c;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::try_init()?;
    let mut scope = SdmScope::connect("esmeralda")?;

    scope.add_network(networks::LocalNet::default())?;
    scope.add_volume(volumes::SharedVolume::default())?;
    scope.add_volume(volumes::SharedGrafanaVolume::default())?;
    scope.add_image(images::Tor::default())?;
    scope.add_image(images::TariBaseNode::default())?;
    scope.add_image(images::TariWallet::default())?;
    scope.add_image(images::TariSha3Miner::default())?;
    scope.add_image(images::Loki::default())?;
    scope.add_image(images::Promtail::default())?;
    scope.add_image(images::Grafana::default())?;

    ctrl_c().await?;
    log::info!("Set config");

    let mut configurator = Configurator::init()?;
    let data_directory = configurator.base_path().clone();
    configurator.repair_configuration().await?;
    // let mut config = configurator.read_config().await?;
    let wallet_config = WalletConfig {
        password: "123".to_string().into(),
    };
    let mut config = LaunchpadConfig {
        data_directory,
        with_monitoring: true,
        tor_control_password: "tari".to_string().into(), // create_password(16).into(),
        wallet: Some(wallet_config),
        ..Default::default()
    };
    scope.set_config(Some(config.clone()))?;

    ctrl_c().await?;
    log::info!("Turn off monitoring");
    config.with_monitoring = false;
    scope.set_config(Some(config))?;

    ctrl_c().await?;
    log::info!("Reset config");
    scope.set_config(None)?;

    ctrl_c().await?;
    scope.stop();
    // TODO: Get events from the manager
    Ok(())
}

/// Create a cryptographically secure password on length `len`
pub fn create_password(len: usize) -> String {
    use rand::distributions::{Alphanumeric, Distribution};
    let mut rng = rand::thread_rng();
    Alphanumeric.sample_iter(&mut rng).take(len).map(char::from).collect()
}
