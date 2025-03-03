use rkyv::{Archive, Deserialize, Serialize};

use super::base::ResponseStatus;

/// 剪贴板内容类型
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ClipboardContentType {
    /// 纯文本
    PlainText,
    /// 富文本
    RichText,
    /// 图片（Base64编码）
    Image,
    /// 文件列表
    FileList,
}

/// 剪贴板内容
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ClipboardContent {
    /// 内容类型
    pub content_type: ClipboardContentType,
    /// 内容数据
    pub data: String,
    /// 内容元数据
    pub metadata: Option<String>,
}

/// 剪贴板请求枚举
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ClipboardRequest {
    /// 更新剪贴板内容
    Update {
        /// 新的剪贴板内容
        content: ClipboardContent,
        /// 是否强制更新（即使目标设备的剪贴板更新时间更新）
        force: bool,
    },
    /// 请求获取剪贴板内容
    Get {
        /// 请求的内容类型（为空表示获取所有可用类型）
        content_types: Option<Vec<ClipboardContentType>>,
        /// 上次同步的时间戳
        last_sync_timestamp: Option<u64>,
    },
    /// 清除剪贴板
    Clear,
}

/// 剪贴板响应枚举
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ClipboardResponse {
    /// 更新剪贴板内容响应
    Update {
        /// 响应状态
        status: ResponseStatus,
        /// 更新时间戳
        timestamp: u64,
    },
    /// 获取剪贴板内容响应
    Get {
        /// 响应状态
        status: ResponseStatus,
        /// 剪贴板内容（如果有多种类型，返回所有可用的）
        contents: Option<Vec<ClipboardContent>>,
        /// 内容时间戳
        timestamp: u64,
    },
    /// 清除剪贴板响应
    Clear {
        /// 响应状态
        status: ResponseStatus,
    },
}
