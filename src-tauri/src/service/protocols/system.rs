use rkyv::{Archive, Deserialize, Serialize};
use std::collections::HashMap;

use super::base::ResponseStatus;

/// 同步数据类型
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum SyncDataType {
    /// 屏幕信息
    ScreenInfo,
    /// 性能设置
    PerformanceSettings,
    /// 应用状态
    ApplicationState,
    /// 其他自定义数据
    Custom(String),
}

/// 系统请求枚举
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum SystemRequest {
    /// 状态同步请求
    StatusSync {
        /// 请求同步的数据类型
        data_type: SyncDataType,
        /// 额外参数
        parameters: HashMap<String, String>,
    },
    /// 错误报告
    ErrorReport {
        /// 错误代码
        code: String,
        /// 错误描述
        message: String,
        /// 错误发生时间
        timestamp: u64,
        /// 错误上下文
        context: HashMap<String, String>,
        /// 堆栈跟踪（可选）
        stack_trace: Option<String>,
    },
    /// 性能配置更新请求
    UpdatePerformanceConfig {
        /// 目标帧率
        target_fps: Option<u32>,
        /// 输入延迟补偿（毫秒）
        input_delay_compensation: Option<u32>,
        /// 是否启用平滑滚动
        smooth_scroll: Option<bool>,
        /// 自定义性能参数
        custom_params: HashMap<String, String>,
    },
    /// 日志请求
    Log {
        /// 日志级别
        level: String,
        /// 日志消息
        message: String,
        /// 日志时间
        timestamp: u64,
        /// 日志标签
        tags: Vec<String>,
        /// 额外数据
        metadata: HashMap<String, String>,
    },
}

/// 系统响应枚举
#[derive(Archive, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum SystemResponse {
    /// 状态同步响应
    StatusSync {
        /// 响应状态
        status: ResponseStatus,
        /// 同步数据类型
        data_type: SyncDataType,
        /// 同步数据（JSON格式）
        data: Option<String>,
    },
    /// 错误报告响应
    ErrorReport {
        /// 响应状态
        status: ResponseStatus,
        /// 处理建议
        suggestion: Option<String>,
        /// 是否需要客户端立即重试
        should_retry: bool,
    },
    /// 性能配置更新响应
    UpdatePerformanceConfig {
        /// 响应状态
        status: ResponseStatus,
        /// 实际应用的配置
        applied_config: HashMap<String, String>,
    },
    /// 日志响应
    Log {
        /// 响应状态
        status: ResponseStatus,
        /// 日志ID（用于追踪）
        log_id: String,
    },
}
