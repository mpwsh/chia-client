use anyhow::{anyhow, Result};
use chia_client::{datalayer, util::xch_to_mojo, ClientBuilder};
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use std::{
    fmt,
    path::Path,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

pub struct Elapsed(Duration);
impl Elapsed {
    pub fn from(start: &Instant) -> Self {
        Elapsed(start.elapsed())
    }
}

impl fmt::Display for Elapsed {
    fn fmt(&self, out: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match (self.0.as_secs(), self.0.subsec_nanos()) {
            (0, n) if n < 1000 => write!(out, "{} ns", n),
            (0, n) if n < 1_000_000 => write!(out, "{} µs", n / 1000),
            (0, n) => write!(out, "{} ms", n / 1_000_000),
            (s, n) if s < 10 => write!(out, "{}.{:02} s", s, n / 10_000_000),
            (s, _) => write!(out, "{} s", s),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let theme = ColorfulTheme::default();
    let client = setup().await?;

    let fee = 0.00005;
    let store_id: String = Input::with_theme(&theme)
        .with_prompt("Please input your store_id (or type 'new' to create a new one)")
        .interact_text()
        .unwrap();

    // If the user wants a new store, create one
    let store_id = if store_id == "new" {
        let id = match client.create_data_store(xch_to_mojo(fee)).await {
            Ok(res) => res.id.unwrap(),
            Err(e) => {
                println!("Error: {:?}", e);
                return Err(anyhow!("Error: {:?}", e));
            }
        };
        println!("Please share this Store ID with your counterpart: {}", id);
        id
    } else {
        store_id
    };
    wait_for_sync(&client, &store_id).await?;

    // Get counter part store_id from the user
    let counter_part_store_id: String = Input::with_theme(&theme)
        .with_prompt("Please input your counter part store_id")
        .interact_text()
        .unwrap();

    client.subscribe(&counter_part_store_id, Vec::new()).await?;
    // Wait for the counterpart store to be synced
    wait_for_sync(&client, &counter_part_store_id).await?;
    // Chat loop
    loop {
        // Display new messages from the counterpart
        let new_messages = client.get_kv_diff(&counter_part_store_id).await?;
        for message in new_messages {
            println!("{}: {:?}", counter_part_store_id, hex::decode(&message)?);
        }

        // Get a string message from the user and convert it to HEX
        let message: String = Input::with_theme(&theme)
            .with_prompt("You")
            .interact_text()
            .unwrap();

        let key = generate_key(&store_id); // Generate a key based on the store id
        let value = hex::encode(&message); // Convert the message to HEX

        // Insert the message
        let insert_tx_id = client.insert(&store_id, &key, &value).await?;
        println!("Message sent (Tx ID: {:?})", insert_tx_id);
        // Option to end the chat
        let continue_chat = Confirm::with_theme(&theme)
            .with_prompt("Do you want to continue chatting?")
            .default(true)
            .interact()
            .unwrap();

        if !continue_chat {
            break;
        }
    }

    println!("Goodbye!");
    Ok(())
}

// Example function that waits for a store to be synced (you might need to implement the actual logic based on your API)
async fn wait_for_sync(client: &datalayer::Rpc, store_id: &str) -> Result<()> {
    let started = Instant::now();

    loop {
        if let Ok(sync_status) = client.get_sync_status(store_id).await {
            if sync_status.generation == sync_status.target_generation {
                break;
            }
        } else {
            tokio::time::sleep(Duration::from_secs(2)).await;
            println!("Still waiting");
        }
    }
    println!("Sync took: {}", Elapsed::from(&started));
    println!("Store {} is now synced!", store_id);
    Ok(())
}

/// Generate a key based on the store_id.
/// This combines the hash of the store_id with the current timestamp.
fn generate_key(store_id: &str) -> String {
    let start_of_key = "0x";
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let timestamp = since_the_epoch.as_secs();

    let hashed_store_id = format!("{:x}", hash(store_id));
    format!("{}{}{:x}", start_of_key, hashed_store_id, timestamp)
}

/// Simple hash function for demonstration purposes.
/// In a real-world scenario, consider using a cryptographic hash function.
fn hash(s: &str) -> u64 {
    let mut h = 0;
    for c in s.chars() {
        h = (h << 5) - h + c as u64;
    }
    h
}

async fn setup() -> Result<datalayer::Rpc> {
    let key_path = Path::new("/Users/mpw/projects/chia/chia-client/client1/private_data_layer.key");
    let cert_path =
        Path::new("/Users/mpw/projects/chia/chia-client/client1/private_data_layer.crt");

    let config = ClientBuilder::new()
        .addr("192.168.1.31", 8562)
        .key_path(key_path)
        .cert_path(cert_path)
        .build()
        .await?;

    Ok(datalayer::Rpc::init(config))
}
