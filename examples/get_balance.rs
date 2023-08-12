use anyhow::Result;
use chia_client::{
    fullnode,
    util::{decode_puzzle_hash, mojo_to_xch},
    ClientBuilder,
};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    //Use the full path for your certs.
    let key_path = Path::new("/Users/mpw/.chia/mainnet/config/ssl/full_node/private_full_node.key");
    let cert_path =
        Path::new("/Users/mpw/.chia/mainnet/config/ssl/full_node/private_full_node.crt");

    let client = ClientBuilder::new()
        .addr("127.0.0.1", 8555)
        .key_path(key_path)
        .cert_path(cert_path)
        .build()
        .daemon(Farmer)
        .await?;

    let node = fullnode::Rpc::init(client);

    let address = "xch10n6l66hhx3qrx2ttdvaj54mmy2u63jvhzalj5t6d89npsl4psmvqtsq8fz";
    //Insert your wallet address
    let puzzle_hash = decode_puzzle_hash(address)?;
    let response = node
        .get_coin_records_by_puzzle_hash(&puzzle_hash, None, None, Some(false))
        .await?;
    let balance_mojos: u64 = response.iter().map(|record| record.coin.amount).sum();
    println!("Balance: {:.12} XCH", mojo_to_xch(balance_mojos));
    Ok(())
}
