use std::path::PathBuf;

use anyhow::Result;
use tracing::{level_filters::LevelFilter, subscriber, Subscriber};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{
    fmt::{self, MakeWriter},
    layer::SubscriberExt,
    EnvFilter, Registry,
};

// 创建订阅者
fn create_subscriber<W>(
    name: &str,
    env_filter: EnvFilter,
    writer: W,
) -> impl Subscriber + Sync + Send
where
    W: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    // 创建格式化层
    let fmt_layer = fmt::Layer::default()
        // 显示日志来源的目标信息
        .with_target(true)
        // 显示线程 ID
        .with_thread_ids(true)
        // 线程名称信息
        .with_thread_names(true)
        // 时间格式化器
        .with_timer(fmt::time::ChronoUtc::new("%Y-%m-%d %H:%M:%S%.3f".into()))
        // 启用 ANSI 转义码来支持彩色输出
        .with_ansi(true)
        .compact();
    // 注册订阅者
    Registry::default()
        .with(env_filter)
        .with(fmt_layer)
        // 以 JSON 格式进行处理
        .with(JsonStorageLayer)
        // 以文本格式进行输出到文件
        .with(BunyanFormattingLayer::new(name.into(), writer))
}

// 初始化订阅者
pub fn init_subscriber<S>(subscriber: S) -> Result<()>
where
    S: Subscriber + Send + Sync + 'static,
{
    tracing_log::LogTracer::init()?;
    // 设置全局默认订阅者
    subscriber::set_global_default(subscriber)?;
    Ok(())
}

// 初始化并返回文件句柄
pub fn init(log_path: &PathBuf) -> Result<()> {
    // 构建每日日志，前缀为app.log
    let file_appender = RollingFileAppender::new(Rotation::DAILY, log_path, "backend.log");
    // 构建非阻塞日志
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    // 初始化订阅者，从环境变量设置日志级别
    init_subscriber(create_subscriber(
        "sync-pointer",
        EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy(),
        non_blocking,
    ))?;
    Ok(())
}
