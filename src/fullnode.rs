use crate::prelude::*;

pub struct Rpc {
    pub client: Client,
}
impl Rpc {
    pub fn init(client: Client) -> Self {
        Self { client }
    }
    pub async fn get_network_info(&self) -> Result<NetworkInfoResponse> {
        Ok(self
            .client
            .cmd("get_network_info", None)
            .await?
            .json()
            .await?)
    }
    pub async fn get_blockchain_state(&self) -> Result<BlockchainState> {
        let res: BlockchainStateResponse = self
            .client
            .cmd("get_blockchain_state", None)
            .await?
            .json()
            .await?;
        match res.blockchain_state {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }
    pub async fn get_block_count_metrics(&self) -> Result<BlockCountMetrics> {
        let res: BlockCountMetricsResponse = self
            .client
            .cmd("get_block_count_metrics", None)
            .await?
            .json()
            .await?;
        match res.metrics {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }
    pub async fn get_unfinished_block_headers(&self) -> Result<Vec<BlockHeader>> {
        let res: BlockHeadersResponse = self
            .client
            .cmd("get_unfinished_block_headers", None)
            .await?
            .json()
            .await?;
        match res.headers {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }
    pub async fn get_all_mempool_tx_ids(&self) -> Result<Vec<String>> {
        let res: MemPoolTxIdsRespose = self
            .client
            .cmd("get_all_mempool_tx_ids", None)
            .await?
            .json()
            .await?;
        match res.tx_ids {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }

    pub async fn get_all_mempool_items(&self) -> Result<HashMap<String, MemPoolItem>> {
        let res: MemPoolItemsResponse = self
            .client
            .cmd("get_all_mempool_items", None)
            .await?
            .json()
            .await?;
        match res.mempool_items {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }
    pub async fn get_routes(&self) -> Result<Vec<String>> {
        let res: RoutesResponse = self.client.cmd("get_routes", None).await?.json().await?;
        match res.routes {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }
    pub async fn get_block(&self, header_hash: &str) -> Result<Block> {
        let json = json!({
        "header_hash": header_hash,
        });
        let res: BlockResponse = self
            .client
            .cmd("get_block", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.block {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }
    pub async fn get_block_by_height(&self, height: u64) -> Result<Block> {
        let json = json!({
        "height": height,
        });
        let res: BlockRecordResponse = self
            .client
            .cmd("get_block_record_by_height", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.block_record {
            Some(r) => {
                let json = json!({
                "header_hash": r.header_hash,
                });
                let res: BlockResponse = self
                    .client
                    .cmd("get_block", Some(json.to_string()))
                    .await?
                    .json()
                    .await?;
                match res.block {
                    Some(mut b) => {
                        b.header_hash = Some(r.header_hash);
                        Ok(b)
                    }
                    None => Err(anyhow!("{:#?}", res.error)),
                }
            }
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }
    pub async fn get_block_spends_by_height(&self, height: u64) -> Result<Vec<CoinSpend>> {
        let json = json!({
        "height": height,
        });
        let res: BlockRecordResponse = self
            .client
            .cmd("get_block_record_by_height", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.block_record {
            Some(r) => {
                let json = json!({
                "header_hash": r.header_hash,
                });
                let res: BlockSpendsResponse = self
                    .client
                    .cmd("get_block_spends", Some(json.to_string()))
                    .await?
                    .json()
                    .await?;
                match res.block_spends {
                    Some(r) => Ok(r),
                    None => Err(anyhow!("{:#?}", res.error)),
                }
            }
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }
    pub async fn get_blocks(
        &self,
        start: u64,
        end: u64,
        exclude_header_hash: bool,
    ) -> Result<Vec<Block>> {
        let json = json!(
        {
          "start": start,
          "end": end,
          "exclude_header_hash": exclude_header_hash,
        });
        let res: BlocksResponse = self
            .client
            .cmd("get_blocks", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.blocks {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }
    pub async fn get_block_record(&self, header_hash: &str) -> Result<BlockRecord> {
        let json = json!({
        "header_hash": header_hash,
        });
        let res: BlockRecordResponse = self
            .client
            .cmd("get_block_record", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.block_record {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }
    pub async fn get_block_record_by_height(&self, height: u64) -> Result<BlockRecord> {
        let json = json!({
        "height": height,
        });
        let res: BlockRecordResponse = self
            .client
            .cmd("get_block_record_by_height", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.block_record {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }

    pub async fn get_block_records(&self, start: u64, end: u64) -> Result<Vec<BlockRecord>> {
        let json = json!(
        {
          "start": start,
          "end": end,
        });
        let res: BlockRecordsResponse = self
            .client
            .cmd("get_block_records", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.block_records {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }
    pub async fn get_network_space(
        &self,
        older_block_header_hash: &str,
        newer_block_header_hash: &str,
    ) -> Result<u128> {
        let json = json!(
        {
          "older_block_header_hash": older_block_header_hash,
          "newer_block_header_hash": newer_block_header_hash,
        });
        let res: NetworkSpaceResponse = self
            .client
            .cmd("get_network_space", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.error {
            None => Ok(res.space.unwrap_or_default()),
            Some(e) => Err(anyhow!("{:#?}", e)),
        }
    }

    pub async fn get_additions(&self, header_hash: &str) -> Result<Vec<CoinRecord>> {
        let json = json!({
        "header_hash": header_hash,
        });
        let res: StateTransitionsResponse = self
            .client
            .cmd("get_additions_and_removals", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.error {
            None => Ok(res.additions.unwrap_or_default()),
            Some(e) => Err(anyhow!("{:#?}", e)),
        }
    }
    pub async fn get_removals(&self, header_hash: &str) -> Result<Vec<CoinRecord>> {
        let json = json!({
        "header_hash": header_hash,
        });
        let res: StateTransitionsResponse = self
            .client
            .cmd("get_additions_and_removals", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.error {
            None => Ok(res.removals.unwrap_or_default()),
            Some(e) => Err(anyhow!("{:#?}", e)),
        }
    }

    pub async fn get_state_transitions(&self, header_hash: &str) -> Result<StateTransitions> {
        let json = json!({
        "header_hash": header_hash,
        });
        let res: StateTransitionsResponse = self
            .client
            .cmd("get_additions_and_removals", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.error {
            None => {
                let state_transitions = StateTransitions {
                    additions: res.additions.unwrap_or_default(),
                    removals: res.removals.unwrap_or_default(),
                };
                Ok(state_transitions)
            }
            Some(e) => Err(anyhow!("{:#?}", e)),
        }
    }
    pub async fn get_additions_and_removals(&self, header_hash: &str) -> Result<StateTransitions> {
        let json = json!({
        "header_hash": header_hash,
        });
        let res: StateTransitionsResponse = self
            .client
            .cmd("get_additions_and_removals", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.error {
            None => {
                let state_transitions = StateTransitions {
                    additions: res.additions.unwrap_or_default(),
                    removals: res.removals.unwrap_or_default(),
                };
                Ok(state_transitions)
            }
            Some(e) => Err(anyhow!("{:#?}", e)),
        }
    }
    pub async fn get_coin_record_by_name(&self, name: &str) -> Result<CoinRecord> {
        let json = json!({
        "name": name,
        });
        let res: CoinRecordResponse = self
            .client
            .cmd("get_coin_record_by_name", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.coin_record {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }
    pub async fn get_coin_records_by_names(
        &self,
        names: Vec<&str>,
        start_height: u64,
        end_height: u64,
        include_spent_coins: bool,
    ) -> Result<Vec<CoinRecord>> {
        let json = json!({
        "names": names,
        "start_height": start_height,
        "end_height": end_height,
        "include_spent_coins": include_spent_coins,
        });
        let res: CoinRecordsResponse = self
            .client
            .cmd("get_coin_records_by_names", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.coin_records {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }
    pub async fn get_coin_records_by_parent_ids(
        &self,
        parent_ids: Vec<&str>,
        start_height: u64,
        end_height: u64,
        include_spent_coins: bool,
    ) -> Result<Vec<CoinRecord>> {
        let json = json!({
        "parent_ids": parent_ids,
        "start_height": start_height,
        "end_height": end_height,
        "include_spent_coins": include_spent_coins,
        });
        let res: CoinRecordsResponse = self
            .client
            .cmd("get_coin_records_by_parent_ids", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.coin_records {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }
    pub async fn get_coin_records_by_hint(
        &self,
        start_height: u64,
        end_height: u64,
        include_spent_coins: bool,
        hint: &str,
    ) -> Result<Vec<CoinRecord>> {
        let json = json!({
        "hint": hint,
        "start_height": start_height,
        "end_height": end_height,
        "include_spent_coins": include_spent_coins,
        });
        let res: CoinRecordsResponse = self
            .client
            .cmd("get_coin_records_by_hint", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.coin_records {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }
    pub async fn get_puzzle_and_solution(
        &self,
        coin_id: &str,
        height: u64,
    ) -> Result<CoinSolution> {
        let json = json!({
        "coin_id": coin_id,
        "height": height,
        });
        let res: CoinSolutionResponse = self
            .client
            .cmd("get_puzzle_and_solution", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.coin_solution {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }
    pub async fn get_recent_signage_point_or_eos(
        &self,
        sp_hash: Option<&str>,
        challenge_hash: Option<&str>,
    ) -> Result<SignagePointOrEos> {
        let json = if let Some(x) = sp_hash {
            json!({
            "sp_hash": x,
            })
        } else {
            json!({ "challenge_hash": challenge_hash.unwrap()})
        };
        let res: SignagePointOrEos = self
            .client
            .cmd("get_recent_signage_point_or_eos", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.error {
            None => Ok(res),
            Some(e) => Err(anyhow!("{:#?}", e)),
        }
    }
    pub async fn get_coin_records_by_puzzle_hash(
        &self,
        puzzle_hash: &str,
        start_height: Option<u64>,
        end_height: Option<u64>,
        include_spent_coins: Option<bool>,
    ) -> Result<Vec<CoinRecord>> {
        let mut json = json!({ "puzzle_hash": puzzle_hash });

        if let Some(start_height) = start_height {
            json["start_height"] = Value::from(start_height);
        }
        if let Some(end_height) = end_height {
            json["end_height"] = Value::from(end_height);
        }
        if let Some(include_spent_coins) = include_spent_coins {
            json["include_spent_coins"] = Value::from(include_spent_coins);
        }
        let res: CoinRecordsResponse = self
            .client
            .cmd("get_coin_records_by_puzzle_hash", Some(json.to_string()))
            .await?
            .json()
            .await?;

        match res.coin_records {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }
    pub async fn get_coin_records_by_puzzle_hashes(
        &self,
        puzzle_hashes: Vec<&str>,
        start_height: u64,
        end_height: u64,
        include_spent_coins: bool,
    ) -> Result<Vec<CoinRecord>> {
        let json = json!({
        "puzzle_hashes": puzzle_hashes,
        "start_height": start_height,
        "end_height": end_height,
        "include_spent_coins": include_spent_coins,
        });
        let res: CoinRecordsResponse = self
            .client
            .cmd("get_coin_records_by_puzzle_hashes", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.coin_records {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }
    pub async fn get_mempool_item_by_tx_id(&self, tx_id: &str) -> Result<MemPoolItem> {
        let json = json!({
        "tx_id": tx_id,
        });
        let res: MemPoolItemResponse = self
            .client
            .cmd("get_mempool_item_by_tx_id", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.mempool_item {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }

    pub async fn get_healthz(&self) -> Result<bool, Error> {
        let res: HealthzResponse = self.client.cmd("healthz", None).await?.json().await?;
        Ok(res.success)
    }
}