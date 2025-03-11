use std::sync::Arc;

use crate::service::ServiceControl;
use crate::service::codec::DataPacketReader;
use anyhow::Result;
use futures_util::StreamExt;
use parking_lot::RwLock;
use spdlog::{error, info};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio::{select, time::Instant};

/// 服务端监听器
pub struct ServerListener {
    last_activity: Arc<RwLock<Instant>>,
    service_control: ServiceControl,
}

impl Default for ServerListener {
    fn default() -> Self {
        Self {
            last_activity: Arc::new(RwLock::new(Instant::now())),
            service_control: ServiceControl::new("Server Listener".to_string()),
        }
    }
}

impl ServerListener {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn start(&self, mut reader: DataPacketReader) -> Result<()> {
        let last_activity = self.last_activity.clone();
        // 创建定时器
        let mut interval =
            tokio::time::interval(tokio::time::Duration::from_secs(1));
        let mdns_start_logic =
            move |mut rx: oneshot::Receiver<bool>| -> Result<JoinHandle<()>> {
                let task = tokio::spawn(async move {
                    loop {
                        select! {
                            _ = &mut rx => {
                                info!("Received shutdown signal");
                                break;
                            },
                            _ = interval.tick() => {
                                // 检查会话是否活跃 如果120s无活动则关闭连接
                                let last_activity = last_activity.read();
                                if last_activity.elapsed().as_secs() > 120 {
                                    info!("No activity for 120s, closing connection, last activity: {:?}", *last_activity);
                                    break;
                                }
                            }
                            result = reader.next() => {
                                match result {
                                    None  => {
                                        // Connection closed
                                        info!("Connection closed");
                                        break;
                                    }
                                    Some(Ok(data)) => {
                                        // Message received
                                        let mut last_activity_guard = last_activity.write();
                                        *last_activity_guard = Instant::now();
                                        info!("Received data: {:?}", data.ts);
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
                    drop(reader);
                    drop(last_activity);
                });
                Ok(task)
            };
        self.service_control.start(mdns_start_logic).await?;
        Ok(())
    }

    pub async fn shutdown(&self) -> Result<()> {
        self.service_control.stop().await?;
        Ok(())
    }
}
