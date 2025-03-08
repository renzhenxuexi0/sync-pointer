use anyhow::Result;
use futures_util::{SinkExt as _, StreamExt};
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

use crate::{
    config,
    service::{
        ServiceControl,
        client::mdns::MdnsClient,
        codec::{DataPacketCodec, DataPacketReader, DataPacketWriter},
        protocols::base::{DataPacket, PacketData},
    },
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
    service_control: ServiceControl,
    state_tx: mpsc::Sender<ConnectionState>,
    connection_state: Arc<RwLock<ConnectionState>>,
}

impl TcpClient {
    pub fn instance() -> &'static Self {
        static INSTANCE: OnceLock<TcpClient> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            let (tx, rx) = mpsc::channel(1);

            // 启动状态处理任务
            tokio::spawn(Self::handle_state_changes(rx));

            TcpClient {
                service_control: ServiceControl::new("Tcp Client".to_string()),
                state_tx: tx,
                connection_state: Arc::new(RwLock::new(
                    ConnectionState::Disconnected,
                )),
            }
        })
    }

    pub fn is_running(&self) -> bool {
        self.service_control.is_running()
    }

    fn update_connection_state(&self, state: ConnectionState) {
        let mut connection_state_guard = self.connection_state.write();
        *connection_state_guard = state;
    }

    // 处理状态变化的后台任务
    async fn handle_state_changes(mut rx: mpsc::Receiver<ConnectionState>) {
        while let Some(state) = rx.recv().await {
            match state.clone() {
                ConnectionState::Connected => {
                    Self::instance().update_connection_state(state);
                    info!("State change: Connected");
                    if let Err(e) = MdnsClient::instance().stop().await {
                        error!("Failed to stop mdns after connection: {}", e);
                    }
                }
                ConnectionState::Disconnected => {
                    Self::instance().update_connection_state(state);
                    info!("State change: Disconnected");
                    if let Err(e) = MdnsClient::instance().start().await {
                        error!("Failed to restart after disconnection: {}", e);
                    }
                }
                ConnectionState::Error(e) => {
                    Self::instance().update_connection_state(state);
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
                            let (writer, reader) = framed.split();

                            // 发送连接成功状态
                            if let Err(e) =
                                state_tx.send(ConnectionState::Connected).await
                            {
                                error!("Failed to send connected state: {}", e);
                            }

                            // 阻塞监听消息
                            let state =
                                Self::handle(reader, writer, rx, &server_info)
                                    .await;

                            // 发送连接状态变化
                            if let Err(e) = state_tx.send(state).await {
                                error!("Failed to send state change: {}", e);
                            }
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
        self.service_control.stop().await
    }

    async fn handle(
        mut reader: DataPacketReader,
        mut writer: DataPacketWriter,
        mut rx: oneshot::Receiver<bool>,
        server_info: &ServerInfo,
    ) -> ConnectionState {
        // 创建定时器每30s发送心跳
        let mut interval =
            tokio::time::interval(tokio::time::Duration::from_secs(30));
        loop {
            select! {
                _ = &mut rx => {
                    info!("Received shutdown signal");
                    return ConnectionState::Disconnected;
                },
                _ = interval.tick() => {
                    let system = config::system::config().unwrap();
                    let packet = DataPacket::new(system.id(), PacketData::Ping);
                    if let Err(e) = writer.send(packet).await {
                        let error_msg = format!("Failed to send ping: {}", e);
                        error!("{}", error_msg);
                        return ConnectionState::Error(error_msg);
                    }
                }
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
