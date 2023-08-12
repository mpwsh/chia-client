use anyhow::{anyhow, Result};
use chia_client::{datalayer, util::xch_to_mojo, ClientBuilder};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    //Use the full path for your certs.
    let key_path =
        Path::new("/Users/mpw/projects/chia/chia-node-rs/data_layer/private_data_layer.key");
    let cert_path =
        Path::new("/Users/mpw/projects/chia/chia-node-rs/data_layer/private_data_layer.crt");

    let client = ClientBuilder::new()
        .addr("192.168.2.74", 8562)
        .key_path(key_path)
        .cert_path(cert_path)
        .build()
        .await?;

    let client = datalayer::Rpc::init(client);
    let fee = 0.00005;

    let tx = match client.create_data_store(xch_to_mojo(fee)).await {
        Ok(tx) => tx,
        Err(e) => {
            println!("Error: {:?}", e);
            return Err(anyhow!("Error: {:?}", e));
        }
    };
    println!("Store created - Tx details: {:?}", tx);

    let store_id = client.get_owned_stores().await?[0].clone();
    println!("{store_id}");

    //Insert a Key/value on a specific store_id
    let key = "0x0001";
    let value = "fadedcab";
    let insert_tx_id = client.insert(&store_id, key, value).await?;

    println!("{:?}", insert_tx_id);
    //Retrieve it
    let root_hash = match client.get_root(&store_id).await?.hash {
        Some(hash) => hash,
        None => return Err(anyhow!("No root hash found")),
    };

    println!("Using root hash: {root_hash}");

    let retrieved_value = client.get_value(&store_id, key, &root_hash).await?;
    println!("{retrieved_value}");

    //get all values
    let key_values = client.get_keys_values(&store_id, Some(&root_hash)).await?;
    println!("{key_values:?}");

    Ok(())
}
