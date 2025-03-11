use anyhow::Result;
use mdns_sd::{ServiceDaemon, ServiceEvent};
use spdlog::{debug, error, info};
use std::sync::OnceLock;
use tokio::{select, sync::oneshot, task::JoinHandle};

use crate::{
    config, constant,
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
                    let my_device_id =
                        config::system::config().unwrap_or_default().id();

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
                                    Self::handle_mdns_event(elapsed, event, &my_device_id).await;
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
        my_device_id: &str,
    ) {
        match event {
            ServiceEvent::ServiceResolved(info) => {
                if let Some(device_info) =
                    Self::resolve_device_info(elapsed, info, my_device_id).await
                {
                    // 尝试启动 TCP 连接，不再直接停止 MDNS
                    // TCP 连接成功后会通知 ClientManager，由 ClientManager 决定是否停止 MDNS
                    match client::tcp::TcpClient::instance()
                        .start(device_info)
                        .await
                    {
                        Ok(_) => {
                            info!("TCP client started successfully");
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
        my_device_id: &str,
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

        let ip = addresses.iter().next().map(|ip| ip.to_string())?;
        let tcp_port =
            Self::get_u16(properties, "tcp_port", elapsed, &fullname)?;
        let device_id = Self::get_string(properties, "device_id")?;
        if device_id == my_device_id {
            info!("At {:?}: Skipping own device {}", elapsed, device_id);
            return None;
        }

        Some(ServerInfo { device_id, hostname, ip, tcp_port })
    }

    fn get_u16(
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

    fn get_string(
        properties: &mdns_sd::TxtProperties,
        key: &str,
    ) -> Option<String> {
        properties.get(key).map(|val| val.val_str().to_string())
    }
}
