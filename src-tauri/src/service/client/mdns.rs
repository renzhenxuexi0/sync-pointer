use anyhow::Result;
use mdns_sd::{ServiceDaemon, ServiceEvent};
use parking_lot::RwLock;
use spdlog::{debug, error, info};
use std::sync::{Arc, OnceLock};
use tokio::{select, sync::oneshot, task::JoinHandle};

use crate::{
    constant,
    service::{
        client,
        module::connection::{DeviceInfo, ServiceType},
    },
};

pub struct Mdns {
    running_task: Arc<RwLock<Option<JoinHandle<()>>>>,
    shutdown_tx: Arc<RwLock<Option<oneshot::Sender<bool>>>>,
}

impl Mdns {
    pub fn instance() -> &'static Self {
        static MDNS: OnceLock<Mdns> = OnceLock::new();
        MDNS.get_or_init(|| Mdns {
            running_task: Arc::new(RwLock::new(None)),
            shutdown_tx: Arc::new(RwLock::new(None)),
        })
    }

    fn is_running(&self) -> bool {
        self.running_task.read().is_some()
    }

    pub async fn start(&self) -> Result<()> {
        if self.is_running() {
            self.stop().await?;
        }

        let daemon = ServiceDaemon::new()?;

        let receiver = daemon
            .browse(constant::MDNS_SERVICE_TYPE)
            .map_err(|e| anyhow::anyhow!("Failed to browse: {}", e))?;

        // 信号
        let (send, mut recv) = oneshot::channel();
        let mut shutdown_tx = self.shutdown_tx.write();
        *shutdown_tx = Some(send);
        // 定时器
        let mut interval =
            tokio::time::interval(tokio::time::Duration::from_secs(1));
        // 任务
        let task = tokio::spawn(async move {
            let now = std::time::Instant::now();
            info!("mdns client started");
            loop {
                select! {
                    cmd = &mut recv => {
                        if cmd.is_ok() {
                            info!("Received shutdown signal");
                            break;
                        }
                    }
                    _ = interval.tick() => {
                        let elapsed = now.elapsed();
                        if let Ok(event) = receiver.try_recv() {
                            Self::handle_mdns_event(elapsed, event).await;
                        }
                    }
                }
            }
            if let Err(e) = daemon.shutdown() {
                error!("Error shutting down mdns daemon: {}", e);
            }
            info!("mdns client stopped");
        });

        let mut running_task = self.running_task.write();
        *running_task = Some(task);
        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        if !self.is_running() {
            debug!("mdns client not running");
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
        Ok(())
    }

    async fn handle_mdns_event(
        elapsed: std::time::Duration,
        event: ServiceEvent,
    ) {
        match event {
            ServiceEvent::ServiceResolved(info) => {
                let fullname = info.get_fullname().to_string();
                let hostname = info.get_hostname().to_string();
                let addresses = info.get_addresses();
                let port = info.get_port();
                let properties = info.get_properties();

                if addresses.is_empty() {
                    debug!(
                        "At {:?}: Service {} resolved but no addresses found",
                        elapsed, fullname
                    );
                    return;
                }

                info!(
                    "At {:?}: Resolved service: {} (host: {}, port: {}, addresses: {:?}, properties: {:?})",
                    elapsed, fullname, hostname, port, addresses, properties
                );

                // Get IP address
                let ip = match addresses.iter().next().map(|ip| ip.to_string())
                {
                    Some(ip) => ip,
                    None => {
                        info!(
                            "At {:?}: No address found for service {}",
                            elapsed, fullname
                        );
                        return;
                    }
                };

                // Parse TCP and UDP ports from properties
                let get_port = |key: &str| -> Option<u16> {
                    properties
                        .get(key)
                        .and_then(|val| val.val_str().parse::<u16>().ok())
                };

                let tcp_port = match get_port("tcp_port") {
                    Some(port) => port,
                    None => {
                        info!(
                            "At {:?}: No valid tcp_port for service {}",
                            elapsed, fullname
                        );
                        return;
                    }
                };

                let udp_port = match get_port("udp_port") {
                    Some(port) => port,
                    None => {
                        info!(
                            "At {:?}: No valid udp_port for service {}",
                            elapsed, fullname
                        );
                        return;
                    }
                };

                let device_info = DeviceInfo {
                    hostname,
                    ip,
                    tcp_port,
                    udp_port,
                    service_type: ServiceType::Server,
                };
                match client::tcp::TcpClient::instance()
                    .start(device_info)
                    .await
                {
                    Ok(_) => {
                        // 关闭mdns
                        let mdns = Mdns::instance();
                        mdns.stop().await.ok();
                    }
                    Err(e) => {
                        error!(
                            "At {:?}: Failed to start tcp client: {}",
                            elapsed, e
                        );
                    }
                }
            }
            other_event => {
                debug!("At {:?}: {:?}", elapsed, &other_event);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mdns_singleton() {
        let instance1 = Mdns::instance();
        let instance2 = Mdns::instance();
        assert!(std::ptr::eq(instance1, instance2));
    }

    #[tokio::test]
    async fn test_start_stop() -> Result<()> {
        spdlog::default_logger().set_level_filter(spdlog::LevelFilter::All);
        let mdns = Mdns::instance();
        mdns.start().await?;
        assert!(mdns.running_task.read().is_some());
        assert!(mdns.shutdown_tx.read().is_some());
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        mdns.stop().await?;
        assert!(mdns.running_task.read().is_none());
        assert!(mdns.shutdown_tx.read().is_none());

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        Ok(())
    }
}
