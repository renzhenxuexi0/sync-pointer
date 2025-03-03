use rkyv::{Archive, Deserialize, Serialize};
use std::fmt::Debug;

/// 传输方式
///
/// 区分单向广播和需要响应的消息
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum TransportMode {
    /// 单向广播，不需要响应
    Broadcast,
    /// 需要响应的请求-响应模式
    RequestResponse,
}

/// 设备信息结构
///
/// 包含设备的基本信息和标识
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct DeviceInfo {
    /// 设备唯一标识符
    pub device_id: String,
    /// 设备主机名
    pub hostname: String,
    /// 设备系统类型
    pub os_type: OsType,
    /// 应用版本号
    pub app_version: String,
    /// 设备能力列表（用于功能协商）
    pub capabilities: Vec<String>,
}

/// 设备系统类型枚举
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum OsType {
    Windows,
    MacOS,
    Linux,
    Unknown,
}

/// 基础响应状态
///
/// 用于需要响应的操作
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ResponseStatus {
    /// 是否成功
    pub success: bool,
    /// 错误信息（如果失败）
    pub error: Option<String>,
}

/// 数据包
///
/// 网络传输的基本单元
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct DataPacket {
    /// 消息ID（用于请求-响应模式的匹配）
    pub message_id: Option<u64>,
    /// 传输方式
    pub transport_mode: TransportMode,
    /// 时间戳（毫秒）
    pub timestamp: u64,
    /// 数据
    pub data: PacketData,
}

/// 数据包内容
///
/// 所有可能的数据类型
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum PacketData {
    /// 请求数据
    Request(Request),
    /// 响应数据（仅在RequestResponse模式下使用）
    Response(Response),
}

/// 请求数据
///
/// 所有可能的请求类型
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Request {
    /// 连接相关请求（需要响应）
    Connection(super::connection::ConnectionRequest),
    /// 系统相关请求（需要响应）
    System(super::system::SystemRequest),
    /// 设备相关请求（需要响应）
    Device(super::device::DeviceRequest),
    /// 鼠标相关请求（单向广播）
    Mouse(super::input::MouseRequest),
    /// 键盘相关请求（单向广播）
    Keyboard(super::input::KeyboardRequest),
    /// 剪贴板相关请求（需要响应）
    Clipboard(super::clipboard::ClipboardRequest),
}

/// 响应数据
///
/// 所有可能的响应类型（仅用于RequestResponse模式）
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Response {
    /// 连接相关响应
    Connection(super::connection::ConnectionResponse),
    /// 系统相关响应
    System(super::system::SystemResponse),
    /// 设备相关响应
    Device(super::device::DeviceResponse),
    /// 剪贴板相关响应
    Clipboard(super::clipboard::ClipboardResponse),
}
