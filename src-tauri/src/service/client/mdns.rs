use anyhow::Result;
use mdns_sd::{ServiceDaemon, ServiceEvent};
use spdlog::{debug, error, info};
use std::sync::OnceLock;
use tokio::{select, sync::oneshot, task::JoinHandle};

use crate::{
    constant,
    service::{ServiceControl, client},
};

use super::ServerInfo;

pub struct MdnsClient {
    service_control: ServiceControl,
}

impl MdnsClient {
    pub fn instance() -> &'static Self {
        static MDNS: OnceLock<MdnsClient> = OnceLock::new();
        MDNS.get_or_init(|| MdnsClient {
            service_control: ServiceControl::new("Mdns Client".to_string()),
        })
    }

    pub fn is_running(&self) -> bool {
        self.service_control.is_running()
    }

    pub async fn start(&self) -> Result<()> {
        let mdns_start_logic =
            |mut rx: oneshot::Receiver<bool>| -> Result<JoinHandle<()>> {
                let daemon = ServiceDaemon::new()?;
                let receiver = daemon
                    .browse(constant::MDNS_SERVICE_TYPE)
                    .map_err(|e| anyhow::anyhow!("Failed to browse: {}", e))?;

                let task = tokio::spawn(async move {
                    let now = std::time::Instant::now();
                    info!("mdns client started");
                    let mut interval = tokio::time::interval(
                        tokio::time::Duration::from_secs(1),
                    );

                    loop {
                        select! {
                            result = &mut rx => {
                                match result {
                                    Ok(_) => info!("Received shutdown signal"),
                                    Err(_) => info!("Shutdown channel closed"),
                                }
                                break;
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
                Ok(task)
            };

        self.service_control.start(mdns_start_logic).await
    }

    pub async fn stop(&self) -> Result<()> {
        self.service_control.stop().await
    }

    async fn handle_mdns_event(
        elapsed: std::time::Duration,
        event: ServiceEvent,
    ) {
        match event {
            ServiceEvent::ServiceResolved(info) => {
                if let Some(device_info) =
                    Self::resolve_device_info(elapsed, info).await
                {
                    match client::tcp::TcpClient::instance()
                        .start(device_info)
                        .await
                    {
                        Ok(_) => {
                            // 关闭mdns
                            let mdns = MdnsClient::instance();
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
            }
            other_event => {
                debug!("At {:?}: {:?}", elapsed, &other_event);
            }
        }
    }

    async fn resolve_device_info(
        elapsed: std::time::Duration,
        info: mdns_sd::ServiceInfo,
    ) -> Option<ServerInfo> {
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
            return None;
        }

        info!(
            "At {:?}: Resolved service: {} (host: {}, port: {}, addresses: {:?}, properties: {:?})",
            elapsed, fullname, hostname, port, addresses, properties
        );

        let ip = addresses.iter().next().map(|ip| ip.to_string())?; // 使用 `?` 传播错误
        let tcp_port =
            Self::get_port(properties, "tcp_port", elapsed, &fullname)?;

        Some(ServerInfo { hostname, ip, tcp_port })
    }

    fn get_port(
        properties: &mdns_sd::TxtProperties,
        key: &str,
        elapsed: std::time::Duration,
        fullname: &str,
    ) -> Option<u16> {
        properties
            .get(key)
            .and_then(|val| val.val_str().parse::<u16>().ok())
            .or_else(|| {
                info!(
                    "At {:?}: No valid {} for service {}",
                    elapsed, key, fullname
                );
                None
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mdns_singleton() {
        let instance1 = MdnsClient::instance();
        let instance2 = MdnsClient::instance();
        assert!(std::ptr::eq(instance1, instance2));
    }

    #[tokio::test]
    async fn test_start_stop() -> Result<()> {
        spdlog::default_logger().set_level_filter(spdlog::LevelFilter::All);
        let mdns = MdnsClient::instance();
        mdns.start().await?;
        assert!(mdns.is_running());

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        mdns.stop().await?;
        assert!(!mdns.is_running());

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        Ok(())
    }
}
