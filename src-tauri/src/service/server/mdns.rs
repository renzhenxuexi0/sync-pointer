use anyhow::Result;
use mdns_sd::{ServiceDaemon, ServiceInfo};
use parking_lot::RwLock;
use spdlog::{debug, info};
use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};
use tauri_plugin_os::hostname;
use tokio::{sync::oneshot, task::JoinHandle};

use crate::constant;

#[derive(Debug)]
pub struct Mdns {
    host: Arc<RwLock<String>>,
    mdns_port: Arc<RwLock<u16>>,
    tcp_port: Arc<RwLock<u16>>,
    udp_port: Arc<RwLock<u16>>,
    running_task: Arc<RwLock<Option<JoinHandle<()>>>>,
    shutdown_tx: Arc<RwLock<Option<oneshot::Sender<bool>>>>,
}

impl Mdns {
    pub fn instance() -> &'static Self {
        static MDNS: OnceLock<Mdns> = OnceLock::new();
        MDNS.get_or_init(|| Mdns {
            host: Arc::new(RwLock::new(hostname())),
            mdns_port: Arc::new(RwLock::new(constant::DEFAULT_MDNS_PORT)),
            tcp_port: Arc::new(RwLock::new(constant::DEFAULT_TCP_PORT)),
            udp_port: Arc::new(RwLock::new(constant::DEFAULT_UDP_PORT)),
            running_task: Arc::new(RwLock::new(None)),
            shutdown_tx: Arc::new(RwLock::new(None)),
        })
    }

    pub async fn update_server_info(
        &self,
        host: Option<String>,
        mdns_port: Option<u16>,
        tcp_port: Option<u16>,
        udp_port: Option<u16>,
    ) -> Result<()>{
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

        if let Some(udp_port) = udp_port {
            let mut udp_port_guard = self.udp_port.write();
            *udp_port_guard = udp_port;
            drop(udp_port_guard);
        }

        self.restart_if_running().await
    }

    fn is_running(&self) -> bool {
        self.running_task.read().is_some()
    }

    async fn restart_if_running(&self) -> Result<()>{
        if self.is_running() {
            let mdns = Mdns::instance();
            mdns.stop().await?;
            mdns.start().await?;
        }
        Ok(())
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
    pub fn udp_port(&self) -> u16 {
        *self.udp_port.read()
    }

    pub async fn start(&self) -> Result<()> {
        // 如果已经运行，先停止
        if self.is_running() {
            self.stop().await?;
        }

        let daemon = ServiceDaemon::new()?;
        let host_name = self.host() + ".local.";
        let mut properties = HashMap::new();
        properties
            .insert("tcp_port".to_string(), self.tcp_port.read().to_string());
        properties
            .insert("udp_port".to_string(), self.udp_port.read().to_string());
        let service_info = ServiceInfo::new(
            constant::MDNS_SERVICE_TYPE,
            constant::MDNS_SERVER_NAME,
            &host_name,
            local_ip_address::local_ip()?,
            self.mdns_port(),
            properties,
        )?
        .enable_addr_auto();
        info!("Server Info {:?}", service_info);

        let (tx, rx) = oneshot::channel();
        let mut shutdown_tx = self.shutdown_tx.write();
        *shutdown_tx = Some(tx);
        let task = tokio::spawn(async move {
            if let Err(e) = daemon.register(service_info) {
                info!("Failed to register mdns service: {}", e);
                return;
            }
            info!("mdns server started");
            // 直接等待关闭信号
            if rx.await.is_ok() {
                info!("Received shutdown signal");
                if let Err(e) = daemon.shutdown() {
                    info!("Failed to shutdown mdns daemon: {}", e);
                }
            }
            info!("mdns server stopped");
        });

        let mut running_task = self.running_task.write();
        *running_task = Some(task);
        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        if !self.is_running() {
            debug!("mdns server not running");
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
}
