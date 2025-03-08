use serde::{Deserialize, Serialize};
use tauri_plugin_valtio::ManagerExt;

use crate::{constant, core};

const KEY: &str = "network";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceType {
    Server,
    Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSettings {
    // 服务类型
    service_type: ServiceType,
    // 主机名
    hostname: String,
    // IP 地址
    ip: String,
    // 用于服务发现端口
    mdns_port: u16,
    // tcp端口 用于监听客户端连接和维护会话数据传输
    tcp_port: u16,
}

impl NetworkSettings {
    pub fn service_type(&self) -> ServiceType {
        self.service_type.clone()
    }

    pub fn hostname(&self) -> String {
        self.hostname.clone()
    }

    pub fn ip(&self) -> String {
        self.ip.clone()
    }

    pub fn mdns_port(&self) -> u16 {
        self.mdns_port
    }
}

// 获取配置
pub fn config() -> Option<NetworkSettings> {
    core::handle::Handle::instance()
        .app_handle()
        .and_then(|handle| handle.valtio().get(constant::STORE_ID, KEY))
        .and_then(|value| serde_json::from_value(value).ok())
}
