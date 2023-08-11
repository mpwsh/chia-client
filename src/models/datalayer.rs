use crate::util::deserialize_optional_timestamp;
use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
//use std::collections::HashMap;
use super::fullnode::{Coin, SpendBundle};
// add_mirror - add_missing_files - cancel_offer
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BasicResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct UpdateResponse {
    pub success: bool,
    pub tx_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerifyOfferResponse {
    pub fee: i64,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub valid: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Offer {
    pub maker: Vec<Maker>,
    pub taker: Vec<Taker>,
    pub fee: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Maker {
    pub store_id: String,
    pub inclusions: Vec<Inclusion>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Inclusion {
    pub key: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Taker {
    pub store_id: String,
    pub inclusions: Vec<Inclusion2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Inclusion2 {
    pub key: String,
    pub value: String,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ChangeListEntry {
    pub id: String,
    pub changelist: Vec<Changelist>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Changelist {
    pub action: String,
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct KeysValuesResponse {
    pub keys_values: Option<Vec<KeysValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub success: bool,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct KeysResponse {
    pub keys: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub success: bool,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Key {
    pub id: String,
    pub key: String,
    pub fee: String,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct KeysValue {
    pub atom: Option<String>,
    pub hash: String,
    pub key: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CreateDataStoreResponse {
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub success: bool,
    pub txs: Option<Vec<Transaction>>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub additions: Vec<Coin>,
    pub amount: i64,
    pub confirmed: bool,
    pub confirmed_at_height: i64,
    pub created_at_time: i64,
    pub fee_amount: i64,
    pub memos: Option<Vec<String>>,
    pub name: Option<String>,
    pub removals: Vec<Coin>,
    pub sent: i64,
    pub sent_to: Vec<String>,
    pub spend_bundle: Option<SpendBundle>,
    pub to_puzzle_hash: String,
    pub trade_id: Option<String>,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub wallet_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyValueDiffResponse {
    pub diff: Option<Vec<Diff>>,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalRootResponse {
    pub hash: Option<String>,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TakeOfferResponse {
    pub trade_id: Option<String>,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoresResponse {
    pub store_ids: Option<Vec<String>>,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Diff {
    pub key: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RootResponse {
    pub confirmed: bool,
    pub hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub success: bool,
    #[serde(deserialize_with = "deserialize_optional_timestamp")]
    pub timestamp: Option<DateTime<Utc>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RootsResponse {
    pub root_hashes: Option<Vec<RootHash>>,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RootHash {
    pub confirmed: bool,
    pub hash: String,
    pub id: String,
    #[serde(deserialize_with = "deserialize_optional_timestamp")]
    pub timestamp: Option<DateTime<Utc>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RootHistoryResponse {
    pub root_history: Option<Vec<RootHistory>>,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RootHistory {
    pub confirmed: bool,
    pub root_hash: String,
    #[serde(deserialize_with = "deserialize_optional_timestamp")]
    pub timestamp: Option<DateTime<Utc>>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AncestorsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub ancestors: Option<Vec<String>>,
    pub success: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MirrorsResponse {
    pub mirrors: Option<Vec<Mirror>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub success: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mirror {
    pub amount: i64,
    pub coin_id: String,
    pub launcher_id: String,
    pub ours: bool,
    pub urls: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValueResponse {
    pub success: bool,
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyncStatusResponse {
    pub success: bool,
    pub sync_status: Option<SyncStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyncStatus {
    pub generation: i64,
    pub root_hash: String,
    pub target_generation: i64,
    pub target_root_hash: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OfferResponse {
    pub offer: Option<Offer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub success: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Proof {
    pub key: String,
    pub layers: Vec<Layer>,
    pub node_hash: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Layer {
    pub combined_hash: String,
    pub other_hash: String,
    pub other_hash_side: String,
}
