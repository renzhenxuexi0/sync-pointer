use spdlog::{error, info};

use crate::service::{client, server};

#[tauri::command]
pub async fn start_service(service_type: String) -> Result<(), String> {
    info!("Starting service as {}", service_type);

    if service_type == "server" {
        server::mdns::MdnsServer::instance().start().await.map_err(|e| {
            error!("Failed to start mdns server: {}", e);
            e.to_string()
        })?;

        server::tcp::TcpServer::instance().start().await.map_err(|e| {
            error!("Failed to start tcp server: {}", e);
            e.to_string()
        })?;
    } else {
        client::mdns::MdnsClient::instance().start().await.map_err(|e| {
            error!("Failed to start client service: {}", e);
            e.to_string()
        })?;
    }

    Ok(())
}

#[tauri::command]
pub async fn handle_service_type_change(
    service_type: String,
) -> Result<(), String> {
    info!("Changing service type to {}", service_type);

    // 根据新类型启动服务
    if service_type == "server" {
        client::mdns::MdnsClient::instance().stop().await.map_err(|e| {
            error!("Failed to stop client service: {}", e);
            e.to_string()
        })?;
        client::tcp::TcpClient::instance().stop().await.map_err(|e| {
            error!("Failed to stop tcp client: {}", e);
            e.to_string()
        })?;
        server::mdns::MdnsServer::instance().start().await.map_err(|e| {
            error!("Failed to start mdns server: {}", e);
            e.to_string()
        })?;

        server::tcp::TcpServer::instance().start().await.map_err(|e| {
            error!("Failed to start tcp server: {}", e);
            e.to_string()
        })?;
    } else {
        server::mdns::MdnsServer::instance().stop().await.map_err(|e| {
            error!("Failed to stop mdns server: {}", e);
            e.to_string()
        })?;

        server::tcp::TcpServer::instance().stop().await.map_err(|e| {
            error!("Failed to stop tcp server: {}", e);
            e.to_string()
        })?;
        client::mdns::MdnsClient::instance().start().await.map_err(|e| {
            error!("Failed to start client service: {}", e);
            e.to_string()
        })?;
    }

    Ok(())
}

#[tauri::command]
pub async fn update_server_info(
    host: Option<String>,
    mdns_port: Option<u16>,
    tcp_port: Option<u16>,
) -> Result<(), String> {
    server::mdns::MdnsServer::instance()
        .update_server_info(host, mdns_port, tcp_port)
        .await
        .map_err(|e| {
            error!("Failed to update server info: {}", e);
            e.to_string()
        })
}
