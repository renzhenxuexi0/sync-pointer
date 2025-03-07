use std::sync::{Arc, OnceLock};

use super::session::SessionContext;
use crate::service::codec::DataPacketCodec;
use crate::service::server::listener::ServerListener;
use crate::{constant, service::ServiceControl};
use anyhow::Result;
use dashmap::DashMap;
use futures_util::StreamExt;
use parking_lot::RwLock;
use spdlog::{error, info};
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
                let addr = format!("0.0.0.0:{}", port);
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

                                    let framed = Framed::new(stream, DataPacketCodec::default());
                                    let (writer, reader) = framed.split();
                                    let listener = ServerListener::new();
                                    if let Err(e) = listener.start(reader).await {
                                        error!("Failed to start listener: {}", e);
                                        drop(writer);
                                        continue;
                                    }
                                    let session_context = SessionContext::new(writer, listener);
                                    Self::instance().sessions().insert(addr.to_string(), session_context);

                            }
                                Err(e) => {
                                    error!("Failed to accept connection: {}", e);
                                }
                            }
                        }
                    }
                }

                drop(listener);
                info!("TCP server stopped");
            });
            Ok(task)
        };

        self.service_control.start(tcp_start_logic).await
    }

    // Stop server
    pub async fn stop(&self) -> Result<()> {
        // 停止所有会话
        for session_key in
            self.sessions().iter().map(|s| s.key().clone()).collect::<Vec<_>>()
        {
            if let Some(mut session) = self.sessions().get_mut(&session_key) {
                let r = session.shutdown().await;
                if r.is_err() {
                    error!(
                        "addr: {} Failed to shutdown session: {}",
                        session_key,
                        r.unwrap_err()
                    );
                }
            }
        }
        self.service_control.stop().await
    }
}
