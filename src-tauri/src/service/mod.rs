pub mod client;
pub mod codec;
pub mod handler;
pub mod protocols;
pub mod server;

use anyhow::{Result, anyhow};
use parking_lot::RwLock;
use spdlog::debug;
use std::sync::Arc;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

#[derive(Debug)]
pub struct ServiceControl {
    service_name: String,
    running_task: Arc<RwLock<Option<JoinHandle<()>>>>,
    shutdown_tx: Arc<RwLock<Option<oneshot::Sender<bool>>>>,
}

impl ServiceControl {
    pub fn new(service_name: String) -> Self {
        ServiceControl {
            service_name,
            running_task: Arc::new(RwLock::new(None)),
            shutdown_tx: Arc::new(RwLock::new(None)),
        }
    }

    pub fn is_running(&self) -> bool {
        self.running_task.read().is_some()
    }

    pub async fn start<F>(&self, start_fn: F) -> Result<()>
    where
        F: FnOnce(oneshot::Receiver<bool>) -> Result<JoinHandle<()>>
            + Send
            + 'static,
    {
        if self.is_running() {
            self.stop().await?;
        }

        let (tx, rx) = oneshot::channel();
        let mut shutdown_tx = self.shutdown_tx.write();
        *shutdown_tx = Some(tx);
        drop(shutdown_tx);

        let task_result = start_fn(rx);
        match task_result {
            Ok(task) => {
                let mut running_task = self.running_task.write();
                *running_task = Some(task);
                Ok(())
            }
            Err(e) => {
                let mut shutdown_tx = self.shutdown_tx.write();
                *shutdown_tx = None;
                Err(e)
            }
        }
    }

    pub async fn stop(&self) -> Result<()> {
        if !self.is_running() {
            debug!("{} service not running", self.service_name);
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
                anyhow!(
                    "{} Failed to send shutdown signal: {}",
                    self.service_name,
                    e
                )
            })?;
        }

        if let Some(task) = task {
            task.await.map_err(|e| {
                anyhow!("{} Task join error: {}", self.service_name, e)
            })?;
        }

        {
            let mut shutdown_tx = self.shutdown_tx.write();
            *shutdown_tx = None;
        }
        {
            let mut running_task = self.running_task.write();
            *running_task = None;
        }
        debug!("{} service is stopped", self.service_name);
        Ok(())
    }
}
