use color_eyre::Result;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::app::AppPaths;

pub fn init(app_paths: AppPaths) -> Result<WorkerGuard> {
    color_eyre::install()?;

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let file_appender =
        tracing_appender::rolling::daily(app_paths.cache.join("logs"), "wiremann.log");

    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);

    let file_layer = fmt::layer()
        .json()
        .with_writer(file_writer)
        .with_current_span(true)
        .with_span_list(true)
        .with_thread_names(true)
        .with_target(true);

    let stdout_layer = fmt::layer()
        .compact()
        .with_target(true)
        .with_thread_names(true);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(stdout_layer)
        .with(file_layer)
        .try_init()?;

    Ok(guard)
}
