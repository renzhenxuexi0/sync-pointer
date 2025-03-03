use rkyv::{Archive, Deserialize, Serialize};

/// 组合键修饰符状态
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub struct KeyModifiers {
    /// Shift键是否被按下
    pub shift: bool,
    /// Ctrl键是否被按下
    pub ctrl: bool,
    /// Alt键是否被按下
    pub alt: bool,
    /// Logo键是否被按下（Windows键或macOS的Command键）
    pub logo: bool,
}

/// 键盘请求
///
/// 用于广播键盘事件
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum KeyboardRequest {
    /// 按键按下
    KeyPress {
        /// 键码
        key_code: u32,
        /// 修饰键状态
        modifiers: KeyModifiers,
        /// 输入的字符（如果是文本输入）
        text: Option<char>,
    },
    /// 按键释放
    KeyRelease {
        /// 键码
        key_code: u32,
        /// 修饰键状态
        modifiers: KeyModifiers,
    },
}

/// 鼠标按钮
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum MouseButton {
    /// 鼠标左键
    Left,
    /// 鼠标右键
    Right,
    /// 鼠标中键
    Middle,
    /// 其他鼠标按钮
    Other(u8),
}

/// 鼠标滚轮增量
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum MouseScrollDelta {
    /// 基于行的滚动增量
    LineDelta { x: f32, y: f32 },
    /// 基于像素的滚动增量
    PixelDelta { x: f32, y: f32 },
}

/// 鼠标请求
///
/// 用于广播鼠标事件
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum MouseRequest {
    /// 鼠标移动
    Move {
        /// X坐标
        x: i32,
        /// Y坐标
        y: i32,
        /// 修饰键状态
        modifiers: KeyModifiers,
    },
    /// 按钮操作（按下或释放）
    Button {
        /// 按钮
        button: MouseButton,
        /// 是否按下（true为按下，false为释放）
        pressed: bool,
        /// X坐标
        x: i32,
        /// Y坐标
        y: i32,
        /// 修饰键状态
        modifiers: KeyModifiers,
    },
    /// 滚轮操作
    Scroll {
        /// 滚动增量
        delta: MouseScrollDelta,
        /// X坐标
        x: i32,
        /// Y坐标
        y: i32,
        /// 修饰键状态
        modifiers: KeyModifiers,
    },
}
