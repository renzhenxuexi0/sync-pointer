use spdlog::{error, info};
use tauri::{command, Runtime};

use crate::service::{client, server};

#[command]
pub async fn restart_mdns<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    mode: String,
    host: Option<String>,
    port: Option<u16>,
) -> Result<(), String> {
    info!("Restart mDNS: mode={}, host={:?}, port={:?}", mode, host, port);
    match mode.as_str() {
        "server" => {
            server::mdns::Mdns::instance().stop().await.map_err(|e| {
                error!("Failed to stop mDNS server: {}", e);
                e.to_string()
            })?;
            if let Some(host) = host {
                server::mdns::Mdns::instance().set_host(host);
            }
            if let Some(port) = port {
                server::mdns::Mdns::instance().set_port(port);
            }
            server::mdns::Mdns::instance().start().await.map_err(|e| {
                error!("Failed to start mDNS server: {}", e);
                e.to_string()
            })?;
        }
        "client" => {
            client::mdns::Mdns::instance().stop().await.map_err(|e| {
                error!("Failed to stop mDNS discovery: {}", e);
                e.to_string()
            })?;
            client::mdns::Mdns::instance().start().await.map_err(|e| {
                error!("Failed to start mDNS discovery: {}", e);
                e.to_string()
            })?;
        }
        _ => return Err("Invalid mode. Use 'server' or 'client'".to_string()),
    }
    Ok(())
}
