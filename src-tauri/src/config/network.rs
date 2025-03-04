use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSettings {
    // 服务类型
    pub service_type: ServiceType,
    // 主机名
    pub hostname: String,
    // IP 地址
    pub ip: String,
    // 用于服务发现端口
    pub mdns_port: u16,
    // tcp端口 用于监听客户端连接和维护会话数据传输
    pub tcp_port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceType {
    Server,
    Client,
}
