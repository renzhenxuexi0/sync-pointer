use crate::service::client::listener::ClientListener;
use crate::service::codec::{DataPacketCodec, DataPacketWriter};
use crate::service::module::protocol::DataPacket;
use crate::service::{ServiceControl, module::connection::DeviceInfo};
use anyhow::{Result, anyhow};
use futures_util::{SinkExt, StreamExt};
use parking_lot::RwLock;
use spdlog::{debug, error, info};
use std::sync::{Arc, OnceLock};
use tokio::{io, net::TcpStream, select, sync::oneshot, task::JoinHandle};
use tokio_util::codec::Framed;

pub struct TcpClient {
    service_control: ServiceControl,
    writer: Arc<RwLock<Option<DataPacketWriter>>>,
}

impl TcpClient {
    pub fn instance() -> &'static Self {
        static INSTANCE: OnceLock<TcpClient> = OnceLock::new();
        INSTANCE.get_or_init(|| TcpClient {
            service_control: ServiceControl::new("Tcp Client".to_string()),
            writer: Arc::new(RwLock::new(None)),
        })
    }

    pub fn is_running(&self) -> bool {
        self.service_control.is_running()
    }

    pub async fn send(&self, data: DataPacket) -> Result<()> {
        let mut writer_guard = self.writer.write();
        if let Some(mut writer) = writer_guard.take() {
            writer.send(data).await?;
            Ok(())
        } else {
            Err(anyhow!("Writer is not set"))
        }
    }

    pub async fn start(&self, server_info: DeviceInfo) -> Result<()> {
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
                            error!("Failed to connect to server: {}", e);
                            return;
                        }
                    };
                    let framed = Framed::new(stream, DataPacketCodec);
                    let (writer, reader) = framed.split();
                    let writer_guard = self.writer.write();
                    *writer_guard = Some(writer);
                    // 释放 writer_guard 以释放锁
                    drop(writer_guard);
                    let mut listener = ClientListener::new(reader);
                    match listener.run().await {
                        Ok(_) => {
                            // 开始监听
                            info!("Client listener start");
                        }
                        Err(e) => {
                            // 关闭连接
                            error!("Client listener error: {}", e);
                            let mut writer_guard = self.writer.write();
                            if let Some(mut writer) = writer_guard.take() {
                                writer.close().await?;
                            }
                            *writer_guard = None;
                            drop(writer_guard);
                            match listener.shutdown().await {
                                Ok(_) => {
                                    info!("Listener shutdown successfully");
                                }
                                Err(e) => {
                                    error!(
                                        "Failed to shutdown listener: {}",
                                        e
                                    );
                                }
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
}
