use rkyv::{Archive, Deserialize, Serialize};

/// 剪贴板内容类型
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ClipType {
    // 简化名称
    Text,  // 纯文本
    Rich,  // 富文本
    Img,   // 图片
    Files, // 文件列表
}

/// 剪贴板内容
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ClipData {
    // 简化名称
    pub ty: ClipType,   // 使用更短的字段名
    pub data: Vec<u8>,  // 使用字节数组存储
    pub ts: u64,        // 时间戳
    pub compress: bool, // 是否压缩
}

/// 剪贴板消息
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Clipboard {
    /// 更新内容
    Set { data: ClipData, force: bool },

    /// 获取内容
    Get {
        types: Option<Vec<ClipType>>,
        since: Option<u64>, // 上次同步时间
    },

    /// 内容数据
    Data { items: Vec<ClipData> },

    /// 清除
    Clear,

    /// 已清除
    Cleared,
}

impl ClipData {
    pub fn new(ty: ClipType, data: Vec<u8>, compress: bool) -> Self {
        Self {
            ty,
            data,
            ts: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            compress,
        }
    }

    /// 创建文本内容
    pub fn text(text: impl Into<String>) -> Self {
        Self::new(ClipType::Text, text.into().into_bytes(), false)
    }

    /// 创建压缩的图片内容
    pub fn image(data: Vec<u8>) -> Self {
        Self::new(ClipType::Img, data, true)
    }

    /// 创建文件列表
    pub fn files(paths: Vec<String>) -> Self {
        Self::new(ClipType::Files, paths.join("\n").into_bytes(), false)
    }
}
