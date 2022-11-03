// Copyright 2022 The Tari Project
// SPDX-License-Identifier: BSD-3-Clause

#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]
#[macro_use]
extern crate lazy_static;

use std::thread;

use log::*;
use thiserror::Error;

mod api;
mod commands;
mod docker;
mod error;
mod grpc;
mod rest;

use commands::AppState;
use docker::{shutdown_all_containers, DockerWrapper, DockerWrapperError, Workspaces, DEFAULT_WORKSPACE_NAME};
use tauri::{
    api::cli::get_matches,
    async_runtime::block_on,
    utils::config::CliConfig,
    GlobalWindowEvent,
    Manager,
    Menu,
    MenuItem,
    PackageInfo,
    Submenu,
    WindowEvent,
};
use tauri_plugin_sql::{Migration, MigrationKind, TauriSql};

fn main() {
    match entrypoint() {
        Err(AppError::Docker(err)) => {
            error!("Could not launch docker backend. {}", err.chained_message());
            std::process::exit(-1);
        },
        Err(AppError::PrintHelp(value)) => {
            debug!("{}", value.as_str().unwrap_or("Help is not available"));
            std::process::exit(0);
        },
        Err(_err) => {
            std::process::exit(1);
        },
        _ => {},
    }
}

#[derive(Debug, Error)]
enum AppError {
    #[error("Config is not provided")]
    NoConfig,
    #[error("Tauri runtime failed: {0}")]
    TauriFailed(#[from] tauri::Error),
    #[error("Tauri api error: {0}")]
    TauriApiFailed(#[from] tauri::api::Error),
    #[error("Docker wrapper error: {0}")]
    Docker(#[from] DockerWrapperError),
    #[error("Help: {0:?}")]
    PrintHelp(serde_json::Value),
}

fn entrypoint() -> Result<(), AppError> {
    env_logger::init();
    let context = tauri::generate_context!();
    let cli_config = context.config().tauri.cli.clone().ok_or(AppError::NoConfig)?;

    // We're going to attach this to the AppState because Tauri does not expose it for some reason
    let package_info = context.package_info().clone();
    // Handle --help and --version. Exits after printing
    handle_cli_options(&cli_config, &package_info)?;
    let docker = DockerWrapper::connect()?;
    let docker_cloned = docker.clone();
    thread::spawn(move || block_on(shutdown_all_containers(DEFAULT_WORKSPACE_NAME, &docker_cloned)));

    let menu = create_menus();
    // TODO - Load workspace definitions from persistent storage here
    let workspaces = Workspaces::default();
    info!("Using Docker version: {}", docker.version());
    let migrations = do_migrations();
    tauri::Builder::default()
        .plugin(TauriSql::default().add_migrations("sqlite:launchpad.db", migrations))
        .manage(AppState::new(docker, workspaces, package_info))
        .setup(tari_sdm_launchpad::tauri::bus_setup)
        .menu(menu)
        .invoke_handler(handler())
        .on_window_event(on_event)
        .run(context)?;
    info!("At exit here!");
    Ok(())
}

fn handler() -> impl Fn(tauri::Invoke<tauri::Wry>) + Send + Sync + 'static {
    use api::*;
    use commands::*;
    tauri::generate_handler![
        base_node_sync_progress,
        create_new_workspace,
        create_default_workspace,
        delete_seed_words,
        events,
        get_seed_words,
        health_check,
        image_info,
        network_list,
        pull_image,
        pull_images,
        check_docker,
        launch_docker,
        check_internet_connection,
        open_terminal,
        node_identity,
        clean_docker,
        start_service,
        stop_service,
        shutdown,
        transaction_fee,
        transfer,
        wallet_events,
        wallet_balance,
        wallet_identity,
    ]
}

fn do_migrations() -> Vec<Migration> {
    let migrations = vec![
        Migration {
            version: 1,
            description: "create stats table",
            sql: include_str!("../migrations/2022-06-13.create-stats-table.sql"),
            kind: MigrationKind::Up,
        },
        Migration {
            version: 2,
            description: "create transactions table",
            sql: include_str!("../migrations/2022-06-14.create-transactions-table.sql"),
            kind: MigrationKind::Up,
        },
    ];
    migrations
}

fn create_menus() -> Menu {
    let about_menu = Submenu::new(
        "App",
        Menu::new()
            .add_native_item(MenuItem::Hide)
            .add_native_item(MenuItem::HideOthers)
            .add_native_item(MenuItem::ShowAll)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Quit),
    );

    let edit_menu = Submenu::new(
        "Edit",
        Menu::new()
            .add_native_item(MenuItem::Cut)
            .add_native_item(MenuItem::Copy)
            .add_native_item(MenuItem::Paste)
            .add_native_item(MenuItem::SelectAll),
    );

    let view_menu = Submenu::new("View", Menu::new().add_native_item(MenuItem::EnterFullScreen));

    Menu::new()
        .add_submenu(about_menu)
        .add_submenu(edit_menu)
        .add_submenu(view_menu)
}

fn on_event(evt: GlobalWindowEvent) {
    if let WindowEvent::Destroyed = evt.event() {
        info!("Stopping and destroying all tari containers");
        let docker = evt.window().state::<AppState>().docker.clone();
        let task = thread::spawn(move || block_on(shutdown_all_containers(DEFAULT_WORKSPACE_NAME, &docker)));
        drop(task.join());
    }
}

fn handle_cli_options(cli_config: &CliConfig, pkg_info: &PackageInfo) -> Result<(), AppError> {
    let matches = get_matches(cli_config, pkg_info)?;
    let help = matches.args.get("help");
    let version = matches.args.get("version");
    if let Some(data) = help.or(version) {
        Err(AppError::PrintHelp(data.value.clone()))
    } else {
        Ok(())
    }
}
