use crate::service::ServiceControl;
use crate::service::codec::DataPacketCodec;
use crate::service::module::protocol::DataPacket;
use crate::service::server::session::SessionContext;
use anyhow::Result;
use futures_util::StreamExt;
use futures_util::stream::{SplitSink, SplitStream};
use spdlog::{debug, error, info};
use tokio::io::{self, AsyncReadExt};
use tokio::net::TcpStream;
use tokio::net::tcp::OwnedReadHalf;
use tokio::select;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio_util::codec::Framed;

/// 服务端监听器
pub struct ServerListener {
    addr: String,
    reader: SplitStream<Framed<TcpStream, DataPacketCodec>>,
    service_control: ServiceControl,
}

impl ServerListener {
    pub fn new(
        addr: String,
        reader: SplitStream<Framed<TcpStream, DataPacketCodec>>,
    ) -> Self {
        ServerListener {
            addr,
            reader,
            service_control: ServiceControl::new("ServerListener".to_string()),
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        let mdns_start_logic =
            move |mut rx: oneshot::Receiver<bool>| -> Result<JoinHandle<()>> {
                let task = tokio::spawn(async move {
                    let mut buffer = Vec::new(); // 使用动态缓冲区
                    loop {
                        select! {
                            _ = &mut rx => {
                                info!("Received shutdown signal");
                                break;
                            },
                            result = self.reader.next() => {
                                match result {
                                    None  => {
                                        // Connection closed
                                        info!("Connection closed");
                                        break;
                                    }
                                    Some(Ok(req_resp)) => {
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
                });
                Ok(task)
            };
        self.service_control.start(mdns_start_logic).await?;
        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        self.service_control.stop().await
    }

    async fn handle_message(&mut self, message: &[u8]) -> Result<()> {
        // Process the received message
        info!("Received message: {:?}", message);
        // You can add more logic here to handle different types of messages
        Ok(())
    }
}
