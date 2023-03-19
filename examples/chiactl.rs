use anyhow::Result;
use chia_node::fullnode::Client as FullNodeClient;
use chia_node::fullnode::Config as FullNodeConfig;
use chia_node::util::{decode_puzzle_hash, encode_puzzle_hash};
use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::cmp::Ordering;
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
    #[structopt(name = "clvm")]
    Clvm {
        #[structopt(subcommand)]
        subcommand: ClvmSubcommand,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Amount {
    xch: f64,
    mojo: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Direction {
    Sent,
    Received,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    coin: String,
    sender: String,
    recipient: String,
    confirmed_height: u64,
    spent_height: u64,
    amount: Amount,
    direction: Direction,
    timestamp: Option<DateTime<Utc>>,
}
#[derive(Debug, StructOpt)]
pub enum GetSubcommand {
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
    },
    #[structopt(name = "coin")]
    Coin {
        #[structopt(name = "coin")]
        value: String,
        #[structopt(long = "show-parent")]
        show_parent: bool,
        #[structopt(long = "encode")]
        encode: bool,
        #[structopt(long = "prefix")]
        prefix: String,
    },
    #[structopt(name = "balance")]
    Balance {
        #[structopt(name = "wallet_address")]
        address: String,
    },
    #[structopt(name = "transactions")]
    Transactions {
        #[structopt(name = "wallet_address")]
        address: String,
    },
}

#[derive(Debug, StructOpt)]
pub enum ClvmSubcommand {
    #[structopt(name = "encode")]
    Encode {
        #[structopt(name = "puzzle_hash")]
        hash: String,
        #[structopt(name = "prefix")]
        prefix: String,
    },
    #[structopt(name = "decode")]
    Decode {
        #[structopt(name = "address")]
        address: String,
    },
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
            GetSubcommand::Coin { value, show_parent, encode, prefix } => get_coin(&client, value, show_parent, encode, prefix).await?,
            GetSubcommand::Transactions { address } => get_transactions(&client, address).await?,
        },
        Command::Clvm { subcommand } => match subcommand {
            ClvmSubcommand::Encode { hash, prefix } => {
                println!("{}", encode_puzzle_hash(&hash, &prefix)?)
            }
            ClvmSubcommand::Decode { address } => println!("{}", decode_puzzle_hash(&address)?),
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

async fn get_coin(
    client: &FullNodeClient,
    value: String,
    show_parent: bool,
    encode: bool,
    prefix: String,
) -> Result<()> {
    let response = client
        .get_coin_record_by_name(&value)
        .await?;
    let mut coin_record = response.clone();
    if encode {
        let prefix = "xch";
        coin_record.coin.puzzle_hash = encode_puzzle_hash(&response.coin.puzzle_hash, prefix)?;
    }

    let json = serde_json::to_string_pretty(&coin_record)?;
    println!("{}", json);

    if show_parent {
        let parent_response = client
            .get_coin_record_by_name(&response.coin.parent_coin_info)
            .await?;
        let mut parent_coin_record = parent_response.clone();

        if encode {
            parent_coin_record.coin.puzzle_hash =
                encode_puzzle_hash(&parent_response.coin.puzzle_hash, &prefix)?;
        }

        let parent_json = serde_json::to_string_pretty(&parent_coin_record)?;
        println!("\nParent Coin:");
        println!("{}", parent_json);
    }

    
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
async fn get_transactions(client: &FullNodeClient, address: String) -> Result<()> {
    let puzzle_hash = decode_puzzle_hash(&address)?;
    let prefix = "xch";
    let response = client
        .get_coin_records_by_puzzle_hash(&puzzle_hash, None, None, Some(true))
        .await?;

    let mut transactions: Vec<Transaction> = Vec::new();

    for record in response {
        let received_record = client
            .get_coin_record_by_name(&record.coin.parent_coin_info)
            .await?;
        let coin = &record.coin.parent_coin_info;
        let sender = encode_puzzle_hash(&received_record.coin.puzzle_hash, prefix).unwrap();
        let recipient = encode_puzzle_hash(&record.coin.puzzle_hash, prefix).unwrap();
        let direction = Direction::Received;
        let confirmed_height = record.confirmed_block_index;
        let spent_height = record.spent_block_index;
        let amount = Amount {
            xch: record.coin.amount as f64 / 1_000_000_000_000.0,
            mojo: record.coin.amount,
        };
        transactions.push(Transaction {
            coin: coin.clone(),
            recipient,
            sender,
            amount: amount.clone(),
            confirmed_height,
            spent_height,
            direction,
            timestamp: record.timestamp,
        });

        if received_record.spent {
            let spent_record = client
                .get_coin_record_by_name(&record.coin.parent_coin_info)
                .await?;
            let recipient = encode_puzzle_hash(&spent_record.coin.puzzle_hash, prefix).unwrap();
            let sender = address.clone(); //encode_puzzle_hash(&address, prefix).unwrap();
            let direction = Direction::Sent;
            let confirmed_height = spent_record.confirmed_block_index;
            let spent_height = spent_record.spent_block_index;

            let amount = Amount {
                xch: spent_record.coin.amount as f64 / 1_000_000_000_000.0,
                mojo: spent_record.coin.amount,
            };
            transactions.push(Transaction {
                coin: coin.clone(),
                sender,
                recipient,
                amount: amount.clone(),
                confirmed_height,
                spent_height,
                direction,
                timestamp: spent_record.timestamp,
            });
        }
    }

    let json = to_string_pretty(&sort_by_date(transactions))?;
    println!("{}", json);
    Ok(())
}
/*
async fn get_transactions(client: &FullNodeClient, address: String) -> Result<()> {
    let puzzle_hash = decode_puzzle_hash(&address)?;
    let prefix = "xch";
    let response = client
        .get_coin_records_by_puzzle_hash(&puzzle_hash, None, None, Some(true))
        .await?;

    let mut transactions: Vec<Transaction> = Vec::new();

    for record in response {
        let spent_record = client
            .get_coin_record_by_name(&record.coin.parent_coin_info)
            .await?;
        let from = encode_puzzle_hash(&spent_record.coin.puzzle_hash, prefix).unwrap();
        let to = encode_puzzle_hash(&record.coin.puzzle_hash, prefix).unwrap();
        let direction = if spent_record.spent {
            Direction::Received
        } else {
            Direction::Sent
        };

        let amount = Amount {
            xch: record.coin.amount as f64 / 1_000_000_000_000.0,
            mojo: record.coin.amount,
        };
        transactions.push(Transaction {
            to,
            from,
            amount: amount.clone(),
            direction,
            timestamp: record.timestamp,
        });
    }

    let json = to_string_pretty(&sort_by_date(transactions))?;
    println!("{}", json);
    Ok(())
}
*/
fn extract_puzzle_hash_and_encode(solution: String) -> String {
    let re = Regex::new(r"0x[0-9a-fA-F]+").unwrap();
    let puzzle_hashes: Vec<String> = re
        .find_iter(&solution)
        .map(|m| m.as_str().to_owned())
        .collect();

    for hash in puzzle_hashes.clone() {
        let encoded_hash = encode_puzzle_hash(&hash, "xch").unwrap_or("error".to_string());
        println!("found wallet: {encoded_hash}");
    }
    String::from("testing")
}
fn sort_by_date(mut coin_records: Vec<Transaction>) -> Vec<Transaction> {
    coin_records.sort_by(|a, b| match (a.timestamp, b.timestamp) {
        (Some(a_date), Some(b_date)) => a_date.cmp(&b_date),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    });
    coin_records
}

async fn get_spends_by_height(client: &FullNodeClient, height: u64, address: &str) -> Result<()> {
    let coin_spends = client.get_block_spends_by_height(height).await?;
    for spend in coin_spends.iter() {
        let assembled = &spend.puzzle_reveal;
        let disassembled_solution = chia_node::util::disassemble_program(assembled).unwrap();
        if disassembled_solution.contains(address) {
            println!("Found wallet {address}");
        }
        /*
        let re = Regex::new(r"0x[0-9a-fA-F]+").unwrap();
        let hex_value = re
            .captures(&disassembled_solution)
            .and_then(|caps| caps.get(1).map(|m| m.as_str()))
            .unwrap_or_default();
            */
    }
    Ok(())
}
