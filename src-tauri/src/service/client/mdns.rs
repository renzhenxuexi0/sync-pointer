use anyhow::Result;
use mdns_sd::{ServiceDaemon, ServiceEvent};
use parking_lot::RwLock;
use spdlog::{debug, info};
use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};
use tokio::{sync::watch, task::JoinHandle};

use crate::{constant, service::Server};

pub struct Mdns {
    pub servers: Arc<RwLock<HashMap<String, Server>>>,
    running_task: Option<JoinHandle<()>>,
    shutdown_tx: Option<watch::Sender<bool>>,
}

impl Mdns {
    pub fn instance() -> &'static Self {
        static MDNS: OnceLock<Mdns> = OnceLock::new();
        MDNS.get_or_init(|| Mdns {
            servers: Arc::new(RwLock::new(HashMap::new())),
            running_task: None,
            shutdown_tx: None,
        })
    }

    pub async fn start(&mut self) -> Result<()> {
        // 如果已经运行，先停止
        if self.running_task.is_some() {
            self.stop().await?;
        }

        let daemon = ServiceDaemon::new()?;

        let receiver = daemon
            .browse(constant::MDNS_SERVICE_TYPE)
            // TODO 异常处理优化
            .expect("Failed to browse");

        let (tx, rx) = watch::channel(false);
        self.shutdown_tx = Some(tx);

        let servers = Arc::clone(&self.servers);
        // 每100ms扫描一次
        let task = tokio::spawn(async move {
            let rx = rx;
            let now = std::time::Instant::now();

            loop {
                if *rx.borrow() {
                    break;
                }

                if let Ok(event) = receiver.try_recv() {
                    match event {
                        // 服务发现事件
                        ServiceEvent::ServiceResolved(info) => {
                            info!(
                                "At {:?}: Resolved a new service: {}\n host: {}\n port: {}",
                                now.elapsed(),
                                info.get_fullname(),
                                info.get_hostname(),
                                info.get_port(),
                            );
                            for addr in info.get_addresses().iter() {
                                info!(" Address: {}", addr);
                            }
                            for prop in info.get_properties().iter() {
                                info!(" Property: {}", prop);
                            }

                            // 保存服务信息
                            let fullname = info.get_fullname().to_string();
                            let mut servers = servers.write();
                            servers.insert(
                                fullname,
                                Server::new(
                                    info.get_hostname().to_string(),
                                    info.get_addresses().clone(),
                                    info.get_port(),
                                    info.get_properties().clone(),
                                ),
                            );
                        }
                        // 服务被移除事件
                        ServiceEvent::ServiceRemoved(
                            service_type,
                            fullname,
                        ) => {
                            let mut servers = servers.write();
                            servers.remove(&fullname);

                            info!(
                                "At {:?}: Removed a service: {} of type {}",
                                now.elapsed(),
                                fullname,
                                service_type
                            );
                        }
                        other_event => {
                            debug!(
                                "At {:?}: {:?}",
                                now.elapsed(),
                                &other_event
                            );
                        }
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(100))
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

        self.servers.write().clear();
        info!("mdns client stopped");
        Ok(())
    }

    pub fn servers(&self) -> HashMap<String, Server> {
        self.servers.read().clone()
    }
}
