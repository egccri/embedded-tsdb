use anyhow::anyhow;
use clap::error::ErrorKind;
use clap::Parser;
use cli::commands::exec::ExecCommand;
use tonic::transport::Channel;

/// Embedded tsdb terminal client
#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct EmbeddedTsdbClient {
    /// Embedded tsdb server addr
    #[arg(short = 's', long = "server_addr", default_value = "127.0.0.1:9999")]
    server_addr: String,

    /// Sub command
    #[clap(subcommand)]
    command: ClientCommand,
}

#[derive(clap::Subcommand, Debug)]
enum ClientCommand {
    /// Exec command
    Exec(ExecCommand),
}

impl EmbeddedTsdbClient {
    pub async fn execute(self) -> anyhow::Result<()> {
        let server_addr = self.server_addr.clone();
        let channel = Client::connect(server_addr).await?;
        match self.command {
            ClientCommand::Exec(exec_command) => exec_command.execute(channel).await,
        }
    }
}

pub struct Client;

impl Client {
    pub async fn connect(server_addr: String) -> anyhow::Result<Channel> {
        tracing::info!("Connect to runtime server: {}", &server_addr);
        let server_addr = format!("http://{}", server_addr);
        Channel::from_shared(server_addr)
            .map_err(|err| anyhow!("Connect server error, cause by {err}"))?
            .connect()
            .await
            .map_err(|err| anyhow!("Connect server error, cause by {:?}", err))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    // parse or else default run command.
    EmbeddedTsdbClient::try_parse()
        .unwrap_or_else(|e| match e.kind() {
            ErrorKind::InvalidSubcommand | ErrorKind::UnknownArgument => {
                eprintln!("Error command, please check your command.");
                e.exit()
            }
            _ => e.exit(),
        })
        .execute()
        .await
}
