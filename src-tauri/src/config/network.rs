use local_ip_address::local_ip;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use tauri::AppHandle;
use tauri_plugin_valtio::ManagerExt as _;
use spdlog::{debug, info, warn};

use crate::constant;

const KEY: &str = "network";

static CONFIG: LazyLock<RwLock<NetworkSettings>> =
    LazyLock::new(|| RwLock::new(NetworkSettings::default()));

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

impl Default for NetworkSettings {
    fn default() -> Self {
        Self {
            service_type: ServiceType::Client,
            hostname: "sync-pointer".to_string(),
            ip: local_ip()
                .map(|addr| addr.to_string())
                .unwrap_or("".to_string()),
            mdns_port: constant::DEFAULT_MDNS_PORT,
            tcp_port: constant::DEFAULT_TCP_PORT,
        }
    }
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

    pub fn tcp_port(&self) -> u16 {
        self.tcp_port
    }
}

// 新增配置管理功能
pub fn get_config() -> NetworkSettings {
    CONFIG.read().clone()
}

pub fn set_config(config: NetworkSettings) {
    info!("更新网络配置: {:?}", config);
    *CONFIG.write() = config;
}

pub fn update_config_from_store(
    app: &AppHandle,
) -> Result<(), tauri_plugin_valtio::Error> {
    match app.valtio().try_get::<NetworkSettings>("store", KEY) {
        Ok(config) => {
            debug!("从存储加载网络配置: {:?}", config);
            set_config(config);
        }
        Err(err) => {
            warn!("无法从存储加载网络配置: {}", err);
            // 使用默认配置
            let default_config = NetworkSettings::default();
            debug!("使用默认网络配置: {:?}", default_config);
        }
    }
    Ok(())
}

pub fn setup_config_watcher(
    app: &AppHandle,
) -> Result<(), tauri_plugin_valtio::Error> {
    // 首次从存储中读取配置
    info!("初始化网络配置监听器");
    update_config_from_store(app)?;

    // 监听配置变更
    app.valtio().watch("store", move |handle| {
        if let Ok(config) =
            handle.valtio().try_get::<NetworkSettings>("store", KEY)
        {
            debug!("检测到网络配置变更: {:?}", config);
            set_config(config);
        }
        Ok(())
    })?;
    
    info!("网络配置监听器设置完成");
    Ok(())
}
