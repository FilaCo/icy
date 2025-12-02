use ::tracing::{info, trace};
use clap::Parser;
use icy::{prelude::*, tracing};

#[tokio::main]
async fn main() {
    let _guard = tracing::init();
    info!("icy starting...");
    let mut cli = Cli::parse();
    trace!("command line args parsed");
    cli.handle().await;
}
