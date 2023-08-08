use anyhow::Result;
use chia_node::fullnode::Client as FullNodeClient;
use chia_node::fullnode::ConfigBuilder as FullNodeConfigBuilder;
use chia_node::util::{decode_puzzle_hash, mojo_to_xch};
use std::net::{Ipv4Addr, SocketAddr};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    //Configure your node
    let ip: Ipv4Addr = "127.0.0.1".parse().expect("Invalid IPv4 address");
    let port: u16 = 8555;
    let addr = SocketAddr::new(ip.into(), port);
    let key_path = Path::new("~/.chia/mainnet/config/ssl/full_node/private_full_node.key");
    let cert_path = Path::new("~/.chia/mainnet/config/ssl/full_node/private_full_node.crt");

    let config = FullNodeConfigBuilder::new()
        .addr(addr)
        .key_path(key_path)
        .cert_path(cert_path)
        .build()
        .expect("Failed to create FullNodeConfig");

    let client = FullNodeClient::new(config).await?;
    let address = "xch10n6l66hhx3qrx2ttdvaj54mmy2u63jvhzalj5t6d89npsl4psmvqtsq8fz";
    //Insert your wallet address
    let puzzle_hash = decode_puzzle_hash(address)?;
    let response = client
        .get_coin_records_by_puzzle_hash(&puzzle_hash, None, None, Some(false))
        .await?;
    let balance_mojos: u64 = response.iter().map(|record| record.coin.amount).sum();
    println!("Balance: {:.12} XCH", mojo_to_xch(balance_mojos));
    Ok(())
}
