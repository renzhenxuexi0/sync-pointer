use anyhow::Result;
use mdns_sd::{ServiceDaemon, ServiceInfo};
use spdlog::info;
use std::{collections::HashMap, sync::OnceLock};
use tauri_plugin_os::hostname;
use tokio::{sync::watch, task::JoinHandle};

use crate::constant;

#[derive(Debug)]
pub struct Mdns {
    host: String,
    port: u16,
    running_task: Option<JoinHandle<()>>,
    shutdown_tx: Option<watch::Sender<bool>>,
}

impl Mdns {
    pub fn instance() -> &'static Self {
        static MDNS: OnceLock<Mdns> = OnceLock::new();
        MDNS.get_or_init(|| Mdns {
            host: hostname(),
            port: 3456,
            running_task: None,
            shutdown_tx: None,
        })
    }

    pub fn set_host(&mut self, host: String) {
        self.host = host;
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    pub async fn start(&mut self) -> Result<()> {
        // 如果已经运行，先停止
        if self.running_task.is_some() {
            self.stop().await?;
        }

        let daemon = ServiceDaemon::new()?;

        let (tx, rx) = watch::channel(false);
        self.shutdown_tx = Some(tx);
        let service_info = ServiceInfo::new(
            constant::MDNS_SERVICE_TYPE,
            constant::MDNS_SERVER_NAME,
            &self.host,
            "",
            self.port,
            HashMap::new(),
        )?;

        // 每100ms扫描一次
        let task = tokio::spawn(async move {
            let rx = rx;
            daemon
                .register(service_info)
                .expect("register mdns service failed");
            loop {
                if *rx.borrow() {
                    break;
                }

                tokio::time::sleep(tokio::time::Duration::from_millis(1000))
                    .await;
            }
            daemon.shutdown().unwrap();
        });

        self.running_task = Some(task);
        Ok(())
    }

    pub async fn stop(&mut self) -> Result<()> {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(true);
        }

        if let Some(task) = self.running_task.take() {
            task.await?;
        }
        info!("mdns client stopped");
        Ok(())
    }
}
