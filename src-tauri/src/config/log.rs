use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use spdlog::{error, sink::RotatingFileSink};

/// 初始化日志
pub fn init(log_path: PathBuf) -> Result<()> {
    let default_log = spdlog::default_logger().fork_with(|new| {
        // 旋转日志
        let rotating_logger = Arc::new(
            RotatingFileSink::builder()
                .level_filter(spdlog::LevelFilter::All)
                .base_path(log_path.join("app.log"))
                // 最多保留10个文件
                .max_files(10)
                .rotation_policy(spdlog::sink::RotationPolicy::FileSize(
                    1024 * 1024 * 5,
                ))
                .error_handler(|e| error!("err: {}", e))
                .build()?,
        );
        new.sinks_mut().push(rotating_logger);
        Ok(())
    })?;
    default_log.set_flush_level_filter(spdlog::LevelFilter::All);
    spdlog::set_default_logger(default_log);
    Ok(())
}
