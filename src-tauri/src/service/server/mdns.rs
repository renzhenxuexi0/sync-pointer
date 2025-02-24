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
    port: Arc<RwLock<u16>>,
    running_task: Arc<RwLock<Option<JoinHandle<()>>>>,
    shutdown_tx: Arc<RwLock<Option<oneshot::Sender<bool>>>>,
}

impl Mdns {
    pub fn instance() -> &'static Self {
        static MDNS: OnceLock<Mdns> = OnceLock::new();
        MDNS.get_or_init(|| Mdns {
            host: Arc::new(RwLock::new(hostname())),
            port: Arc::new(RwLock::new(3456)),
            running_task: Arc::new(RwLock::new(None)),
            shutdown_tx: Arc::new(RwLock::new(None)),
        })
    }

    pub fn set_host(&self, host: String) {
        let mut host_guard = self.host.write();
        *host_guard = host;
    }

    pub fn set_port(&self, port: u16) {
        let mut port_guard = self.port.write();
        *port_guard = port;
    }

    pub fn host(&self) -> String {
        self.host.read().clone()
    }

    pub fn port(&self) -> u16 {
        *self.port.read()
    }

    pub async fn start(&self) -> Result<()> {
        // 如果已经运行，先停止
        if self.running_task.read().is_some() {
            self.stop().await?;
        }

        let daemon = ServiceDaemon::new()?;
        let host_name = self.host() + ".local.";
        let service_info = ServiceInfo::new(
            constant::MDNS_SERVICE_TYPE,
            constant::MDNS_SERVER_NAME,
            &host_name,
            local_ip_address::local_ip()?,
            self.port(),
            HashMap::new(),
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
        if self.running_task.read().is_none() {
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
