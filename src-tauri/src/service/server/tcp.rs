use std::sync::{Arc, OnceLock};

use anyhow::{Result, anyhow};
use dashmap::DashMap;
use parking_lot::RwLock;
use spdlog::{debug, error, info};
use tauri_plugin_dialog::DialogExt;
use tokio::{net::TcpListener, select, sync::oneshot, task::JoinHandle};

use crate::{
    constant,
    core::handle::Handle,
    service::{module::connection::DeviceInfo, session::SessionContext},
};

// Connection manager
pub struct TcpServer {
    connections: Arc<DashMap<String, SessionContext>>,
    // Server port
    port: Arc<RwLock<u16>>,
    // Running task handle
    running_task: Arc<RwLock<Option<JoinHandle<()>>>>,
    // Shutdown channel
    shutdown_tx: Arc<RwLock<Option<oneshot::Sender<bool>>>>,
}

impl TcpServer {
    pub fn instance() -> &'static Self {
        static INSTANCE: OnceLock<TcpServer> = OnceLock::new();
        INSTANCE.get_or_init(|| TcpServer {
            connections: Arc::new(DashMap::new()),
            port: Arc::new(RwLock::new(constant::DEFAULT_TCP_PORT)),
            running_task: Arc::new(RwLock::new(None)),
            shutdown_tx: Arc::new(RwLock::new(None)),
        })
    }

    fn connections(&self) -> Arc<DashMap<String, SessionContext>> {
        self.connections.clone()
    }

    /// 是否正在运行
    fn is_running(&self) -> bool {
        self.running_task.read().is_some()
    }

    /// 处理连接请求 是否同意连接
    async fn handle_connection_request(
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
    async fn accept_connection(&self, _device_info: DeviceInfo) -> Result<()> {
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

        let (shutdown_tx, mut shutdown_rx) = oneshot::channel::<bool>();
        let mut tx_guard = self.shutdown_tx.write();
        *tx_guard = Some(shutdown_tx);

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
                    _ = &mut shutdown_rx => {
                        info!("TCP server received shutdown signal");
                        break;
                    }
                    accept_result = listener.accept() => {
                        match accept_result {
                            Ok((stream, addr)) => {
                                info!("Received connection request from {}", addr);
                                Self::instance().connections().insert(addr.to_string(), SessionContext::new(Arc::new(stream)));
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

        let mut task_guard = self.running_task.write();
        *task_guard = Some(task);

        Ok(())
    }

    // Stop server
    pub async fn stop(&self) -> Result<()> {
        if !self.is_running() {
            debug!("tcp server not running");
            return Ok(());
        }
        let tx = {
            let mut shutdown_tx = self.shutdown_tx.write();
            shutdown_tx.take()
        };

        let task = {
            let mut running_task = self.running_task.write();
            running_task.take()
        };

        if let Some(tx) = tx {
            tx.send(true).map_err(|e| {
                anyhow::anyhow!("Failed to send shutdown signal: {}", e)
            })?;
        }

        if let Some(task) = task {
            task.await
                .map_err(|e| anyhow::anyhow!("Task join error: {}", e))?;
        }

        {
            let mut shutdown_tx = self.shutdown_tx.write();
            *shutdown_tx = None;
        }
        {
            let mut running_task = self.running_task.write();
            *running_task = None;
        }
        info!("tcp server is stopped");
        Ok(())
    }
}
