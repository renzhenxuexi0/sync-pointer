use anyhow::Result;
use std::sync::OnceLock;
use tokio::{sync::oneshot, task::JoinHandle};

use crate::{
    config::{self},
    service::ServiceControl,
};

#[derive(Debug)]
pub struct MdnsServer {
    service_control: ServiceControl,
}

impl MdnsServer {
    pub fn instance() -> &'static Self {
        static MDNS: OnceLock<MdnsServer> = OnceLock::new();
        MDNS.get_or_init(|| MdnsServer {
            service_control: ServiceControl::new("Mdns Server".to_string()),
        })
    }

    pub async fn start(&self) -> Result<()> {
        // Clone the values we need from self to avoid capturing self in the closure
        let network = config::network::get_config();
        let system = config::system::config().unwrap_or_default();
        let hostname = network.hostname() + ".local.";
        let tcp_port = network.tcp_port();
        let mdns_port = network.mdns_port();
        let device_id = system.id();

        let mdns_start_logic =
            move |rx: oneshot::Receiver<bool>| -> Result<JoinHandle<()>> {
                let daemon = mdns_sd::ServiceDaemon::new()?;
                let mut properties = std::collections::HashMap::new();

                properties.insert("tcp_port".to_string(), tcp_port.to_string());
                properties.insert("device_id".to_string(), device_id);
                let service_info = mdns_sd::ServiceInfo::new(
                    crate::constant::MDNS_SERVICE_TYPE,
                    crate::constant::MDNS_SERVER_NAME,
                    &hostname,
                    local_ip_address::local_ip()?,
                    mdns_port,
                    properties,
                )?
                .enable_addr_auto();
                spdlog::info!("Server Info {:?}", service_info);

                let task = tokio::spawn(async move {
                    if let Err(e) = daemon.register(service_info) {
                        spdlog::info!("Failed to register mdns service: {}", e);
                        return;
                    }
                    spdlog::info!("mdns server started");
                    // 直接等待关闭信号
                    if rx.await.is_ok() {
                        spdlog::info!("Received shutdown signal");
                        if let Err(e) = daemon.shutdown() {
                            spdlog::info!(
                                "Failed to shutdown mdns daemon: {}",
                                e
                            );
                        }
                    }
                    spdlog::info!("mdns server stopped");
                });
                Ok(task)
            };
        self.service_control.start(mdns_start_logic).await
    }

    pub async fn stop(&self) -> Result<()> {
        self.service_control.stop().await
    }

    pub fn is_running(&self) -> bool {
        self.service_control.is_running()
    }
}
