// src/logger.rs
use std::path::Path;

use tracing::Level;
use tracing_appender::non_blocking;
use tracing_appender::rolling::RollingFileAppender;
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
    // 控制台输出：默认是同步输出，这里不包装为异步（更方便实时调试）
    let stdout_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_timer(fmt::time::ChronoLocal::rfc_3339())
        .with_level(true)
        .with_target(false)
        .with_thread_names(true);

    // 创建文件写入器并包装成异步写入器
    let file_appender: RollingFileAppender =
        tracing_appender::rolling::daily(log_dir, log_filename);
    let (non_blocking_file_writer, _guard) = non_blocking(file_appender);

    // 注意：_guard 必须保留到程序结束，否则后台线程会退出

    // 文件输出层（异步写入）
    let file_layer = fmt::layer()
        .with_writer(non_blocking_file_writer)
        .with_timer(fmt::time::ChronoLocal::rfc_3339())
        .with_level(true)
        .with_target(false);

    // 日志等级过滤器
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::from(level.to_string()));

    // 组合日志层并设置为全局默认
    tracing::subscriber::set_global_default(
        Registry::default()
            .with(filter)
            .with(stdout_layer)
            .with(file_layer),
    )?;

    // 让后台线程的 _guard 在后台运行
    // 最好用 tokio::task::spawn_blocking 保证其生命周期
    tokio::spawn(async move {
        // 保持 _guard 不被 drop，直到程序结束
        let _ = _guard;
    });

    Ok(())
}
