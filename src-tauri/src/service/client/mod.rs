use serde::{Deserialize, Serialize};

pub mod mdns;
pub mod tcp;

/// 从mdns属性解析出用于连接服务端
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    /// 主机名
    pub hostname: String,
    /// IP地址
    pub ip: String,
    /// tcp端口
    pub tcp_port: u16,
}
