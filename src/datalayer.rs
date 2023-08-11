<<<<<<< HEAD
use crate::prelude::*;

pub struct Rpc {
    pub client: Client,
}
impl Rpc {
    pub fn init(client: Client) -> Self {
        Self { client }
    }

    pub async fn add_mirror(&self, id: &str, urls: Vec<&str>, amount: u64) -> Result<()> {
        let json = json!({
            "id": id,
            "urls": urls,
            "amount": amount,
        });
        let res: BasicResponse = self
            .client
            .cmd("add_mirror", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.error {
            Some(e) => Err(anyhow!("{:#?}", e)),
            None => Ok(()),
        }
    }

    pub async fn add_missing_files(&self) -> Result<()> {
        let res: BasicResponse = self
            .client
            .cmd("add_missing_files", None)
            .await?
            .json()
            .await?;
        match res.error {
            Some(e) => Err(anyhow!("{:#?}", e)),
            None => Ok(()),
        }
    }

    pub async fn get_owned_stores(&self) -> Result<Vec<String>> {
        let res: StoresResponse = self
            .client
            .cmd("get_owned_stores", None)
            .await?
            .json()
            .await?;
        match res.store_ids {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }

    pub async fn close_connection(&self) -> Result<Vec<String>> {
        Ok(self
            .client
            .cmd("close_connection", None)
            .await?
            .json()
            .await?)
    }

    pub async fn get_connections(&self) -> Result<Vec<Connection>> {
        let res: ConnectionsResponse = self
            .client
            .cmd("get_connections", None)
            .await?
            .json()
            .await?;
        match res.connections {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }

    pub async fn open_connection(&self, host: &str, port: u64) -> Result<()> {
        let json = json!({
            "host": host,
            "port": port.to_string(),
        });
        let res: BasicResponse = self
            .client
            .cmd("batch_update", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.error {
            Some(e) => Err(anyhow!("{:#?}", e)),
            None => Ok(()),
        }
    }

    pub async fn stop_node(&self) -> Result<()> {
        let res: BasicResponse = self.client.cmd("stop_node", None).await?.json().await?;
        match res.error {
            Some(e) => Err(anyhow!("{:#?}", e)),
            None => Ok(()),
        }
    }

    pub async fn get_routes(&self) -> Result<Vec<String>> {
        let res: RoutesResponse = self.client.cmd("get_routes", None).await?.json().await?;
        match res.routes {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }

    pub async fn batch_update(&self, id: &str, changelist: Vec<ChangeListEntry>) -> Result<String> {
        let json = json!({
            "id": id,
            "changelist": changelist,
        });
        let res: UpdateResponse = self
            .client
            .cmd("batch_update", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.tx_id {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }

    pub async fn cancel_offer(&self, trade_id: &str, secure: bool, fee: u64) -> Result<()> {
        let json = json!({
            "trade_id": trade_id,
            "secure": secure,
            "fee": fee.to_string(),
        });
        let res: BasicResponse = self
            .client
            .cmd("cancel_offer", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.error {
            Some(e) => Err(anyhow!("{:#?}", e)),
            None => Ok(()),
        }
    }

    pub async fn create_data_store(&self, fee: u64) -> Result<Vec<Transaction>> {
        let json = json!({
            "fee": fee.to_string(),
        });
        let res: CreateDataStoreResponse = self
            .client
            .cmd("create_data_store", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.txs {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }

    pub async fn delete_key(&self, id: &str, key: &str, fee: u64) -> Result<String> {
        let json = json!({
            "id": id,
            "key": key,
            "fee": fee.to_string(),
        });
        let res: UpdateResponse = self
            .client
            .cmd("delete_key", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.tx_id {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }

    pub async fn delete_mirror(&self, id: &str) -> Result<()> {
        let json = json!({
            "id": id,
        });
        let res: BasicResponse = self
            .client
            .cmd("delete_mirror", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.error {
            Some(e) => Err(anyhow!("{:#?}", e)),
            None => Ok(()),
        }
    }
    pub async fn get_root(&self, id: &str) -> Result<RootResponse> {
        let json = json!({
            "id": id,
        });
        let res: RootResponse = self
            .client
            .cmd("get_root", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.hash {
            Some(_) => Ok(res),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }
    pub async fn get_root_history(&self, id: &str) -> Result<Vec<RootHistory>> {
        let json = json!({
            "id": id,
        });
        let res: RootHistoryResponse = self
            .client
            .cmd("get_root", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.root_history {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }

    pub async fn get_roots(&self, ids: Vec<&str>) -> Result<Vec<RootHash>> {
        let json = json!({
            "ids": ids,
        });
        let res: RootsResponse = self
            .client
            .cmd("get_roots", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.root_hashes {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }

    pub async fn get_ancestors(&self, id: &str, hash: &str) -> Result<Vec<String>> {
        let json = json!({
            "id": id,
            "hash": hash,
        });
        let res: AncestorsResponse = self
            .client
            .cmd("get_ancestors", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.ancestors {
            None => Err(anyhow!("{:#?}", res.error)),
            Some(r) => Ok(r),
        }
    }

    pub async fn get_keys(&self, id: &str, root_hash: &str) -> Result<Vec<String>> {
        let json = json!({
            "id": id,
            "root_hash": root_hash,
        });
        let res: KeysResponse = self
            .client
            .cmd("get_keys", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.keys {
            None => Err(anyhow!("{:#?}", res.error)),
            Some(r) => Ok(r),
        }
    }

    pub async fn get_kv_diff(&self, id: &str, hash_1: &str, hash_2: &str) -> Result<Vec<Diff>> {
        let json = json!({
            "id": id,
            "hash_1": hash_1,
            "hash_2": hash_2,
        });
        let res: KeyValueDiffResponse = self
            .client
            .cmd("get_kv_diff", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.diff {
            None => Err(anyhow!("{:#?}", res.error)),
            Some(r) => Ok(r),
        }
    }

    pub async fn get_local_root(&self, id: &str) -> Result<String> {
        let json = json!({
            "id": id,
        });
        let res: LocalRootResponse = self
            .client
            .cmd("get_local_root", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.hash {
            None => Err(anyhow!("{:#?}", res.error)),
            Some(r) => Ok(r),
        }
    }

    pub async fn get_mirrors(&self, id: &str) -> Result<Vec<Mirror>> {
        let json = json!({
            "id": id,
        });
        let res: MirrorsResponse = self
            .client
            .cmd("get_mirrors", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.mirrors {
            None => Err(anyhow!("{:#?}", res.error)),
            Some(r) => Ok(r),
        }
    }

    pub async fn get_sync_status(&self, id: &str) -> Result<SyncStatus> {
        let json = json!({
            "id": id,
        });
        let res: SyncStatusResponse = self
            .client
            .cmd("get_sync_status", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.sync_status {
            None => Err(anyhow!("{:#?}", res.error)),
            Some(r) => Ok(r),
        }
    }

    pub async fn get_keys_values(
        &self,
        id: &str,
        root_hash: Option<&str>,
    ) -> Result<Vec<KeysValue>> {
        let json = json!({
            "id": id,
            "root_hash": root_hash.unwrap_or(""),
        });
        let res: KeysValuesResponse = self
            .client
            .cmd("get_keys_values", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.keys_values {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }

    pub async fn get_value(&self, id: &str, key: &str, root_hash: &str) -> Result<String> {
        let json = json!({
            "id": id,
            "key": key,
            "root_hash": root_hash,
        });

        let res: ValueResponse = self
            .client
            .cmd("get_value", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.value {
            None => Err(anyhow!("{:#?}", res.error)),
            Some(r) => Ok(r),
        }
    }
    pub async fn insert(&self, store_id: &str, key: &str, value: &str) -> Result<String> {
        let json = json!({
            "id": store_id,
            "key": key,
            "value": value,
        });

        let res: UpdateResponse = self
            .client
            .cmd("insert", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.tx_id {
            None => Err(anyhow!("{:#?}", res.error)),
            Some(r) => Ok(r),
        }
    }
    pub async fn make_offer(&self, offer: Offer) -> Result<Offer> {
        let res: OfferResponse = self
            .client
            .cmd("make_offer", Some(serde_json::to_string(&offer)?))
            .await?
            .json()
            .await?;
        match res.offer {
            None => Err(anyhow!("{:#?}", res.error)),
            Some(r) => Ok(r),
        }
    }
    pub async fn take_offer(&self, offer: Offer) -> Result<String> {
        let res: TakeOfferResponse = self
            .client
            .cmd("take_offer", Some(serde_json::to_string(&offer)?))
            .await?
            .json()
            .await?;
        match res.trade_id {
            None => Err(anyhow!("{:#?}", res.error)),
            Some(r) => Ok(r),
        }
    }
    pub async fn verify_offer(&self, offer: Offer) -> Result<VerifyOfferResponse> {
        Ok(self
            .client
            .cmd("verify_offer", Some(serde_json::to_string(&offer)?))
            .await?
            .json()
            .await?)
    }
    pub async fn remove_subscriptions(&self, id: &str, urls: Vec<&str>) -> Result<()> {
        let json = json!({
            "id": id,
            "urls": urls,
        });

        let res: BasicResponse = self
            .client
            .cmd("remove_subscriptions", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.error {
            Some(e) => Err(anyhow!("{:#?}", e)),
            None => Ok(()),
        }
    }

    pub async fn subscribe(&self, id: &str, urls: Vec<&str>) -> Result<()> {
        let json = json!({
            "id": id,
            "urls": urls,
        });

        let res: BasicResponse = self
            .client
            .cmd("subscribe", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.error {
            Some(e) => Err(anyhow!("{:#?}", e)),
            None => Ok(()),
        }
    }
    pub async fn unsubscribe(&self, id: &str) -> Result<()> {
        let json = json!({
            "id": id,
        });

        let res: BasicResponse = self
            .client
            .cmd("subscribe", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.error {
            Some(e) => Err(anyhow!("{:#?}", e)),
            None => Ok(()),
        }
    }
    pub async fn subscriptions(&self) -> Result<Vec<String>> {
        let res: StoresResponse = self.client.cmd("subscriptions", None).await?.json().await?;
        match res.store_ids {
            None => Err(anyhow!("{:#?}", res.error)),
            Some(r) => Ok(r),
        }
    }
}
||||||| parent of a2d8e39 (introduce datalayer api, migrate models and rename crate to chia-client)
=======
use crate::prelude::*;

pub struct Config {
    pub addr: SocketAddr,
    pub key_path: PathBuf,
    pub cert_path: PathBuf,
}

impl Config {
    pub fn new(addr: SocketAddr, key_path: &Path, cert_path: &Path) -> Self {
        Self {
            addr,
            key_path: key_path.to_path_buf(),
            cert_path: cert_path.to_path_buf(),
        }
    }
}
pub struct ConfigBuilder {
    addr: Option<SocketAddr>,
    key_path: Option<PathBuf>,
    cert_path: Option<PathBuf>,
}
impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            addr: None,
            key_path: None,
            cert_path: None,
        }
    }

    pub fn addr(mut self, addr: SocketAddr) -> Self {
        self.addr = Some(addr);
        self
    }

    pub fn key_path<P: Into<PathBuf>>(mut self, key_path: P) -> Self {
        self.key_path = Some(key_path.into());
        self
    }

    pub fn cert_path<P: Into<PathBuf>>(mut self, cert_path: P) -> Self {
        self.cert_path = Some(cert_path.into());
        self
    }

    pub fn build(self) -> Result<Config, &'static str> {
        let addr = self.addr.ok_or("Address is required")?;
        let key_path = self.key_path.ok_or("Key path is required")?;
        let cert_path = self.cert_path.ok_or("Cert path is required")?;

        Ok(Config {
            addr,
            key_path,
            cert_path,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    addr: SocketAddr,
    http: reqwest::Client,
}

impl Client {
    pub async fn new(config: Config) -> Result<Self, Error> {
        let identity = load_pem_pair(config.key_path, config.cert_path).await?;
        let http = ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            //.danger_accept_invalid_hostnames(true)
            .identity(identity)
            .build()?;
        Ok(Self {
            addr: config.addr,
            http,
        })
    }

    pub async fn add_mirror(&self, id: &str, urls: Vec<&str>, amount: u64) -> Result<()> {
        let json = json!({
            "id": id,
            "urls": urls,
            "amount": amount,
        });
        let res: BasicResponse = self
            .cmd("add_mirror", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.error {
            Some(e) => Err(anyhow!("{:#?}", e)),
            None => Ok(()),
        }
    }

    pub async fn add_missing_files(&self) -> Result<()> {
        let res: BasicResponse = self.cmd("add_missing_files", None).await?.json().await?;
        match res.error {
            Some(e) => Err(anyhow!("{:#?}", e)),
            None => Ok(()),
        }
    }

    pub async fn get_owned_stores(&self) -> Result<Vec<String>> {
        let res: StoresResponse = self.cmd("get_owned_stores", None).await?.json().await?;
        match res.store_ids {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }

    pub async fn close_connection(&self) -> Result<Vec<String>> {
        Ok(self.cmd("close_connection", None).await?.json().await?)
    }

    pub async fn get_connections(&self) -> Result<Vec<Connection>> {
        let res: ConnectionsResponse = self.cmd("get_connections", None).await?.json().await?;
        match res.connections {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }

    pub async fn open_connection(&self, host: &str, port: u64) -> Result<()> {
        let json = json!({
            "host": host,
            "port": port.to_string(),
        });
        let res: BasicResponse = self
            .cmd("batch_update", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.error {
            Some(e) => Err(anyhow!("{:#?}", e)),
            None => Ok(()),
        }
    }

    pub async fn stop_node(&self) -> Result<()> {
        let res: BasicResponse = self.cmd("stop_node", None).await?.json().await?;
        match res.error {
            Some(e) => Err(anyhow!("{:#?}", e)),
            None => Ok(()),
        }
    }

    pub async fn get_routes(&self) -> Result<Vec<String>> {
        let res: RoutesResponse = self.cmd("get_routes", None).await?.json().await?;
        match res.routes {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }

    pub async fn batch_update(&self, id: &str, changelist: Vec<ChangeListEntry>) -> Result<String> {
        let json = json!({
            "id": id,
            "changelist": changelist,
        });
        let res: UpdateResponse = self
            .cmd("batch_update", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.tx_id {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }

    pub async fn cancel_offer(&self, trade_id: &str, secure: bool, fee: u64) -> Result<()> {
        let json = json!({
            "trade_id": trade_id,
            "secure": secure,
            "fee": fee.to_string(),
        });
        let res: BasicResponse = self
            .cmd("cancel_offer", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.error {
            Some(e) => Err(anyhow!("{:#?}", e)),
            None => Ok(()),
        }
    }

    pub async fn create_data_store(&self, fee: u64) -> Result<Vec<Transaction>> {
        let json = json!({
            "fee": fee.to_string(),
        });
        let res: CreateDataStoreResponse = self
            .cmd("create_data_store", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.txs {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }

    pub async fn delete_key(&self, id: &str, key: &str, fee: u64) -> Result<String> {
        let json = json!({
            "id": id,
            "key": key,
            "fee": fee.to_string(),
        });
        let res: UpdateResponse = self
            .cmd("delete_key", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.tx_id {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }

    pub async fn delete_mirror(&self, id: &str) -> Result<()> {
        let json = json!({
            "id": id,
        });
        let res: BasicResponse = self
            .cmd("delete_mirror", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.error {
            Some(e) => Err(anyhow!("{:#?}", e)),
            None => Ok(()),
        }
    }
    pub async fn get_root(&self, id: &str) -> Result<RootResponse> {
        let json = json!({
            "id": id,
        });
        let res: RootResponse = self
            .cmd("get_root", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.hash {
            Some(_) => Ok(res),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }
    pub async fn get_root_history(&self, id: &str) -> Result<Vec<RootHistory>> {
        let json = json!({
            "id": id,
        });
        let res: RootHistoryResponse = self
            .cmd("get_root", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.root_history {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }

    pub async fn get_roots(&self, ids: Vec<&str>) -> Result<Vec<RootHash>> {
        let json = json!({
            "ids": ids,
        });
        let res: RootsResponse = self
            .cmd("get_roots", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.root_hashes {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }

    pub async fn get_ancestors(&self, id: &str, hash: &str) -> Result<Vec<String>> {
        let json = json!({
            "id": id,
            "hash": hash,
        });
        let res: AncestorsResponse = self
            .cmd("get_ancestors", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.ancestors {
            None => Err(anyhow!("{:#?}", res.error)),
            Some(r) => Ok(r),
        }
    }

    pub async fn get_keys(&self, id: &str, root_hash: &str) -> Result<Vec<String>> {
        let json = json!({
            "id": id,
            "root_hash": root_hash,
        });
        let res: KeysResponse = self
            .cmd("get_keys", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.keys {
            None => Err(anyhow!("{:#?}", res.error)),
            Some(r) => Ok(r),
        }
    }

    pub async fn get_kv_diff(&self, id: &str, hash_1: &str, hash_2: &str) -> Result<Vec<Diff>> {
        let json = json!({
            "id": id,
            "hash_1": hash_1,
            "hash_2": hash_2,
        });
        let res: KeyValueDiffResponse = self
            .cmd("get_kv_diff", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.diff {
            None => Err(anyhow!("{:#?}", res.error)),
            Some(r) => Ok(r),
        }
    }

    pub async fn get_local_root(&self, id: &str) -> Result<String> {
        let json = json!({
            "id": id,
        });
        let res: LocalRootResponse = self
            .cmd("get_local_root", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.hash {
            None => Err(anyhow!("{:#?}", res.error)),
            Some(r) => Ok(r),
        }
    }

    pub async fn get_mirrors(&self, id: &str) -> Result<Vec<Mirror>> {
        let json = json!({
            "id": id,
        });
        let res: MirrorsResponse = self
            .cmd("get_mirrors", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.mirrors {
            None => Err(anyhow!("{:#?}", res.error)),
            Some(r) => Ok(r),
        }
    }

    pub async fn get_sync_status(&self, id: &str) -> Result<SyncStatus> {
        let json = json!({
            "id": id,
        });
        let res: SyncStatusResponse = self
            .cmd("get_sync_status", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.sync_status {
            None => Err(anyhow!("{:#?}", res.error)),
            Some(r) => Ok(r),
        }
    }

    pub async fn get_keys_values(
        &self,
        id: &str,
        root_hash: Option<&str>,
    ) -> Result<Vec<KeysValue>> {
        let json = json!({
            "id": id,
            "root_hash": root_hash.unwrap_or(""),
        });
        let res: KeysValuesResponse = self
            .cmd("get_keys_values", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.keys_values {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }

    pub async fn get_value(&self, id: &str, key: &str, root_hash: &str) -> Result<String> {
        let json = json!({
            "id": id,
            "key": key,
            "root_hash": root_hash,
        });

        let res: ValueResponse = self
            .cmd("get_value", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.value {
            None => Err(anyhow!("{:#?}", res.error)),
            Some(r) => Ok(r),
        }
    }
    pub async fn insert(&self, store_id: &str, key: &str, value: &str) -> Result<String> {
        let json = json!({
            "id": store_id,
            "key": key,
            "value": value,
        });

        let res: UpdateResponse = self
            .cmd("insert", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.tx_id {
            None => Err(anyhow!("{:#?}", res.error)),
            Some(r) => Ok(r),
        }
    }
    pub async fn make_offer(&self, offer: Offer) -> Result<Offer> {
        let res: OfferResponse = self
            .cmd("make_offer", Some(serde_json::to_string(&offer)?))
            .await?
            .json()
            .await?;
        match res.offer {
            None => Err(anyhow!("{:#?}", res.error)),
            Some(r) => Ok(r),
        }
    }
    pub async fn take_offer(&self, offer: Offer) -> Result<String> {
        let res: TakeOfferResponse = self
            .cmd("take_offer", Some(serde_json::to_string(&offer)?))
            .await?
            .json()
            .await?;
        match res.trade_id {
            None => Err(anyhow!("{:#?}", res.error)),
            Some(r) => Ok(r),
        }
    }
    pub async fn verify_offer(&self, offer: Offer) -> Result<VerifyOfferResponse> {
        Ok(self
            .cmd("verify_offer", Some(serde_json::to_string(&offer)?))
            .await?
            .json()
            .await?)
    }
    pub async fn remove_subscriptions(&self, id: &str, urls: Vec<&str>) -> Result<()> {
        let json = json!({
            "id": id,
            "urls": urls,
        });

        let res: BasicResponse = self
            .cmd("remove_subscriptions", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.error {
            Some(e) => Err(anyhow!("{:#?}", e)),
            None => Ok(()),
        }
    }

    pub async fn subscribe(&self, id: &str, urls: Vec<&str>) -> Result<()> {
        let json = json!({
            "id": id,
            "urls": urls,
        });

        let res: BasicResponse = self
            .cmd("subscribe", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.error {
            Some(e) => Err(anyhow!("{:#?}", e)),
            None => Ok(()),
        }
    }
    pub async fn unsubscribe(&self, id: &str) -> Result<()> {
        let json = json!({
            "id": id,
        });

        let res: BasicResponse = self
            .cmd("subscribe", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.error {
            Some(e) => Err(anyhow!("{:#?}", e)),
            None => Ok(()),
        }
    }
    pub async fn subscriptions(&self) -> Result<Vec<String>> {
        let res: StoresResponse = self.cmd("subscriptions", None).await?.json().await?;
        match res.store_ids {
            None => Err(anyhow!("{:#?}", res.error)),
            Some(r) => Ok(r),
        }
    }

    pub async fn cmd(
        &self,
        command: &str,
        json: Option<String>,
    ) -> Result<Response, reqwest::Error> {
        let url = self.make_url(command);
        match json {
            Some(json) => {
                self.http
                    .post(&url)
                    .header("Content-Type", "application/json")
                    .body(json)
                    .send()
                    .await
            }
            None => {
                self.http
                    .post(&url)
                    .header("Content-Type", "application/json")
                    .body("{}")
                    .send()
                    .await
            }
        }
    }

    fn make_url(&self, command: &str) -> String {
        format!("https://{}/{}", &self.addr.to_string(), &command)
    }
}
>>>>>>> a2d8e39 (introduce datalayer api, migrate models and rename crate to chia-client)
