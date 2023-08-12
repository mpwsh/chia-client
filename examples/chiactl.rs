use anyhow::Result;
use chia_client::{
    fullnode,
    models::fullnode::MemPoolItem,
    util::{decode_puzzle_hash, encode_puzzle_hash, mojo_to_xch},
    Client, ClientBuilder,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::{
    cmp::Ordering,
    collections::HashSet,
    fs,
    path::PathBuf,
    thread,
    time::{Duration, Instant},
};
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemPool {
    height: u64,
    size: usize,
    cost: f64,
    fees: f64,
    additions: f64,
    removals: f64,
    duration: String,
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
    #[structopt(name = "mempool")]
    MemPool {
        #[structopt(long = "continuous")]
        continuous: bool,
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
    pub async fn load_config(&self) -> Result<Client> {
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

        Ok(ClientBuilder::new()
            .addr(&host, port)
            .key_path(key_path)
            .cert_path(cert_path)
            .build()
            .await?)
    }
}

impl MemPool {
    pub fn new(height: u64) -> Self {
        Self {
            height,
            size: 0,
            cost: 0.0,
            additions: 0.0,
            removals: 0.0,
            fees: 0.0,
            duration: String::new(),
        }
    }

    pub fn update(&mut self, item: &MemPoolItem) {
        self.size += 1;
        self.cost += mojo_to_xch(item.cost);
        self.fees += mojo_to_xch(item.fee);
        self.additions += mojo_to_xch(item.additions.iter().map(|coin| coin.amount).sum());
        self.removals += mojo_to_xch(
            item.removals
                .clone()
                .unwrap_or(Vec::new())
                .iter()
                .map(|coin| coin.amount)
                .sum(),
        );
    }
}
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::from_args();
    let client = fullnode::Rpc::init(cli.load_config().await?);
    match cli.cmd {
        Command::Get { subcommand } => match subcommand {
            GetSubcommand::Balance { address } => get_balance(&client, address).await?,
            GetSubcommand::NetworkInfo => get_network_info(&client).await?,
            GetSubcommand::MemPool { continuous } => get_mempool(&client, continuous).await?,
            GetSubcommand::BlockchainState => get_blockchain_state(&client).await?,
            GetSubcommand::BlockCountMetrics => get_block_count_metrics(&client).await?,
            GetSubcommand::Block { value } => get_block(&client, value).await?,
            GetSubcommand::Coin {
                value,
                show_parent,
                encode,
                prefix,
            } => get_coin(&client, value, show_parent, encode, prefix).await?,
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

async fn get_block(client: &fullnode::Rpc, value: String) -> Result<()> {
    let response = match value.parse::<u64>() {
        Ok(height) => client.get_block_by_height(height).await,
        Err(_) => client.get_block(&value).await,
    };

    let json = to_string_pretty(&response?)?;
    println!("{}", json);
    Ok(())
}

async fn get_coin(
    client: &fullnode::Rpc,
    value: String,
    show_parent: bool,
    encode: bool,
    prefix: String,
) -> Result<()> {
    let response = client.get_coin_record_by_name(&value).await?;
    let mut coin_record = response.clone();
    if encode {
        let prefix = "xch";
        coin_record.coin.puzzle_hash = encode_puzzle_hash(&response.coin.puzzle_hash, prefix)?;
    }

    let json = to_string_pretty(&coin_record)?;
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

        let parent_json = to_string_pretty(&parent_coin_record)?;
        println!("\nParent Coin:");
        println!("{}", parent_json);
    }

    Ok(())
}

async fn get_balance(client: &fullnode::Rpc, address: String) -> Result<()> {
    let puzzle_hash = decode_puzzle_hash(&address)?;
    let response = client
        .get_coin_records_by_puzzle_hash(&puzzle_hash, None, None, Some(false))
        .await?;
    let balance_mojos: u64 = response.iter().map(|record| record.coin.amount).sum();
    println!("Balance: {:.12} XCH", mojo_to_xch(balance_mojos));
    Ok(())
}

async fn get_network_info(client: &fullnode::Rpc) -> Result<()> {
    let res = client.get_network_info().await?;
    let json = to_string_pretty(&res)?;
    println!("{}", json);
    Ok(())
}

async fn get_mempool(client: &fullnode::Rpc, continuous: bool) -> Result<()> {
    let mut initial_height = client.get_blockchain_state().await?.peak.height;
    let mut seen_items = HashSet::new();
    let mut mempool = MemPool::new(initial_height);
    let mut start_time = Instant::now();
    loop {
        let current_height = client.get_blockchain_state().await?.peak.height;

        if current_height != initial_height {
            mempool.duration = format!("{:.2}", start_time.elapsed().as_secs_f64());
            println!("{}", to_string_pretty(&mempool)?);
            initial_height = current_height;
            seen_items.clear();
            mempool = MemPool::new(initial_height);
            start_time = Instant::now();
            if !continuous {
                break;
            }
        }

        let items = client.get_all_mempool_items().await?;

        for (key, item) in items.iter() {
            if seen_items.insert(key.clone()) {
                mempool.update(item);
            }
        }

        thread::sleep(Duration::from_millis(300));
    }

    Ok(())
}

async fn get_blockchain_state(client: &fullnode::Rpc) -> Result<()> {
    let res = client.get_blockchain_state().await?;
    let json = to_string_pretty(&res)?;
    println!("{}", json);
    Ok(())
}

async fn get_block_count_metrics(client: &fullnode::Rpc) -> Result<()> {
    let res = client.get_block_count_metrics().await?;
    let json = to_string_pretty(&res)?;
    println!("{}", json);
    Ok(())
}
async fn get_transactions(client: &fullnode::Rpc, address: String) -> Result<()> {
    let puzzle_hash = decode_puzzle_hash(&address)?;
    let prefix = "xch";
    let response = client
        .get_coin_records_by_puzzle_hash(&puzzle_hash, None, None, Some(true))
        .await?;
    let mut transactions: Vec<Transaction> = Vec::new();

    for record in response {
        let parent_record = client
            .get_coin_record_by_name(&record.coin.parent_coin_info)
            .await?;
        let amount = Amount {
            xch: mojo_to_xch(record.coin.amount),
            mojo: record.coin.amount,
        };
        let mut transaction = Transaction {
            coin: record.coin.parent_coin_info,
            recipient: address.clone(),
            sender: encode_puzzle_hash(&parent_record.coin.puzzle_hash, prefix)?,
            amount: amount.clone(),
            confirmed_height: record.confirmed_block_index,
            spent_height: record.spent_block_index,
            direction: Direction::Sent,
            timestamp: record.timestamp,
        };
        if parent_record.coin.puzzle_hash == puzzle_hash {
            transaction.direction = Direction::Sent;
        } else {
            transaction.direction = Direction::Received;
        }
        transactions.push(transaction);
    }

    let json = to_string_pretty(&sort_by_date(transactions))?;
    println!("{}", json);
    Ok(())
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
