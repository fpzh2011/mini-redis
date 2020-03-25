use clap::Clap;
use mini_redis::{server, DEFAULT_PORT};
use anyhow::{anyhow, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    // enable logging
    // see https://docs.rs/tracing for more info
    tracing_subscriber::fmt::try_init().map_err(|e| anyhow!("{:?}", e))?;

    let cli = Cli::parse();
    let port = cli.port.unwrap_or(DEFAULT_PORT.to_string());
    server::run(&port).await
}

#[derive(Clap, Debug)]
#[clap(name = "mini-redis-server", version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = "A Redis server")]
struct Cli {
    #[clap(name = "port", long = "--port")]
    port: Option<String>,
}
