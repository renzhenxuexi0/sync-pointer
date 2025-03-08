use crate::{service::protocols::input, util};
use rkyv::{Archive, Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};

use super::clipboard::Clipboard;

/// 基础设备信息
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct DeviceInfo {
    pub id: String, // 改用更短的字段名
    pub name: String,
    pub os: OsType,
    pub version: String,
    pub caps: Vec<String>, // 简化 capabilities
}

#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum OsType {
    Win, // 简化枚举名
    Mac,
    Nix,
    Unknown,
}

/// 统一状态信息
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct StatusInfo {
    pub device_id: String,
    pub timestamp: u64,
    pub data: HashMap<String, String>,
}

/// 基础消息包
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct DataPacket {
    // 核心字段
    pub id: u32,   // 消息ID 同一组消息的唯一标识
    pub d: String, // device_id 简写
    pub ts: u64,   // timestamp 简写
    pub data: PacketData,
}

#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum PacketData {
    // 基本响应
    Ok,           // 成功 时间戳
    Fail(String), // 失败
    // 设备管理
    Init(DeviceInfo), // 设备初始化
    Join(DeviceInfo), // 加入网络
    Leave(String),    // 离开网络
    Ping,             // 心跳检测
    Pong,             // 心跳响应

    // 输入事件
    Mouse(input::Mouse),  // 鼠标事件
    Key(input::Keyboard), // 键盘事件

    // 剪贴板
    Clip(Clipboard), // 剪贴板数据
}

impl DataPacket {
    pub fn new(device_id: impl Into<String>, data: PacketData) -> Self {
        Self {
            id: util::generate_id(),
            d: device_id.into(),
            ts: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            data,
        }
    }

    /// 创建错误消息包
    pub fn fail(device_id: impl Into<String>, msg: impl Into<String>) -> Self {
        Self::new(device_id, PacketData::Fail(msg.into()))
    }
}
