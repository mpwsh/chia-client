use anyhow::Result;
<<<<<<< HEAD
use chia_client::{
    fullnode,
    util::{decode_puzzle_hash, mojo_to_xch},
    ClientBuilder,
};
||||||| parent of a2d8e39 (introduce datalayer api, migrate models and rename crate to chia-client)
use chia_node::fullnode::Client as FullNodeClient;
use chia_node::fullnode::ConfigBuilder as FullNodeConfigBuilder;
use chia_node::util::{decode_puzzle_hash, mojo_to_xch};
use std::net::{Ipv4Addr, SocketAddr};
=======
use chia_node::fullnode::Client;
use chia_node::fullnode::ConfigBuilder;
use chia_node::util::{decode_puzzle_hash, mojo_to_xch};
use std::net::{Ipv4Addr, SocketAddr};
>>>>>>> a2d8e39 (introduce datalayer api, migrate models and rename crate to chia-client)
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    //Use the full path for your certs.
    let key_path = Path::new("/Users/mpw/.chia/mainnet/config/ssl/full_node/private_full_node.key");
    let cert_path =
        Path::new("/Users/mpw/.chia/mainnet/config/ssl/full_node/private_full_node.crt");

<<<<<<< HEAD
    let client = ClientBuilder::new()
        .addr("127.0.0.1", 8555)
||||||| parent of a2d8e39 (introduce datalayer api, migrate models and rename crate to chia-client)
    let config = FullNodeConfigBuilder::new()
        .addr(addr)
=======
    let config = ConfigBuilder::new()
        .addr(addr)
>>>>>>> a2d8e39 (introduce datalayer api, migrate models and rename crate to chia-client)
        .key_path(key_path)
        .cert_path(cert_path)
        .build()
<<<<<<< HEAD
        .await?;

    let node = fullnode::Rpc::init(client);
||||||| parent of a2d8e39 (introduce datalayer api, migrate models and rename crate to chia-client)
        .expect("Failed to create FullNodeConfig");
=======
        .expect("Failed to create Config");
>>>>>>> a2d8e39 (introduce datalayer api, migrate models and rename crate to chia-client)

<<<<<<< HEAD
||||||| parent of a2d8e39 (introduce datalayer api, migrate models and rename crate to chia-client)
    let client = FullNodeClient::new(config).await?;
=======
    let client = Client::new(config).await?;
>>>>>>> a2d8e39 (introduce datalayer api, migrate models and rename crate to chia-client)
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
