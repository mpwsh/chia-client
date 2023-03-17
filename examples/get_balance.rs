use anyhow::Result;
use chia_node::fullnode::Client as FullNodeClient;
use chia_node::util::decode_puzzle_hash;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    //Configure your node
    let host = "192.168.1.31";
    let port: u16 = 8555;
    let key_path = Path::new("<path-to-fullnode-key>");
    let cert_path = Path::new("<path-to-fullnode-cert>");
    let client = FullNodeClient::new(host, port, key_path, cert_path).await?;

    //Insert your wallet address
    let puzzle_hash = decode_puzzle_hash("<your-wallet-address>")?;
    let response = client
        .get_coin_records_by_puzzle_hash(&puzzle_hash, None, None, Some(false))
        .await?;
    let balance_mojos: u64 = response.iter().map(|record| record.coin.amount).sum();
    let balance_xch: f64 = balance_mojos as f64 / 1_000_000_000_000.0;
    println!("Balance: {balance_xch:.12} XCH");
    Ok(())
}
