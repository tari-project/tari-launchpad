mod app;
mod dashboard;

use anyhow::Error;
use app::App;
use dashboard::Dashboard;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut app = App::init()?;
    app.routine().await?;
    Ok(())
}

// Create a cryptographically secure password on length `len`
// pub fn create_password(len: usize) -> String {
// use rand::distributions::{Alphanumeric, Distribution};
// let mut rng = rand::thread_rng();
// Alphanumeric.sample_iter(&mut rng).take(len).map(char::from).collect()
// }
