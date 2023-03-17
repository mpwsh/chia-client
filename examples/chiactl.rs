use anyhow::Result;
use serde_json::to_string_pretty;
use chia_node::fullnode::Client as FullNodeClient;
use chia_node::fullnode::Config as FullNodeConfig;
use chia_node::util::decode_puzzle_hash;
use serde::Deserialize;
use std::fs;
use std::net::SocketAddr;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "chiactl",
    about = "A CLI to interact with Chia RPC API Endpoints"
)]
pub struct Cli {
    #[structopt(flatten)]
    global: GlobalOptions,
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
pub struct GlobalOptions {
    #[structopt(
        short,
        long = "key-path",
        global = true,
        help = "Path to FullNode Key",
        parse(from_os_str)
    )]
    key_path: Option<PathBuf>,

    #[structopt(
        short,
        long = "cert-path",
        global = true,
        help = "Path to FullNode Cert",
        parse(from_os_str)
    )]
    cert_path: Option<PathBuf>,

    #[structopt(long, global = true, help = "IP of Chia Node")]
    host: Option<String>,

    #[structopt(long, global = true, help = "Port of Chia FullNode RPC")]
    port: Option<u16>,

    #[structopt(
        long,
        global = true,
        help = "config path for chiactl",
        parse(from_os_str)
    )]
    config: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
struct ConfigFile {
    host: String,
    port: u16,
    key_path: PathBuf,
    cert_path: PathBuf,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "get")]
    Get {
        #[structopt(subcommand)]
        subcommand: GetSubcommand,
    },
}

#[derive(Debug, StructOpt)]
pub enum GetSubcommand {
    #[structopt(name = "balance")]
    Balance {
        #[structopt(name = "wallet_address")]
        address: String,
    },
    #[structopt(name = "network")]
    NetworkInfo,
    #[structopt(name = "blockchain")]
    BlockchainState,
    #[structopt(name = "blockmetrics")]
    BlockCountMetrics,
    #[structopt(name = "block")]
    Block {
        #[structopt(name = "block_hash_or_height", help = "Block Hash or Height")]
        value: String,
    }
}

impl Cli {
    pub async fn load_config(&self) -> Result<FullNodeConfig> {
        // Read the config file and parse it as YAML
        let config_path = if let Some(ref config) = self.global.config {
            config.clone()
        } else {
            dirs::home_dir().unwrap().join(".chiactl/config.yaml")
        };
        let config_str = fs::read_to_string(config_path)?;
        let config_file: ConfigFile = serde_yaml::from_str(&config_str)?;

        // Use the values from the config file as the default values and override them with the values
        // provided as arguments if available
        let host = self.global.host.clone().unwrap_or(config_file.host);
        let port = self.global.port.unwrap_or(config_file.port);
        let key_path = self.global.key_path.clone().unwrap_or(config_file.key_path);
        let cert_path = self
            .global
            .cert_path
            .clone()
            .unwrap_or(config_file.cert_path);

        Ok(FullNodeConfig::new(
            SocketAddr::new(host.parse()?, port),
            &key_path,
            &cert_path,
        ))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::from_args();
    let config = cli.load_config();
    let client = FullNodeClient::new(config.await?).await?;
    match cli.cmd {
        Command::Get { subcommand } => match subcommand {
            GetSubcommand::Balance { address } => get_balance(&client, address).await?,
            GetSubcommand::NetworkInfo => get_network_info(&client).await?,
            GetSubcommand::BlockchainState => get_blockchain_state(&client).await?,
            GetSubcommand::BlockCountMetrics => get_block_count_metrics(&client).await?,
            GetSubcommand::Block { value } => get_block(&client, value).await?,
        },
    }

    Ok(())
}

async fn get_block(client: &FullNodeClient, value: String) -> Result<()> {
    let response = match value.parse::<u64>() {
        Ok(height) => client.get_block_by_height(height).await,
        Err(_) => client.get_block(&value).await,
    };

    let json = serde_json::to_string_pretty(&response?)?;
    println!("{}", json);
    Ok(())
}

async fn get_balance(client: &FullNodeClient, address: String) -> Result<()> {
    let puzzle_hash = decode_puzzle_hash(&address)?;
    let response = client
        .get_coin_records_by_puzzle_hash(&puzzle_hash, None, None, Some(false))
        .await?;
    let balance_mojos: u64 = response.iter().map(|record| record.coin.amount).sum();
    let balance_xch: f64 = balance_mojos as f64 / 1_000_000_000_000.0;
    println!("Balance: {:.12} XCH", balance_xch);
    Ok(())
}

async fn get_network_info(client: &FullNodeClient) -> Result<()> {
    let res = client.get_network_info().await?;
    let json = to_string_pretty(&res)?;
    println!("{}", json);
    Ok(())
}

async fn get_blockchain_state(client: &FullNodeClient) -> Result<()> {
    let res = client.get_blockchain_state().await?;
    let json = to_string_pretty(&res)?;
    println!("{}", json);
    Ok(())

}

async fn get_block_count_metrics(client: &FullNodeClient) -> Result<()> {
    let res = client.get_block_count_metrics().await?;
    let json = to_string_pretty(&res)?;
    println!("{}", json);
    Ok(())
}
