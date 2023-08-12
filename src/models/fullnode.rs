use crate::util::deserialize_optional_timestamp;
use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfoResponse {
    pub network_name: String,
    pub network_prefix: String,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainStateResponse {
    pub blockchain_state: Option<BlockchainState>,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainState {
    pub block_max_cost: u64,
    pub difficulty: u64,
    pub genesis_challenge_initialized: bool,
    pub mempool_cost: u64,
    pub mempool_max_total_cost: u64,
    pub mempool_min_fees: Option<HashMap<String, u64>>,
    pub mempool_size: u64,
    pub node_id: String,
    pub peak: Peak,
    pub space: u128,
    pub sub_slot_iters: u64,
    pub sync: Sync,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSpaceResponse {
    pub space: Option<u128>,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Peak {
    pub challenge_block_info_hash: String,
    pub challenge_vdf_output: Output,
    pub deficit: u64,
    pub farmer_puzzle_hash: String,
    pub fees: Option<u64>,
    pub finished_challenge_slot_hashes: Option<Vec<String>>,
    pub finished_slot_hashes: Option<Vec<String>>,
    pub finished_reward_slot_hashes: Option<Vec<String>>,
    pub header_hash: String,
    pub height: u64,
    pub overflow: bool,
    pub pool_puzzle_hash: String,
    pub prev_hash: String,
    pub prev_transaction_block_hash: Option<String>,
    pub prev_transaction_block_height: u64,
    pub required_iters: u64,
    pub reward_claims_incorporated: Option<Vec<Coin>>,
    pub reward_infusion_new_challenge: String,
    pub signage_point_index: u64,
    pub sub_epoch_summary_included: Option<String>,
    pub sub_slot_iters: u64,
    #[serde(deserialize_with = "deserialize_optional_timestamp")]
    pub timestamp: Option<DateTime<Utc>>,
    pub total_iters: u64,
    pub weight: u64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Sync {
    pub sync_mode: bool,
    pub sync_progress_height: u64,
    pub sync_tip_height: u64,
    pub synced: bool,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BlockCountMetricsResponse {
    pub metrics: Option<BlockCountMetrics>,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BlockCountMetrics {
    pub compact_blocks: u64,
    pub hint_count: u64,
    pub uncompact_blocks: u64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct MemPoolTxIdsRespose {
    pub success: bool,
    pub tx_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeadersResponse {
    pub headers: Option<Vec<BlockHeader>>,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub challenge_chain_sp_proof: ChallengeChainProof,
    pub finished_sub_slots: Vec<Option<String>>,
    pub foliage: Foliage,
    pub foliage_transaction_block: FoliageTransactionBlock,
    pub reward_chain_block: RewardChainBlock,
    pub reward_chain_sp_proof: Option<ChallengeChainProof>,
    pub transactions_filter: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Foliage {
    pub foliage_block_data: FoliageBlockData,
    pub foliage_block_data_signature: String,
    pub foliage_transaction_block_hash: Option<String>,
    pub foliage_transaction_block_signature: Option<String>,
    pub prev_block_hash: String,
    pub reward_block_hash: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct FoliageBlockData {
    pub extension_data: String,
    pub farmer_reward_puzzle_hash: String,
    pub pool_signature: Option<String>,
    pub pool_target: PoolTarget,
    pub unfinished_reward_block_hash: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PoolTarget {
    pub max_height: u64,
    pub puzzle_hash: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct FoliageTransactionBlock {
    pub additions_root: String,
    pub filter_hash: String,
    pub prev_transaction_block_hash: String,
    pub removals_root: String,
    #[serde(deserialize_with = "deserialize_optional_timestamp")]
    pub timestamp: Option<DateTime<Utc>>,
    pub transactions_info_hash: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RewardChainBlock {
    pub challenge_chain_sp_vdf: Option<ChallengeData>,
    pub challenge_chain_sp_signature: String,
    pub pos_ss_cc_challenge_hash: String,
    pub proof_of_space: ProofOfSpace,
    pub reward_chain_sp_signature: String,
    pub reward_chain_sp_vdf: Option<ChallengeData>,
    pub signage_point_index: u64,
    pub total_iters: u64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ChallengeData {
    pub challenge: String,
    pub number_of_iterations: u64,
    pub output: Output,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    pub data: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ProofOfSpace {
    pub challenge: String,
    pub plot_public_key: String,
    pub pool_contract_puzzle_hash: Option<String>,
    pub pool_public_key: Option<String>,
    pub proof: String,
    pub size: u64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct MemPoolItemsResponse {
    pub mempool_items: Option<HashMap<String, MemPoolItem>>,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct MemPoolItemResponse {
    pub mempool_item: Option<MemPoolItem>,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct MemPoolItem {
    pub additions: Vec<Coin>,
    pub cost: u64,
    pub fee: u64,
    pub npc_result: NpcResult,
    pub program: Option<String>,
    pub removals: Option<Vec<Coin>>,
    pub spend_bundle: SpendBundle,
    pub spend_bundle_name: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct NpcResult {
    pub conds: Conds,
    pub cost: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
/*
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct NpcResult {
    pub clvm_cost: i64,
    pub error: Option<String>,
    pub npc_list: Vec<NpcList>,
}

*/
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct NpcList {
    pub coin_name: String,
    pub conditions: Vec<(String, Vec<Condition>)>,
    pub puzzle_hash: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub opcode: String,
    pub vars: Vec<String>,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Conds {
    pub agg_sig_unsafe: Vec<Option<String>>,
    pub cost: u64,
    pub height_absolute: u64,
    pub reserve_fee: u64,
    pub seconds_absolute: u64,
    pub spends: Vec<Spend>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Spend {
    pub agg_sig_me: Vec<Vec<String>>,
    pub coin_id: String,
    pub create_coin: Vec<(String, u64, Option<String>)>,
    pub height_relative: Option<u64>,
    pub puzzle_hash: String,
    pub seconds_relative: Option<u64>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BlockSpendsResponse {
    pub block_spends: Option<Vec<CoinSpend>>,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SpendBundle {
    pub aggregated_signature: String,
    pub coin_spends: Vec<CoinSpend>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CoinSpend {
    pub coin: Coin,
    pub puzzle_reveal: String,
    pub solution: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Coin {
    pub amount: u64,
    pub parent_coin_info: String,
    pub puzzle_hash: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BlockResponse {
    pub block: Option<Block>,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BlocksResponse {
    pub blocks: Option<Vec<Block>>,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub challenge_chain_ip_proof: Option<ChallengeChainProof>,
    pub challenge_chain_sp_proof: Option<ChallengeChainProof>,
    pub finished_sub_slots: Vec<FinishedSubSlot>,
    pub foliage: Foliage,
    pub foliage_transaction_block: Option<FoliageTransactionBlock>,
    pub header_hash: Option<String>,
    pub reward_chain_block: RewardChainBlock,
    pub reward_chain_ip_proof: Option<ChallengeChainProof>,
    pub reward_chain_sp_proof: Option<ChallengeChainProof>,
    pub transactions_generator: Option<String>,
    pub transactions_generator_ref_list: Option<Vec<u64>>,
    pub transactions_info: Option<TransactionsInfo>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ChallengeChainProof {
    pub normalized_to_identity: bool,
    pub witness: String,
    pub witness_type: u64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsInfo {
    pub aggregated_signature: String,
    pub cost: u64,
    pub fees: u64,
    pub generator_refs_root: String,
    pub generator_root: String,
    pub reward_claims_incorporated: Vec<Coin>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct FinishedSubSlot {
    pub challenge_chain: ChallengeChain,
    pub infused_challenge_chain: HashMap<String, ChallengeData>,
    pub proofs: Proofs,
    pub reward_chain: RewardChain,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ChallengeChain {
    pub challenge_chain_end_of_slot_vdf: ChallengeData,
    pub infused_challenge_chain_sub_slot_hash: Option<String>,
    pub new_difficulty: Option<u64>,
    pub new_sub_slot_iters: Option<u64>,
    pub subepoch_summary_hash: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RewardChain {
    pub challenge_chain_sub_slot_hash: String,
    pub deficit: u64,
    pub end_of_slot_vdf: ChallengeData,
    pub infused_challenge_chain_sub_slot_hash: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Proofs {
    pub challenge_chain_slot_proof: ChallengeChainProof,
    pub infused_challenge_chain_slot_proof: ChallengeChainProof,
    pub reward_chain_slot_proof: ChallengeChainProof,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BlockRecordResponse {
    pub block_record: Option<BlockRecord>,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BlockRecordsResponse {
    pub block_records: Option<Vec<BlockRecord>>,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BlockRecord {
    pub challenge_block_info_hash: String,
    pub challenge_vdf_output: Output,
    pub deficit: Option<u64>,
    pub farmer_puzzle_hash: String,
    pub fees: Option<u64>,
    pub finished_challenge_slot_hashes: Option<Vec<String>>,
    pub finished_infused_challenge_slot_hashes: Option<Vec<String>>,
    pub finished_reward_slot_hashes: Option<Vec<String>>,
    pub header_hash: String,
    pub height: u64,
    pub infused_challenge_vdf_output: Option<Output>,
    pub overflow: bool,
    pub pool_puzzle_hash: String,
    pub prev_hash: String,
    pub prev_transaction_block_hash: Option<String>,
    pub prev_transaction_block_height: u64,
    pub required_iters: u64,
    pub reward_claims_incorporated: Option<Vec<Coin>>,
    pub reward_infusion_new_challenge: String,
    pub signage_point_index: u64,
    pub sub_epoch_summary_included: Option<SubEpochSummaryIncluded>,
    pub sub_slot_iters: u64,
    #[serde(deserialize_with = "deserialize_optional_timestamp")]
    pub timestamp: Option<DateTime<Utc>>,
    pub total_iters: u64,
    pub weight: u64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SubEpochSummaryIncluded {
    pub new_difficulty: Option<u64>,
    pub new_sub_slot_iters: Option<u64>,
    pub num_blocks_overflow: u64,
    pub prev_subepoch_summary_hash: String,
    pub reward_chain_hash: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct StateTransitionsResponse {
    pub additions: Option<Vec<CoinRecord>>,
    pub removals: Option<Vec<CoinRecord>>,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct StateTransitions {
    pub additions: Vec<CoinRecord>,
    pub removals: Vec<CoinRecord>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CoinRecordResponse {
    pub coin_record: Option<CoinRecord>,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CoinRecordsResponse {
    pub coin_records: Option<Vec<CoinRecord>>,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CoinRecord {
    pub coin: Coin,
    pub coinbase: bool,
    pub confirmed_block_index: u64,
    pub spent: bool,
    pub spent_block_index: u64,
    #[serde(deserialize_with = "deserialize_optional_timestamp")]
    pub timestamp: Option<DateTime<Utc>>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CoinSolutionResponse {
    pub coin_solution: Option<CoinSolution>,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CoinSolution {
    pub coin: Coin,
    pub puzzle_reveal: String,
    pub solution: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SignagePointOrEos {
    pub time_received: Option<f32>,
    pub reverted: Option<bool>,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub eos: Option<FinishedSubSlot>,
    pub signage_point: Option<SignagePoint>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SignagePoint {
    pub cc_vdf: ChallengeData,
    pub cc_proof: ChallengeChainProof,
    pub rc_vdf: ChallengeData,
    pub rc_proof: ChallengeChainProof,
}
