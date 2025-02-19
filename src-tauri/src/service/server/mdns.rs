use anyhow::Result;
use mdns_sd::{ServiceDaemon, ServiceInfo};
use parking_lot::RwLock;
use spdlog::info;
use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};
use tauri_plugin_os::hostname;
use tokio::{sync::watch, task::JoinHandle};

use crate::constant;

#[derive(Debug)]
pub struct Mdns {
    host: Arc<RwLock<String>>,
    port: Arc<RwLock<u16>>,
    running_task: Arc<RwLock<Option<JoinHandle<()>>>>,
    shutdown_tx: Arc<RwLock<Option<watch::Sender<bool>>>>,
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

        let (tx, mut rx) = watch::channel(false);
        let mut shutdown_tx = self.shutdown_tx.write();
        *shutdown_tx = Some(tx);
        let service_info = ServiceInfo::new(
            constant::MDNS_SERVICE_TYPE,
            constant::MDNS_SERVER_NAME,
            &self.host(),
            "",
            self.port(),
            HashMap::new(),
        )?;

        let task = tokio::spawn(async move {
            daemon
                .register(service_info)
                .expect("register mdns service failed");

            // 直接等待关闭信号
            rx.changed().await.expect("shutdown signal error");
            daemon.shutdown().unwrap();
        });

        let mut running_task = self.running_task.write();
        *running_task = Some(task);
        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
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
            tx.closed().await;
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
        info!("mdns server stopped");
        Ok(())
    }
}
