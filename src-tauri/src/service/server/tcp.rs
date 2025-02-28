use std::sync::{Arc, OnceLock};

use super::session::SessionContext;
use crate::service::codec::DataPacketCodec;
use crate::service::module::connection;
use crate::service::module::protocol::DataPacket;
use crate::service::server::listener::ServerListener;
use crate::{
    constant,
    core::handle::Handle,
    service::{ServiceControl, module::connection::DeviceInfo},
};
use anyhow::{Result, anyhow};
use dashmap::DashMap;
use futures_util::StreamExt;
use parking_lot::RwLock;
use spdlog::{error, info};
use tauri_plugin_dialog::DialogExt;
use tokio::io::AsyncWriteExt;
use tokio::{net::TcpListener, select, sync::oneshot};
use tokio_util::codec::Framed;

// Connection manager
pub struct TcpServer {
    sessions: Arc<DashMap<String, SessionContext>>,
    // Server port
    port: Arc<RwLock<u16>>,
    service_control: ServiceControl,
}

impl TcpServer {
    pub fn instance() -> &'static Self {
        static INSTANCE: OnceLock<TcpServer> = OnceLock::new();
        INSTANCE.get_or_init(|| TcpServer {
            sessions: Arc::new(DashMap::new()),
            port: Arc::new(RwLock::new(constant::DEFAULT_TCP_PORT)),
            service_control: ServiceControl::new("TCP Server".to_string()),
        })
    }

    fn sessions(&self) -> Arc<DashMap<String, SessionContext>> {
        self.sessions.clone()
    }

    /// 是否正在运行
    fn is_running(&self) -> bool {
        self.service_control.is_running()
    }

    /// 处理连接请求 是否同意连接
    async fn _handle_connection_request(
        device_info: DeviceInfo,
    ) -> Result<bool> {
        let app_handle = Handle::instance()
            .app_handle()
            .ok_or_else(|| anyhow!("Cannot get app handle"))?;

        let message = t!(
            "connection.connect-request-dialog.message",
            "hostname" => device_info.hostname.clone(),
        );

        let result = app_handle
            .dialog()
            .message(message)
            .kind(tauri_plugin_dialog::MessageDialogKind::Info)
            .title(t!("connection.connect-request-dialog.title"))
            .buttons(tauri_plugin_dialog::MessageDialogButtons::OkCancelCustom(
                t!("connection.connect-request-dialog.accept").to_string(),
                t!("connection.connect-request-dialog.reject").to_string(),
            ))
            .blocking_show();
        Ok(result)
    }

    // 同意连接请求
    async fn _accept_connection(&self, _device_info: DeviceInfo) -> Result<()> {
        // TODO 发送tauri事件 前端监听事件进行更新设备信
        Ok(())
    }

    // 更新服务信息
    pub async fn update_server_info(&self, port: Option<u16>) -> Result<()> {
        if let Some(port) = port {
            let mut port_guard = self.port.write();
            *port_guard = port;
            drop(port_guard);
        }
        self.restart_if_running().await
    }

    // 重启服务
    async fn restart_if_running(&self) -> Result<()> {
        if self.is_running() {
            let tcp_server = TcpServer::instance();
            tcp_server.stop().await?;
            tcp_server.start().await?;
        }
        Ok(())
    }

    // 开启服务
    pub async fn start(&self) -> Result<()> {
        if self.is_running() {
            self.stop().await?;
        }

        let port = *self.port.read();

        let tcp_start_logic = move |mut rx: oneshot::Receiver<bool>| {
            let task = tokio::spawn(async move {
                let addr = format!("127.0.0.1:{}", port);
                let listener = match TcpListener::bind(&addr).await {
                    Ok(listener) => listener,
                    Err(e) => {
                        // TODO 待优化
                        error!("Failed to bind to address {}: {}", addr, e);
                        return;
                    }
                };

                info!("TCP server started, listening on: {}", addr);

                loop {
                    select! {
                        _ = &mut rx => {
                            info!("TCP server received shutdown signal");
                            break;
                        }
                        accept_result = listener.accept() => {
                            match accept_result {
                                Ok((stream, addr)) => {
                                    info!("Received connection request from {}", addr);

                                    let framed = Framed::new(stream, DataPacketCodec);
                                    let (writer, reader) = framed.split::<DataPacket>();
                                    let mut listener = ServerListener::new(addr.to_string(), reader);
                                    match listener.run().await {
                                        Ok(()) => {
                                            info!("Start Listener {}", addr);
                                            let session_context = SessionContext::new(writer, listener);
                                            Self::instance().sessions().insert(addr.to_string(), session_context);
                                        }
                                        Err(e) => {
                                            error!("Failed to start Listener {}", addr);
                                            let session = Self::instance().sessions().remove(&addr.to_string());
                                            if let Some((_, mut session)) = session {
                                                match session.shutdown().await{
                                                    Ok(_) => {
                                                        info!("Session shutdown successfully");
                                                    }
                                                    Err(_) => {
                                                        error!("Failed to shutdown session");
                                                    }
                                                };
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to accept connection: {}", e);
                                }
                            }
                        }
                    }
                }

                info!("TCP server stopped");
            });
            Ok(task)
        };

        self.service_control.start(tcp_start_logic).await
    }

    // Stop server
    pub async fn stop(&self) -> Result<()> {
        self.service_control.stop().await
    }
}
