use spdlog::{error, info};
use tauri::{command, Runtime};

use crate::service::{client, server};

#[command]
pub async fn update_mdns_server_info<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    host: Option<String>,
    mdns_port: Option<u16>,
    tcp_port: Option<u16>,
    udp_port: Option<u16>,
) -> Result<(), String> {
    server::mdns::Mdns::instance().update_server_info(host, mdns_port, tcp_port, udp_port);
    Ok(())
}
