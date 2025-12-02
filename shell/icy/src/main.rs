use clap::Parser;
use icy::prelude::*;

#[tokio::main]
async fn main() {
    let mut cli = Cli::parse();
    cli.handle().await;
}
