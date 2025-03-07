use anyhow::Result;
use parking_lot::RwLock;
use std::sync::{Arc, OnceLock};
use tauri_plugin_os::hostname;
use tokio::{sync::oneshot, task::JoinHandle};

use crate::{constant, service::ServiceControl};

#[derive(Debug)]
pub struct MdnsServer {
    service_control: ServiceControl,
    host: Arc<RwLock<String>>,
    mdns_port: Arc<RwLock<u16>>,
    tcp_port: Arc<RwLock<u16>>,
}

impl MdnsServer {
    pub fn instance() -> &'static Self {
        static MDNS: OnceLock<MdnsServer> = OnceLock::new();
        MDNS.get_or_init(|| MdnsServer {
            service_control: ServiceControl::new("Mdns Server".to_string()),
            host: Arc::new(RwLock::new(hostname())),
            mdns_port: Arc::new(RwLock::new(constant::DEFAULT_MDNS_PORT)),
            tcp_port: Arc::new(RwLock::new(constant::DEFAULT_TCP_PORT)),
        })
    }

    pub fn host(&self) -> String {
        self.host.read().clone()
    }
    pub fn mdns_port(&self) -> u16 {
        *self.mdns_port.read()
    }
    pub fn tcp_port(&self) -> u16 {
        *self.tcp_port.read()
    }

    pub async fn update_server_info(
        &self,
        host: Option<String>,
        mdns_port: Option<u16>,
        tcp_port: Option<u16>,
    ) -> Result<()> {
        if let Some(host) = host {
            let mut host_guard = self.host.write();
            *host_guard = host;
            drop(host_guard); // Release lock before restarting
        }

        if let Some(mdns_port) = mdns_port {
            let mut mdns_port_guard = self.mdns_port.write();
            *mdns_port_guard = mdns_port;
            drop(mdns_port_guard);
        }

        if let Some(tcp_port) = tcp_port {
            let mut tcp_port_guard = self.tcp_port.write();
            *tcp_port_guard = tcp_port;
            drop(tcp_port_guard);
        }

        if self.is_running() {
            let mdns = MdnsServer::instance();
            mdns.stop().await?;
            mdns.start().await?;
        }
        Ok(())
    }

    pub async fn start(&self) -> Result<()> {
        // Clone the values we need from self to avoid capturing self in the closure
        let host_name = self.host() + ".local.";
        let tcp_port = self.tcp_port();
        let mdns_port = self.mdns_port();

        let mdns_start_logic =
            move |rx: oneshot::Receiver<bool>| -> Result<JoinHandle<()>> {
                let daemon = mdns_sd::ServiceDaemon::new()?;
                #[cfg(not(target_os = "windows"))]
                {
                    daemon.set_multicast_loop_v4(false)?;
                    daemon.set_multicast_loop_v6(false)?;
                }
                let mut properties = std::collections::HashMap::new();
                properties.insert("tcp_port".to_string(), tcp_port.to_string());
                let service_info = mdns_sd::ServiceInfo::new(
                    crate::constant::MDNS_SERVICE_TYPE,
                    crate::constant::MDNS_SERVER_NAME,
                    &host_name,
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
