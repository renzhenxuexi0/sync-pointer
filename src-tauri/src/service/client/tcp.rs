use anyhow::Result;
use spdlog::{debug, error, info};
use std::sync::OnceLock;
use tokio::{io, net::TcpStream, select, sync::oneshot, task::JoinHandle};

use crate::service::{ServiceControl, module::connection::DeviceInfo};

pub struct TcpClient {
    service_control: ServiceControl,
}

impl TcpClient {
    pub fn instance() -> &'static Self {
        static INSTANCE: OnceLock<TcpClient> = OnceLock::new();
        INSTANCE.get_or_init(|| TcpClient {
            service_control: ServiceControl::new("Tcp Client".to_string()),
        })
    }

    pub fn is_running(&self) -> bool {
        self.service_control.is_running()
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

                    Self::handle_connection(stream, rx).await;
                });

                Ok(task)
            };

        self.service_control.start(tcp_start_logic).await
    }

    pub async fn stop(&self) -> Result<()> {
        self.service_control.stop().await
    }

    async fn handle_connection(
        stream: TcpStream,
        mut rx: oneshot::Receiver<bool>,
    ) {
        loop {
            select! {
                result = &mut rx => {
                    match result {
                        Ok(_) => info!("Received shutdown signal"),
                        Err(_) => info!("Shutdown channel closed"),
                    }
                    break;
                }
                _ = stream.readable() => {
                    let mut buf = vec![0; 1024];
                    match stream.try_read(&mut buf) {
                        Ok(0) => {
                            info!("Server closed connection");
                            break;
                        }
                        Ok(n) => {
                            debug!("Received {} bytes", n);
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            continue;
                        }
                        Err(e) => {
                            error!("Failed to read from socket: {}", e);
                            break;
                        }
                    }
                }
            }
        }
        info!("Connection handler stopped");
    }
}
