// src/logger.rs
use std::path::Path;

use tracing::Level;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{EnvFilter, Registry, fmt, layer::SubscriberExt};

/// 初始化日志系统
///
/// # 参数
/// - `log_dir`: 日志文件夹路径（如 "logs"）
/// - `log_filename`: 日志文件名（如 "server.log"）
/// - `level`: 日志等级（如 Level::INFO）
///
/// # 示例
/// ```
/// init_logger("logs", "server.log", Level::INFO).unwrap();
/// ```
pub fn init_logger<P: AsRef<Path>>(
    log_dir: P,
    log_filename: &str,
    level: Level,
) -> Result<(), Box<dyn std::error::Error>> {
    // 控制台输出
    let stdout_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_target(false)
        .with_timer(fmt::time::ChronoLocal::rfc_3339())
        .with_level(true)
        .with_thread_ids(true)
        .with_thread_names(true);

    // 文件输出
    let file_appender: RollingFileAppender =
        tracing_appender::rolling::daily(log_dir, log_filename);

    let file_layer = fmt::layer()
        .with_writer(file_appender)
        .with_ansi(false)
        .with_timer(fmt::time::ChronoLocal::rfc_3339())
        .with_level(true)
        .with_target(false);

    // 日志等级过滤器
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::from(level.to_string()));

    // 注册日志层
    tracing::subscriber::set_global_default(
        Registry::default()
            .with(filter)
            .with(stdout_layer)
            .with(file_layer),
    )?;

    Ok(())
}

fn main() -> anyhow::Result<(), Box<dyn std::error::Error>> {
    init_logger("logs", "my_app.log", tracing::Level::INFO)?;

    tracing::info!("App started");
    tracing::error!("database connection failed");
    Ok(())
}
