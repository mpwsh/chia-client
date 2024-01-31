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
use env_logger::{Builder, WriteStyle};
use log::{error, info, LevelFilter};
struct App {
    participants: Vec<Participant>,
}

impl App {
    pub fn new() -> Self {
        Self {
            participants: Vec::new(),
        }
    }

    pub fn add_participant(&mut self, p: Participant) -> &Self {
        self.participants.push(p);
        self
    }
}
#[derive(Clone)]
struct Participant {
    name: String,
    store_id: String,
    mirror_url: String,
    clients: Clients,
    fee: u64,
    mirrors: Vec<String>,
    subscriptions: Vec<String>,
}

#[derive(Clone)]
struct Clients {
    wallet: wallet::Rpc,
    datalayer: datalayer::Rpc,
}

#[tokio::main]
async fn main() -> Result<()> {
    Builder::new()
        .write_style(WriteStyle::Always)
        .filter_level(LevelFilter::Info)
        .init();

    let mut app = App::new();
    let fee = xch_to_mojo(0.0000005);

    let alice = Participant {
        name: "Alice".to_string(),
        store_id: "923bf39ca5511fad813680943773d597c3c1e3608e4fcc93d2bff600e0bcf937".to_string(),
        mirror_url: "http://tun.hashlink.ch:9090".to_string(),
        mirror_url: "http://chia-node1:8575".to_string(),
        clients: Clients {
            wallet: setup_wallet("node1", "127.0.0.1", 9250).await?,
            datalayer: setup_datalayer("node1", "127.0.0.1", 8560).await?,
        },
        mirrors: Vec::new(),
        fee,
        subscriptions: Vec::new(),
    };

    app.add_participant(alice);

    let bob = Participant {
        name: "Bob".to_string(),
        store_id: "05915a7ec111ff8070d380e71654d74d5e95418646b264eb2b94eca3541dfe54".to_string(),
        mirror_url: "http://chia-node2:8575".to_string(),
        clients: Clients {
            wallet: setup_wallet("node2", "127.0.0.1", 9251).await?,
            datalayer: setup_datalayer("node2", "127.0.0.1", 8561).await?,
        },
        mirrors: Vec::new(),
        fee,
        subscriptions: Vec::new(),
    };

    app.add_participant(bob);

    info!(
        "Creating a data store for participants without an owned data store compatible with the app"
    );
    for i in 0..app.participants.len() {
        let participant = &mut app.participants[i];
        if participant.store_id.is_empty() {
            participant.store_id = match participant
                .clients
                .datalayer
                .create_data_store(participant.fee)
                .await?
                .id
            {
                Some(id) => id,
                None => {
                    panic!("Error while creating data store for {}", participant.name);
                },
            };
        }
    }

    info!("Checking if participants have enough balance to run all operations.");
    for participant in &app.participants {
        let balance = participant.clients.wallet.get_wallet_balance(1).await?;
        if balance.spendable_balance < (participant.fee * 3) as i64 {
            error!(
                "{}'s balance is too low. Please top up the wallet with at least {} mojos",
                participant.name,
                participant.fee * 3
            );
        }
    }

    info!("Verifying both data stores are ready to be used");
    for participant in &app.participants {
        wait_for_sync(
            &participant.name,
            &participant.clients.datalayer,
            &participant.store_id,
        )
        .await?;
    }

    info!("Handling subscriptions between participants");

    for i in 0..app.participants.len() {
        // Fetch current subscriptions for the participant
        let subscriptions = app.participants[i]
            .clients
            .datalayer
            .subscriptions()
            .await?;
        app.participants[i].subscriptions = subscriptions.clone();

        // Temporary collection to store new subscriptions
        let mut new_subs = Vec::new();

        for j in 0..app.participants.len() {
            if i != j
                && !app.participants[i]
                    .subscriptions
                    .iter()
                    .any(|u| u == &app.participants[j].store_id)
            {
                info!(
                    "Subscribing {} to {}'s store",
                    app.participants[i].name, app.participants[j].name
                );
                app.participants[i]
                    .clients
                    .datalayer
                    .subscribe(&app.participants[j].store_id, vec![])
                    .await?;

                // Collect new subscription
                new_subs.push(app.participants[j].store_id.clone());
            }
        }

        // Update the participant's subscriptions with new subscriptions
        app.participants[i].subscriptions.extend(new_subs);
    }

    // Collect store IDs and names before the mutable borrow
    let participant_info: Vec<(Clients, String)> = app
        .participants
        .iter()
        .map(|p| (p.clients.clone(), p.name.clone()))
        .collect();

    for i in 0..app.participants.len() {
        let participant = &mut app.participants[i];
        participant.mirrors = participant
            .clients
            .datalayer
            .get_mirrors(&participant.store_id)
            .await?
            .into_iter()
            .flat_map(|m| m.urls)
            .collect();

        info!("{} mirrors: {:?}", participant.name, participant.mirrors);

        if !participant
            .mirrors
            .iter()
            .any(|u| u == &participant.mirror_url)
        {
            info!("Adding {} DataLayer address as mirror", participant.name);
            participant
                .clients
                .datalayer
                .add_mirror(
                    &participant.store_id,
                    vec![&participant.mirror_url],
                    0,
                    participant.fee,
                )
                .await?;
        } else {
            info!(
                "{} is already a mirror of their own data store. Skipping",
                participant.name
            );
        }

        info!("Inserting random data into {} store", participant.name);
        //let value = "fadedcab";
        let single_char = 'a';
        let size = 20 * 1_000_000;
        let value: String = std::iter::repeat(single_char).take(size).collect();
        let tx_id = participant
            .clients
            .datalayer
            .insert(
                &participant.store_id,
                &generate_key(&participant.store_id),
                &value,
                participant.fee,
            )
            .await?;

        wait_for_confirmation(&participant.clients.wallet, &tx_id).await?;
        info!("Wrote data successfully with TX: {tx_id}");

        for (_, (other_clients, other_name)) in participant_info.iter().enumerate() {
            wait_for_sync(
                &participant.name,
                &other_clients.datalayer,
                &participant.store_id,
            )
            .await?;
            info!(
                "{}'s store is now synced with {}'s store",
                participant.name, other_name
            );
        }
    }

    info!("Done!");
    Ok(())
}

async fn wait_for_confirmation(client: &wallet::Rpc, tx_id: &str) -> Result<()> {
    let started = Instant::now();
    info!("Checking transaction status of tx: {tx_id}");
    loop {
        if let Ok(tx) = client.get_transaction(tx_id).await {
            if tx.confirmed {
                break;
            }
        } else {
            error!(
                "Error while getting transaction status.\nResponse: {:?}",
                client.get_transaction(tx_id).await.unwrap()
            );
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    info!("Transaction {} confirmed!", tx_id);
    info!("Confirmation took: {}", Elapsed::from(&started));
    Ok(())
}

async fn wait_for_sync(name: &str, client: &datalayer::Rpc, store_id: &str) -> Result<()> {
    let started = Instant::now();
    info!("Checking sync status of {name}'s DataLayer Client of store_id: {store_id}");
    loop {
        if let Ok(sync_status) = client.get_sync_status(store_id).await {
            if sync_status.generation == sync_status.target_generation {
                break;
            }
        }
    }
    tokio::time::sleep(Duration::from_secs(2)).await;
    info!("Sync took: {}", Elapsed::from(&started));
    info!("Store {} is now synced!", store_id);
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
