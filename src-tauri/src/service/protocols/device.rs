use rkyv::{Archive, Deserialize, Serialize};
use std::collections::HashMap;

use super::base::{DeviceInfo, ResponseStatus};

/// 屏幕布局信息
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ScreenLayout {
    /// 显示器分辨率宽度
    pub width: u32,
    /// 显示器分辨率高度
    pub height: u32,
    /// 缩放因子
    pub scale_factor: f64,
    /// 相对位置X坐标
    pub position_x: i32,
    /// 相对位置Y坐标
    pub position_y: i32,
    /// 是否是主显示器
    pub is_primary: bool,
}

/// 设备请求枚举
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum DeviceRequest {
    /// 更新设备信息
    UpdateInfo {
        /// 新的设备信息
        device_info: DeviceInfo,
    },
    /// 更新设备状态
    UpdateStatus {
        /// 设备ID
        device_id: String,
        /// 新的状态数据
        status: HashMap<String, String>,
    },
    /// 更新屏幕布局
    UpdateScreenLayout {
        /// 设备ID
        device_id: String,
        /// 屏幕布局信息
        layout: Vec<ScreenLayout>,
    },
    /// 查询设备信息
    QueryInfo {
        /// 设备ID
        device_id: String,
    },
    /// 查询设备状态
    QueryStatus {
        /// 设备ID
        device_id: String,
        /// 查询的状态键列表（为空表示查询所有）
        keys: Option<Vec<String>>,
    },
    /// 查询屏幕布局
    QueryScreenLayout {
        /// 设备ID
        device_id: String,
    },
}

/// 设备响应枚举
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum DeviceResponse {
    /// 更新设备信息响应
    UpdateInfo {
        /// 响应状态
        status: ResponseStatus,
    },
    /// 更新设备状态响应
    UpdateStatus {
        /// 响应状态
        status: ResponseStatus,
    },
    /// 更新屏幕布局响应
    UpdateScreenLayout {
        /// 响应状态
        status: ResponseStatus,
    },
    /// 查询设备信息响应
    QueryInfo {
        /// 响应状态
        status: ResponseStatus,
        /// 查询到的设备信息
        device_info: Option<DeviceInfo>,
    },
    /// 查询设备状态响应
    QueryStatus {
        /// 响应状态
        status: ResponseStatus,
        /// 查询到的状态数据
        status_data: Option<HashMap<String, String>>,
    },
    /// 查询屏幕布局响应
    QueryScreenLayout {
        /// 响应状态
        status: ResponseStatus,
        /// 屏幕布局信息
        layout: Option<Vec<ScreenLayout>>,
    },
}
