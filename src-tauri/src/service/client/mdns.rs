use anyhow::Result;
use mdns_sd::{ServiceDaemon, ServiceEvent};
use parking_lot::RwLock;
use spdlog::{debug, error, info};
use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};
use tokio::{select, sync::oneshot, task::JoinHandle};

use crate::{constant, service::Server};

pub struct Mdns {
    pub servers: Arc<RwLock<HashMap<String, Server>>>,
    running_task: Arc<RwLock<Option<JoinHandle<()>>>>,
    shutdown_tx: Arc<RwLock<Option<oneshot::Sender<bool>>>>,
}

impl Mdns {
    pub fn instance() -> &'static Self {
        static MDNS: OnceLock<Mdns> = OnceLock::new();
        MDNS.get_or_init(|| Mdns {
            servers: Arc::new(RwLock::new(HashMap::new())),
            running_task: Arc::new(RwLock::new(None)),
            shutdown_tx: Arc::new(RwLock::new(None)),
        })
    }

    pub fn servers(&self) -> HashMap<String, Server> {
        self.servers.read().clone()
    }

    pub async fn start(&self) -> Result<()> {
        if self.running_task.read().is_some() {
            self.stop().await?;
        }

        let daemon = ServiceDaemon::new()?;

        let receiver = daemon
            .browse(constant::MDNS_SERVICE_TYPE)
            .map_err(|e| anyhow::anyhow!("Failed to browse: {}", e))?;
        let servers = Arc::clone(&self.servers);

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
                            Self::handle_mdns_event(&servers, elapsed, event);
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
        if self.running_task.read().is_none() {
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
            self.servers.write().clear();
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

    fn handle_mdns_event(
        servers: &Arc<RwLock<HashMap<String, Server>>>,
        elapsed: std::time::Duration,
        event: ServiceEvent,
    ) {
        match event {
            ServiceEvent::ServiceResolved(info) => {
                let fullname = info.get_fullname().to_string();
                let hostname = info.get_hostname().to_string();
                let addresses = info.get_addresses();
                let port = info.get_port();

                if addresses.is_empty() {
                    debug!(
                        "At {:?}: Service {} resolved but no addresses found",
                        elapsed, fullname
                    );
                    return;
                }

                info!(
                    "At {:?}: Resolved service: {} (host: {}, port: {})",
                    elapsed, fullname, hostname, port
                );

                for addr in addresses.iter() {
                    debug!("Address: {}", addr);
                }

                for prop in info.get_properties().iter() {
                    debug!("Property: {}", prop);
                }

                let mut servers = servers.write();
                servers.insert(
                    fullname.clone(),
                    Server::new(
                        hostname,
                        addresses.clone(),
                        port,
                        info.get_properties().clone(),
                    ),
                );
                info!("Service {} added to servers map", fullname);
            }
            ServiceEvent::ServiceRemoved(service_type, fullname) => {
                let mut servers = servers.write();
                if servers.remove(&fullname).is_some() {
                    info!(
                        "At {:?}: Removed service: {} of type {}",
                        elapsed, fullname, service_type
                    );
                } else {
                    debug!(
                        "At {:?}: Attempted to remove non-existent service: {} of type {}",
                        elapsed, fullname, service_type
                    );
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
        assert!(mdns.servers.read().is_empty());

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        Ok(())
    }
}
