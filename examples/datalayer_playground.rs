use anyhow::{anyhow, Result};
use chia_client::{datalayer, util::xch_to_mojo, ClientBuilder};
use std::{path::Path, thread::sleep};

#[tokio::main]
async fn main() -> Result<()> {
    //Use the full path for your certs.
    let key_path =
        Path::new("/Users/mpw/.chia/mainnet/config/ssl/data_layer/private_data_layer.key");
    let cert_path =
        Path::new("/Users/mpw/.chia/mainnet/config/ssl/data_layer/private_data_layer.crt");

    let client = ClientBuilder::new()
        .addr("127.0.0.1", 8562)
        .key_path(key_path)
        .cert_path(cert_path)
        .build()
        .await?;

    let client = datalayer::Rpc::init(client);
    let fee = 0.00005;

    /*
        let tx = match client.create_data_store(xch_to_mojo(fee)).await {
            Ok(tx) => tx,
            Err(e) => {
                println!("Error: {:?}", e);
                return Err(anyhow!("Error: {:?}", e));
            }
        };
        println!("Store created - Tx details: {:?}", tx);
        let mut store_ids: Vec<String> = Vec::new();
        while let Err(e) = client.get_owned_stores().await {
            sleep(std::time::Duration::from_millis(1000));
            println!(
                "Waiting for store creation to be confirmed.\nConfirmed stores: {:?} | errors: {e}",
                store_ids
            );
        }
    */
    let store_id = "1eb80b2c4c4e505cb61c792af654d8567cb978fd52d030b9d5ce7260da78b30a";
    /*
    if let Ok(stores) = client.get_owned_stores().await {
        store_ids = stores;
    println!("Got store ids: {store_ids:?}");
    }*/
    //Insert a Key/value on a specific store_id
    let key = "0x0001";
    let value = "fadedcab";
    /*
    let insert_tx_id = client.insert(&store_id, key, value).await?;

    println!("{:?}", insert_tx_id);
    */
    //Retrieve it
    let root_hash = match client.get_root(&store_id).await?.hash {
        Some(hash) => hash,
        None => return Err(anyhow!("No root hash found")),
    };

    println!("Using root hash: {root_hash}");

    //let retrieved_value = client.get_value(&store_id, key, &root_hash).await?;
    //println!("{retrieved_value}");

    //get missing files (pull data)
    //let data = client.add_missing_files().await?;
    //println!("{data:?}");

    //Get sync status
    let sync_status = client.get_sync_status(store_id).await?;
    println!("{sync_status:?}");
    //get all values
    let key_values = client.get_keys_values(&store_id, Some(&root_hash)).await?;
    println!("{key_values:?}");
    Ok(())
}
