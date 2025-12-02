use std::io;

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, fmt::Layer, layer::SubscriberExt};

use crate::directories;

/// Initialize tracing for icy.
/// NOTE: returning a WorkerGuard instance to main is necessary, because it should live until main
/// finishes the execution.
pub fn init() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::daily(directories::logs(), "icy.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    tracing::subscriber::set_global_default(
        tracing_subscriber::registry()
            .with(EnvFilter::from_env("ICY_LOG"))
            .with(Layer::new().with_writer(io::stdout))
            .with(Layer::new().with_writer(non_blocking)),
    )
    .expect("unable to set a global subscriber");

    guard
}
