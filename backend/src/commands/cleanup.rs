use std::{convert::TryFrom, fs, path::PathBuf};

use anyhow::Error;
use log::{debug, error};
use serde::{Deserialize, Serialize};
use tauri::{
    api::path::{app_dir, cache_dir},
    AppHandle,
    Config,
    Wry,
};

use crate::{commands::ServiceSettings, docker::LaunchpadConfig};

#[derive(Default, Serialize, Deserialize)]
pub struct CleanupScript {
    paths: Vec<PathBuf>,
}

impl CleanupScript {
    const FILENAME: &'static str = "cleanup.toml";

    fn path(config: &Config) -> Result<PathBuf, Error> {
        let mut buf = app_dir(config).ok_or_else(|| Error::msg("App Dir is not available"))?;
        buf.push(Self::FILENAME);
        Ok(buf)
    }
}

pub fn try_cleanup(config: &Config) -> Result<(), Error> {
    let path = CleanupScript::path(config)?;
    if path.exists() {
        let data = fs::read_to_string(path)?;
        let script: CleanupScript = toml::from_str(&data)?;
        for path in script.paths {
            if let Err(err) = fs::remove_dir_all(&path) {
                error!("Can't remove entry {}: {}", path.display(), err);
            }
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn reset_settings(app: AppHandle<Wry>, settings: ServiceSettings) -> Result<(), String> {
    reset_settings_impl(app, settings).await.map_err(|err| err.to_string())
}

async fn reset_settings_impl(app: AppHandle<Wry>, settings: ServiceSettings) -> Result<(), Error> {
    let app_config = app.config();
    let config = LaunchpadConfig::try_from(settings)?;
    let mut script = CleanupScript::default();

    // Paths to remove (on the next app start)

    // Data root path
    script.paths.push(config.data_directory);

    // App's dir (`~/.config/com.tari.launchpad`)
    if let Some(app_dir) = app_dir(&app_config) {
        script.paths.push(app_dir);
    }

    if let Some(cache_dir) = cache_dir() {
        // Cache tari dir (`~/.cache/tari`)
        let mut tari_cache = cache_dir.clone();
        tari_cache.push("tari");
        script.paths.push(tari_cache);

        // Cache tari dir (`~/.cache/tari`)
        let mut launchpad_cache = cache_dir;
        launchpad_cache.push("tari-launchpad");
        script.paths.push(launchpad_cache);
    }

    let data = toml::to_string(&script)?;
    let path = CleanupScript::path(&app_config)?;
    debug!("Write cleanup script to: {}", path.display());
    fs::write(path, data)?;
    Ok(())
}
