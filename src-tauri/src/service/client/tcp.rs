use anyhow::Result;
use futures_util::StreamExt;
use parking_lot::RwLock;
use spdlog::{error, info};
use std::sync::{Arc, OnceLock};
use tokio::{net::TcpStream, select, sync::oneshot, task::JoinHandle};
use tokio_util::codec::Framed;

use crate::service::{
    ServiceControl,
    codec::{DataPacketCodec, DataPacketReader, DataPacketWriter},
};

use super::ServerInfo;

pub struct TcpClient {
    writer: Arc<RwLock<Option<DataPacketWriter>>>,
    service_control: ServiceControl,
}

impl TcpClient {
    pub fn instance() -> &'static Self {
        static INSTANCE: OnceLock<TcpClient> = OnceLock::new();
        INSTANCE.get_or_init(|| TcpClient {
            writer: Arc::new(RwLock::new(None)),
            service_control: ServiceControl::new("Tcp Client".to_string()),
        })
    }

    pub fn is_running(&self) -> bool {
        self.service_control.is_running()
    }

    pub async fn start(&self, server_info: ServerInfo) -> Result<()> {
        if self.is_running() {
            self.stop().await?;
        }
        let writer = self.writer.clone();
        let tcp_start_logic =
            move |rx: oneshot::Receiver<bool>| -> Result<JoinHandle<()>> {
                let server_addr =
                    format!("{}:{}", server_info.ip, server_info.tcp_port);

                let task = tokio::spawn(async move {
                    let stream = match TcpStream::connect(&server_addr).await {
                        Ok(stream) => {
                            info!("Connected to server: {}", server_addr);
                            stream
                        }
                        Err(e) => {
                            error!("addr:{} Failed to connect to server: {}",server_addr, e);
                            return;
                        }
                    };
                    let framed =
                        Framed::new(stream, DataPacketCodec::default());
                    let (split_writer, reader) = framed.split();
                    {
                        let mut writer_guard = writer.write();
                        *writer_guard = Some(split_writer);
                    }
                    Self::handle_connection(reader, rx).await;
                });

                Ok(task)
            };

        self.service_control.start(tcp_start_logic).await
    }

    pub async fn stop(&self) -> Result<()> {
        self.service_control.stop().await
    }

    async fn handle_connection(
        mut reader: DataPacketReader,
        mut rx: oneshot::Receiver<bool>,
    ) {
        loop {
            select! {
                _ = &mut rx => {
                    info!("Received shutdown signal");
                    break;
                },
                result = reader.next() => {
                    match result {
                        None  => {
                            // Connection closed
                            info!("Connection closed");
                            break;
                        }
                        Some(Ok(_data)) => {
                            // Message received
                            todo!("Not Implemented")
                        }
                        Some(Err(e)) => {
                            // Error occurred
                            error!("Failed to read from connection: {}", e);
                            break;
                        }
                    }
                }
            }
        }
        info!("Connection handler stopped");
    }
}
