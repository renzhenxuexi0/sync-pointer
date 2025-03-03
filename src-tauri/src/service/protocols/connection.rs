use rkyv::{Archive, Deserialize, Serialize};
use std::collections::HashMap;

use super::base::{DeviceInfo, ResponseStatus};

/// 设备能力描述
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Capability {
    /// 能力名称
    pub name: String,
    /// 能力版本
    pub version: String,
    /// 能力参数
    pub parameters: HashMap<String, String>,
}

/// 连接请求枚举
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ConnectionRequest {
    /// 连接请求
    Connect {
        /// 请求连接的设备信息
        device_info: DeviceInfo,
        /// 连接密钥（可选，用于认证）
        connection_key: Option<String>,
        /// 请求的能力列表
        requested_capabilities: Vec<String>,
    },
    /// 断开连接请求
    Disconnect {
        /// 请求断开的设备ID
        device_id: String,
    },
    /// 能力协商请求
    NegotiateCapabilities {
        /// 设备ID
        device_id: String,
        /// 设备支持的能力列表
        supported_capabilities: Vec<Capability>,
    },
    /// 心跳请求
    Heartbeat {
        /// 设备ID
        device_id: String,
        /// 时间戳
        timestamp: u64,
    },
}

/// 连接响应枚举
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ConnectionResponse {
    /// 连接响应
    Connect {
        /// 响应状态
        status: ResponseStatus,
        /// 连接成功时返回的设备信息
        device_info: Option<DeviceInfo>,
        /// 服务端支持的能力列表
        supported_capabilities: Vec<Capability>,
    },
    /// 断开连接响应
    Disconnect {
        /// 响应状态
        status: ResponseStatus,
    },
    /// 能力协商响应
    NegotiateCapabilities {
        /// 响应状态
        status: ResponseStatus,
        /// 协商结果：最终双方都支持的能力列表
        negotiated_capabilities: Vec<Capability>,
    },
    /// 心跳响应
    Heartbeat {
        /// 响应状态
        status: ResponseStatus,
        /// 原始请求时间戳
        request_timestamp: u64,
        /// 响应时间戳
        response_timestamp: u64,
    },
}
