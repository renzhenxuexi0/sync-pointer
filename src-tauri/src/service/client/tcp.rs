use std::sync::{Arc, OnceLock};

use anyhow::Result;
use parking_lot::RwLock;
use spdlog::{debug, error, info};
use tokio::{io, net::TcpStream, select, sync::oneshot, task::JoinHandle};

use crate::service::module::connection::DeviceInfo;

// 连接管理器
pub struct TcpClient {
    // 正在运行的任务
    running_task: Arc<RwLock<Option<JoinHandle<()>>>>,
    // 关闭通道
    shutdown_tx: Arc<RwLock<Option<oneshot::Sender<bool>>>>,
}

impl TcpClient {
    pub fn instance() -> &'static Self {
        static INSTANCE: OnceLock<TcpClient> = OnceLock::new();
        INSTANCE.get_or_init(|| TcpClient {
            running_task: Arc::new(RwLock::new(None)),
            shutdown_tx: Arc::new(RwLock::new(None)),
        })
    }

    /// 是否正在运行
    fn is_running(&self) -> bool {
        self.running_task.read().is_some()
    }

    /// 处理服务端离线
    fn handle_server_offline(&self) -> Result<()> {
        Ok(())
    }

    /// 尝试重新连接
    fn retry_connect(&self) -> Result<()> {
        Ok(())
    }

    // 启动TcpClient
    pub async fn start(&self, server_info: DeviceInfo) -> Result<()> {
        if self.is_running() {
            self.stop().await?;
        }

        let (shutdown_tx, mut shutdown_rx) = oneshot::channel::<bool>();
        let mut shutdown_tx_guard = self.shutdown_tx.write();
        *shutdown_tx_guard = Some(shutdown_tx);

        // 启动连接状态检查任务
        let task = tokio::spawn(async move {
            let stream = match TcpStream::connect(format!(
                "{}:{}",
                server_info.ip, server_info.tcp_port
            ))
            .await
            {
                Ok(stream) => {
                    info!("Connected to server: {}", server_info.ip);
                    stream
                }
                Err(e) => {
                    // TODO 待优化
                    info!("Failed to connect to server: {}", e);
                    return;
                }
            };

            loop {
                select! {
                    cmd = &mut shutdown_rx => {
                        if cmd.is_ok() {
                            info!("Received shutdown signal");
                            break;
                        }
                    }
                    _ = stream.readable() => {
                        let mut buf = vec![0; 1024];
                        match stream.try_read(&mut buf) {
                            Ok(n) => {
                                if n == 0 {
                                    info!("Server closed connection");
                                    break;
                                }
                                info!("Received data: {:?}", &buf[..n]);
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
        });

        let mut task_guard = self.running_task.write();
        *task_guard = Some(task);

        Ok(())
    }

    // 停止TcpClient
    pub async fn stop(&self) -> Result<()> {
        if !self.is_running() {
            debug!("tcp client not running");
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
        info!("tcp client is stopped");
        Ok(())
    }
}
