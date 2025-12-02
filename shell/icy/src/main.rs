use clap::Parser;
use icy::{prelude::*, tracing};

#[tokio::main]
async fn main() {
    let _guard = tracing::init();
    let mut cli = Cli::parse();
    cli.handle().await;
}
