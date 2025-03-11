use anyhow::Result;
use futures_util::StreamExt;
use parking_lot::RwLock;
use spdlog::{error, info};
use std::sync::{Arc, OnceLock};
use tokio::{
    net::TcpStream,
    select,
    sync::{mpsc, oneshot},
    task::JoinHandle,
};
use tokio_util::codec::Framed;

use crate::service::{
    ServiceControl,
    client::mdns::MdnsClient,
    codec::{DataPacketCodec, DataPacketReader, DataPacketWriter},
};

use super::ServerInfo;

/// 连接状态枚举
#[derive(Debug, Clone)]
pub enum ConnectionState {
    /// 已成功连接
    Connected,
    /// 连接已断开
    Disconnected,
    /// 连接出错
    Error(String),
}

pub struct TcpClient {
    writer: Arc<RwLock<Option<DataPacketWriter>>>,
    service_control: ServiceControl,
    state_tx: mpsc::Sender<ConnectionState>,
}

impl TcpClient {
    pub fn instance() -> &'static Self {
        static INSTANCE: OnceLock<TcpClient> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            let (tx, rx) = mpsc::channel(1);

            // 启动状态处理任务
            tokio::spawn(Self::handle_state_changes(rx));

            TcpClient {
                writer: Arc::new(RwLock::new(None)),
                service_control: ServiceControl::new("Tcp Client".to_string()),
                state_tx: tx,
            }
        })
    }

    pub fn is_running(&self) -> bool {
        self.service_control.is_running()
    }

    // 处理状态变化的后台任务
    async fn handle_state_changes(mut rx: mpsc::Receiver<ConnectionState>) {
        while let Some(state) = rx.recv().await {
            match state {
                ConnectionState::Connected => {
                    info!("State change: Connected");
                    if let Err(e) = MdnsClient::instance().stop().await {
                        error!("Failed to stop mdns after connection: {}", e);
                    }
                }
                ConnectionState::Disconnected => {
                    info!("State change: Disconnected");
                    if let Err(e) = MdnsClient::instance().start().await {
                        error!("Failed to restart after disconnection: {}", e);
                    }
                }
                ConnectionState::Error(e) => {
                    info!("State change: Error - {}", e);
                    if let Err(e) = MdnsClient::instance().start().await {
                        error!("Failed to restart after error: {}", e);
                    }
                }
            }
        }
    }

    pub async fn start(&self, server_info: ServerInfo) -> Result<()> {
        if self.is_running() {
            self.stop().await?;
        }
        let writer = self.writer.clone();
        let server_info = Arc::new(server_info);
        let state_tx = self.state_tx.clone();

        let tcp_start_logic =
            move |rx: oneshot::Receiver<bool>| -> Result<JoinHandle<()>> {
                let server_info = server_info.clone();
                let server_addr =
                    format!("{}:{}", server_info.ip, server_info.tcp_port);

                let task = tokio::spawn(async move {
                    match TcpStream::connect(&server_addr).await {
                        Ok(stream) => {
                            info!("Connected to server: {}", server_addr);
                            let framed =
                                Framed::new(stream, DataPacketCodec::default());
                            let (split_writer, reader) = framed.split();
                            {
                                let mut writer_guard = writer.write();
                                *writer_guard = Some(split_writer);
                            }

                            // 发送连接成功状态
                            if let Err(e) =
                                state_tx.send(ConnectionState::Connected).await
                            {
                                error!("Failed to send connected state: {}", e);
                            }

                            // 开始处理连接
                            let state = Self::handle_connection(
                                reader,
                                rx,
                                &server_info,
                            )
                            .await;

                            // 发送连接状态变化
                            if let Err(e) = state_tx.send(state).await {
                                error!("Failed to send state change: {}", e);
                            }

                            // 清理 writer
                            let mut writer_guard = writer.write();
                            *writer_guard = None;
                        }
                        Err(e) => {
                            error!(
                                "addr:{} Failed to connect to server: {}",
                                server_addr, e
                            );
                            // 发送连接错误状态
                            if let Err(send_err) = state_tx
                                .send(ConnectionState::Error(e.to_string()))
                                .await
                            {
                                error!(
                                    "Failed to send error state: {}",
                                    send_err
                                );
                            }
                        }
                    }
                });

                Ok(task)
            };

        self.service_control.start(tcp_start_logic).await
    }

    pub async fn stop(&self) -> Result<()> {
        // 清理 writer
        {
            let mut writer_guard = self.writer.write();
            *writer_guard = None;
        }
        self.service_control.stop().await
    }

    async fn handle_connection(
        mut reader: DataPacketReader,
        mut rx: oneshot::Receiver<bool>,
        server_info: &ServerInfo,
    ) -> ConnectionState {
        loop {
            select! {
                _ = &mut rx => {
                    info!("Received shutdown signal");
                    return ConnectionState::Disconnected;
                },
                result = reader.next() => {
                    match result {
                        None => {
                            // 连接断开
                            info!("Connection closed for {}", server_info.ip);
                            return ConnectionState::Disconnected;
                        }
                        Some(Ok(_data)) => {
                            // Message received
                            todo!("Not Implemented")
                        }
                        Some(Err(e)) => {
                            // 连接错误
                            let error_msg = format!("Failed to read from connection: {}", e);
                            error!("{}", error_msg);
                            return ConnectionState::Error(error_msg);
                        }
                    }
                }
            }
        }
    }
}
