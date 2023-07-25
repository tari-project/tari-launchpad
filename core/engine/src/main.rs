use anyhow::Error;
use tact::Actor;
use tari_lp_engine::supervisor::Supervisor;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    let supervisor = Supervisor::new();
    let mut addr = supervisor.start();
    addr.join().await?;
    Ok(())
}
