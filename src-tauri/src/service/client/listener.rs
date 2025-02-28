use crate::service::ServiceControl;
use crate::service::codec::{
    DataPacketCodec, DataPacketReader, DataPacketWriter,
};
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

/// 客户端监听器
pub struct ClientListener {
    reader: DataPacketReader,
    service_control: ServiceControl,
}

impl ClientListener {
    pub fn new(reader: DataPacketReader) -> Self {
        ClientListener {
            reader,
            service_control: ServiceControl::new("Client Listener".to_string()),
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        let mdns_start_logic =
            move |mut rx: oneshot::Receiver<bool>| -> Result<JoinHandle<()>> {
                let task = tokio::spawn(async move {
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
