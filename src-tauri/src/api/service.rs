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
            error!("Failed to start mdns client: {}", e);
            e.to_string()
        })?;
    }

    Ok(())
}

#[tauri::command]
pub async fn stop_service() -> Result<(), String> {
    // 停止所有服务，无论当前是什么模式
    let _ = client::mdns::MdnsClient::instance().stop().await;
    let _ = server::mdns::MdnsServer::instance().stop().await;
    let _ = server::tcp::TcpServer::instance().stop().await;
    let _ = client::tcp::TcpClient::instance().stop().await;

    Ok(())
}

#[tauri::command]
pub async fn handle_service_type_change(
    service_type: String,
) -> Result<(), String> {
    info!("Changing service type to {}", service_type);

    // 根据新类型启动服务
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
            error!("Failed to start mdns client: {}", e);
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
