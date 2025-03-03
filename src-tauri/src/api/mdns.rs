use spdlog::error;
use tauri::Runtime;

use crate::service::{client, server};

#[tauri::command]
pub async fn start_mdns_client() -> Result<(), String> {
    client::mdns::Mdns::instance().start().await.map_err(|e| {
        error!("Failed to start mdns client: {}", e);
        e.to_string()
    })
}

#[tauri::command]
pub async fn stop_mdns_client() -> Result<(), String> {
    client::mdns::Mdns::instance().stop().await.map_err(|e| {
        error!("Failed to stop mdns client: {}", e);
        e.to_string()
    })
}

#[tauri::command]
pub async fn start_mdns_server() -> Result<(), String> {
    server::mdns::Mdns::instance().start().await.map_err(|e| {
        error!("Failed to start mdns server: {}", e);
        e.to_string()
    })
}

#[tauri::command]
pub async fn stop_mdns_server() -> Result<(), String> {
    server::mdns::Mdns::instance().stop().await.map_err(|e| {
        error!("Failed to stop mdns server: {}", e);
        e.to_string()
    })
}

#[tauri::command]
pub async fn update_mdns_server_info<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    host: Option<String>,
    mdns_port: Option<u16>,
    tcp_port: Option<u16>,
) -> Result<(), String> {
    server::mdns::Mdns::instance()
        .update_server_info(host, mdns_port, tcp_port)
        .await
        .map_err(|e| {
            error!("Failed to update mdns server info: {}", e);
            e.to_string()
        })
}
