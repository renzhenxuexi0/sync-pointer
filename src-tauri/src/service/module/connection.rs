use serde::{Deserialize, Serialize};

/// 连接类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceType {
    /// 服务端
    Server,
    /// 客户端
    Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// 主机名
    pub hostname: String,
    /// IP地址
    pub ip: String,
    /// tcp端口
    pub tcp_port: u16,
    /// 服务类型
    pub service_type: ServiceType,
}
