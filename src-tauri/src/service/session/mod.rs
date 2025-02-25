use std::sync::Arc;

use super::module::connection::DeviceInfo;

#[derive(Debug, Clone)]
pub struct SessionContext {
    pub device_info: Option<DeviceInfo>,
    pub stream: Arc<tokio::net::TcpStream>,
}

impl SessionContext {
    pub fn new(stream: Arc<tokio::net::TcpStream>) -> Self {
        Self { device_info: None, stream }
    }
}
