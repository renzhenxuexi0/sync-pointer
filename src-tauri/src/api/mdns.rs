use spdlog::error;
use tauri::{command, Runtime};

use crate::service::{client, server};

#[command]
pub async fn start_mdns_server<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
) -> Result<(), String> {
    server::mdns::Mdns::instance().start().await.map_err(|e| {
        error!("Failed to start mDNS server: {}", e);
        e.to_string()
    })?;
    Ok(())
}

#[command]
pub async fn stop_mdns_server<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
) -> Result<(), String> {
    server::mdns::Mdns::instance().stop().await.map_err(|e| {
        error!("Failed to stop mDNS server: {}", e);
        e.to_string()
    })?;
    Ok(())
}

#[command]
pub async fn start_mdns_discovery<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
) -> Result<(), String> {
    client::mdns::Mdns::instance().start().await.map_err(|e| {
        error!("Failed to start mDNS discovery: {}", e);
        e.to_string()
    })?;
    Ok(())
}

#[command]
pub async fn stop_mdns_discovery<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
) -> Result<(), String> {
    client::mdns::Mdns::instance().stop().await.map_err(|e| {
        error!("Failed to stop mDNS discovery: {}", e);
        e.to_string()
    })?;
    Ok(())
}
