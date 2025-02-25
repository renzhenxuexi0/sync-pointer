//! # 网络协议模块
//!
//! 此模块定义了Sync-Pointer应用中所有网络通信使用的数据结构和事件类型。
//! 使用rkyv序列化库进行高效的二进制序列化和反序列化，确保跨平台兼容性和低延迟传输。
//!
//! 主要包括以下事件类型：
//! - 键盘事件：按键按下、释放和输入
//! - 鼠标事件：移动、按键、滚轮和窗口交互
//! - 剪贴板事件：内容同步

use rkyv::{Archive, Deserialize, Serialize};

/// 键盘事件类型枚举
///
/// 表示不同类型的键盘交互事件
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum KeyboardEventType {
    /// 按键被按下事件
    KeyPressed,
    /// 按键按被释放事件
    KeyReleased,
    /// 按键输入事件（通常用于文本输入）
    KeyTyped,
}

/// 组合键修饰符状态
///
/// 记录Shift、Ctrl、Alt和Logo键的按下状态
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub struct KeyModifiers {
    /// Shift键是否被按下
    pub shift: bool,
    /// Ctrl键是否被按下
    pub ctrl: bool,
    /// Alt键是否被按下
    pub alt: bool,
    /// Logo键是否被按下（Windows键或macOS的Command 键键）
    pub logo: bool,
}

/// 键盘事件数据结构
///
/// 包含完整的键盘事件信息，包括事件类型、键码、输入文本和修饰键状态

#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct KeyboardEventData {
    /// 键盘事件类型（按下、释放或输入）
    pub event_type: KeyboardEventType,
    /// 触发事件的键码（与平台相关的键盘扫描码）
    pub key_code: u32,
    /// 输入的字符
    ///
    /// 仅在KeyTyped事件中有效，表示实际输入的Unicode字符
    pub text: Option<char>,
    /// 组合键状态
    ///
    /// 记录事件发生时各修饰键的状态
    pub modifiers: KeyModifiers,
}

/// 剪贴板事件类型枚举
///
/// 定义可能的剪贴板相关事件类型
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ClipboardEventType {
    /// 剪贴板内容已更新
    ClipboardUpdated,
}

/// 剪贴板事件数据结构
///
/// 包含剪贴板事件的类型和内容信息
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ClipboardEventData {
    /// 剪贴板事件类型
    pub event_type: ClipboardEventType,
    /// 剪贴板文本内容
    ///
    /// 当前版本仅支持文本内容传输
    ///
    /// 注：未来可扩展支持其他数据格式（如图片、文件等），
    /// 可通过枚举或二进制数据(Vec<u8>)实现多格式支持
    pub content: String,
}

/// 事件枚举
///
/// 统一不同类型事件的数据容器
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Event {
    /// 鼠标相关事件
    Mouse(MouseEventData),
    /// 键盘相关事件
    Keyboard(KeyboardEventData),
    /// 剪贴板相关事件
    Clipboard(ClipboardEventData),
    /// 心跳事件
    Heartbeat,
}

/// 事件类型枚举
///
/// 定义系统支持的主要事件类别
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum EventType {
    /// 鼠标事件类型
    MouseEvent,
    /// 键盘事件类型
    KeyboardEvent,
    /// 剪贴板事件类型
    ClipboardEvent,
}

/// 鼠标按钮枚举
///
/// 定义鼠标可能的按键
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum MouseButton {
    /// 鼠标左键
    Left,
    /// 鼠标右键
    Right,
    /// 鼠标中键（通常是滚轮按下）
    Middle,
    /// 其他鼠标按钮（如侧键、额外功能键等）
    Other(u8),
}

/// 鼠标按钮状态枚举
///
/// 定义鼠标按钮的可能状态
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum MouseButtonState {
    /// 按钮被按下
    Pressed,
    /// 按钮被释放
    Released,
}

/// 鼠标滚轮增量枚举
///
/// 表示鼠标滚轮滚动的不同测量方式
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum MouseScrollDelta {
    /// 基于行的滚动增量
    ///
    /// 通常用于基于行的滚动，如文本编辑器
    LineDelta {
        /// 水平滚动行数
        x: f32,
        /// 垂直滚动行数
        y: f32,
    },
    /// 基于像素的滚动增量
    ///
    /// 通常用于平滑滚动或精确定位
    PixelDelta {
        /// 水平滚动像素
        x: f32,
        /// 垂直滚动像素
        y: f32,
    },
}

/// 鼠标事件类型枚举
///
/// 定义所有支持的鼠标事件类型

#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum MouseEventType {
    /// 鼠标光标移动
    Moved,
    /// 鼠标按钮被按下
    ButtonPressed,
    /// 鼠标按键钮被释放
    ButtonReleased,
    /// 鼠标按滚轮滚动
    WheelScrolled,
    /// 鼠标滚指针进入窗口
    EnteredWindow,
    /// 鼠标进指针离开窗口
    LeftWindow,
}

/// 鼠标事件数据结构
///
/// 包含完整的鼠标事件信息，包括位置、按钮状态和滚轮数据
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct MouseEventData {
    /// 鼠标事件类型
    pub event_type: MouseEventType,
    /// 鼠标指针X坐标（相对于窗口或屏幕）
    pub x: i32,
    /// 鼠标指针Y坐标（相对于窗口或屏幕）
    pub y: i32,
    /// 触发事件的鼠标按钮
    ///
    /// 对于移动和滚轮事件可能为None
    pub button: Option<MouseButton>,
    /// 鼠标按钮状态
    ///
    /// 仅对按键事件有效（按下或释放）
    pub button_state: Option<MouseButtonState>,
    /// 滚轮滚动增量
    ///
    /// 仅对滚轮事件有效
    pub scroll_delta: Option<MouseScrollDelta>,
    /// 组合键修饰符状态
    ///
    /// 记录事件发生时各修饰键（Shift、Ctrl、Alt等）的状态
    pub modifiers: KeyModifiers,
}

/// 数据包结构
///
/// 网络传输的基本单元，包含事件类型和具体事件数据
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct DataPacket {
    /// 事件类型（鼠标、键盘或剪贴板）
    pub event_type: EventType,
    /// 事件具体数据
    ///
    /// 使用Event枚举包装不同类型的事件数据
    pub event_data: Event,
}
