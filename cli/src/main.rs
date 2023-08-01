use std::env;

use anyhow::{Context, Error};
use tact::Actor;
use tari_lp_cli::supervisor::Supervisor;
use tari_sdm_assets::configurator::Configurator;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut configurator = Configurator::init()?;
    configurator.clean_configuration().await?;
    configurator.init_configuration().await?;

    let workdir = configurator.base_path();
    env::set_current_dir(workdir)?;

    log4rs::init_file("config/log4rs-cli.yml", Default::default()).context("Can't read a logs configuration file")?;

    let supervisor = Supervisor::default();
    let mut addr = supervisor.start();
    addr.join().await?;
    Ok(())
}
