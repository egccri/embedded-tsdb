mod server;
mod service;

use clap::error::ErrorKind;
use clap::Parser;

/// Egccri runtime server
#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct TsdbServer {
    /// Server addr
    server_addr: String,
    // /// Server config file path, the default is "./config/config.toml"
    // #[arg(
    //     short = 'c',
    //     long = "config-file",
    //     default_value = "config/config.toml"
    // )]
    // config_file: PathBuf,
}

impl TsdbServer {
    pub async fn execute(self) -> anyhow::Result<()> {
        tracing::info!("Embedded server staring...");
        server::start(self.server_addr).await;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let tsdb_server = TsdbServer::try_parse().unwrap_or_else(|e| match e.kind() {
        ErrorKind::InvalidSubcommand | ErrorKind::UnknownArgument => TsdbServer {
            server_addr: "0.0.0.0:9999".to_string(),
        },
        _ => e.exit(),
    });
    tsdb_server.execute().await
}
