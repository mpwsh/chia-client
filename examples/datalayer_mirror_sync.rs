use std::{
    path::Path,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use anyhow::Result;
use chia_client::{
    datalayer,
    util::{xch_to_mojo, Elapsed},
    wallet, ClientBuilder,
};

struct Participant {
    name: String,
    store_id: String,
    mirror_url: String,
    clients: Clients,
    mirrors: Vec<String>,
}

struct Clients {
    wallet: wallet::Rpc,
    datalayer: datalayer::Rpc,
}

#[tokio::main]
async fn main() -> Result<()> {
    let fee = 0.00005;

    let mut alice = Participant {
        name: "Alice".to_string(),
        store_id: "d1e0190388d414a5073f8d5d70d234e4f71d25d909748b5cb1e8ec8ac715f2a7".to_string(),
        mirror_url: "http://chia-node1:8575".to_string(),
        clients: Clients {
            wallet: setup_wallet("node1", "127.0.0.1", 9250).await?,
            datalayer: setup_datalayer("node1", "127.0.0.1", 8560).await?,
        },
        mirrors: Vec::new(),
    };

    let mut bob = Participant {
        name: "Bob".to_string(),
        store_id: "05915a7ec111ff8070d380e71654d74d5e95418646b264eb2b94eca3541dfe54".to_string(),
        mirror_url: "http://chia-node2:8575".to_string(),
        clients: Clients {
            wallet: setup_wallet("node2", "127.0.0.1", 9251).await?,
            datalayer: setup_datalayer("node2", "127.0.0.1", 8561).await?,
        },
        mirrors: Vec::new(),
    };

    println!("Verifying both data stores are ready to be used");
    wait_for_sync(&alice.name, &alice.clients.datalayer, &alice.store_id).await?;
    wait_for_sync(&bob.name, &bob.clients.datalayer, &bob.store_id).await?;

    println!("Both data stores are now synced");

    alice.mirrors = alice
        .clients
        .datalayer
        .get_mirrors(&alice.store_id)
        .await?
        .into_iter()
        .flat_map(|m| m.urls)
        .collect();

    bob.mirrors = bob
        .clients
        .datalayer
        .get_mirrors(&bob.store_id)
        .await?
        .into_iter()
        .flat_map(|m| m.urls)
        .collect();

    println!("Getting mirror lists:");
    println!("Bob mirrors: {:?}", bob.mirrors);
    println!("Alice mirrors: {:?}", alice.mirrors);

    if !alice.mirrors.iter().any(|u| u == &bob.mirror_url) {
        println!("Adding bob as mirror of Alices data store");
        alice
            .clients
            .datalayer
            .add_mirror(
                &alice.store_id,
                vec![&bob.mirror_url, &alice.mirror_url],
                xch_to_mojo(fee),
            )
            .await?;
    } else {
        println!("bob is already a mirror of Alices data store. skipping")
    }
    if !bob.mirrors.iter().any(|u| u == &alice.mirror_url) {
        println!("Adding Alice as mirror of Bobs data store");
        bob.clients
            .datalayer
            .add_mirror(
                &bob.store_id,
                vec![&bob.mirror_url, &alice.mirror_url],
                xch_to_mojo(fee),
            )
            .await?;
    } else {
        println!("Alice is already a mirror of Bobs data store. skipping")
    }

    println!("Writing data into alice's store");

    let value = "fadedcab";

    let tx_id = alice
        .clients
        .datalayer
        .insert(
            &alice.store_id,
            &generate_key(&alice.store_id),
            value,
            xch_to_mojo(fee),
        )
        .await?;

    println!("Wrote data successfully with TX: {tx_id}");

    wait_for_confirmation(&alice.clients.wallet, &tx_id).await?;

    println!("Writing data into bobs's store");

    let tx_id = bob
        .clients
        .datalayer
        .insert(
            &bob.store_id,
            &generate_key(&bob.store_id),
            value,
            xch_to_mojo(fee),
        )
        .await?;

    wait_for_confirmation(&bob.clients.wallet, &tx_id).await?;
    println!("Wrote data successfully with TX: {tx_id}");

    println!("Waiting for both clients to sync the new data");
    wait_for_sync(&alice.name, &alice.clients.datalayer, &bob.store_id).await?;
    wait_for_sync(&bob.name, &bob.clients.datalayer, &alice.store_id).await?;
    println!("Done!");

    Ok(())
}

async fn wait_for_confirmation(client: &wallet::Rpc, tx_id: &str) -> Result<()> {
    let started = Instant::now();
    println!("Checking transaction status of tx: {tx_id}");
    loop {
        if let Ok(tx) = client.get_transaction(tx_id).await {
            if tx.confirmed {
                break;
            }
        } else {
            tokio::time::sleep(Duration::from_secs(2)).await;
            println!(
                "Error while getting transaction status.\nResponse: {:?}",
                client.get_transaction(tx_id).await.unwrap()
            );
        }
    }
    println!("Transaction {} confirmed!", tx_id);
    println!("Confirmation took: {}", Elapsed::from(&started));
    Ok(())
}

async fn wait_for_sync(name: &str, client: &datalayer::Rpc, store_id: &str) -> Result<()> {
    let started = Instant::now();
    println!("Checking sync status of {name}'s store_ID: {store_id}");
    loop {
        if let Ok(sync_status) = client.get_sync_status(store_id).await {
            if sync_status.generation == sync_status.target_generation {
                break;
            }
        } else {
            tokio::time::sleep(Duration::from_secs(2)).await;
            println!("New data not found. Waiting a bit more...");
        }
    }
    println!("Sync took: {}", Elapsed::from(&started));
    println!("Store {} is now synced!", store_id);
    Ok(())
}

fn generate_key(store_id: &str) -> String {
    let start_of_key = "0x";
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let timestamp = since_the_epoch.as_secs();

    let hashed_store_id = format!("{:x}", hash(store_id));
    format!("{}{}{:x}", start_of_key, hashed_store_id, timestamp)
}

fn hash(s: &str) -> u64 {
    let mut h = 0u64;
    for c in s.chars() {
        h = h.wrapping_shl(5).wrapping_sub(h).wrapping_add(c as u64);
    }
    h
}

async fn setup_wallet(node: &str, ip: &str, port: u16) -> Result<wallet::Rpc> {
    let key = format!(
        "/Users/mpw/projects/chia/chia-client/data/{node}/config/ssl/wallet/private_wallet.key"
    );
    let cert = format!(
        "/Users/mpw/projects/chia/chia-client/data/{node}/config/ssl/wallet/private_wallet.crt"
    );
    let key_path = Path::new(&key);
    let cert_path = Path::new(&cert);

    let config = ClientBuilder::new()
        .addr(ip, port)
        .key_path(key_path)
        .cert_path(cert_path)
        .build()
        .await?;

    Ok(wallet::Rpc::init(config))
}

async fn setup_datalayer(node: &str, ip: &str, port: u16) -> Result<datalayer::Rpc> {
    let key = format!(
        "/Users/mpw/projects/chia/chia-client/data/{node}/config/ssl/data_layer/private_data_layer.key"
    );
    let cert = format!(
        "/Users/mpw/projects/chia/chia-client/data/{node}/config/ssl/data_layer/private_data_layer.crt"
    );
    let key_path = Path::new(&key);
    let cert_path = Path::new(&cert);

    let config = ClientBuilder::new()
        .addr(ip, port)
        .key_path(key_path)
        .cert_path(cert_path)
        .build()
        .await?;

    Ok(datalayer::Rpc::init(config))
}
