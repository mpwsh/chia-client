use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::fullnode::{Coin, SpendBundle};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct OfferSummaryResponse {
    pub success: bool,
    pub summary: Option<OfferSummary>,
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct OfferSummary {
    pub fees: u64,
    pub infos: HashMap<String, OfferInfo>,
    pub offered: HashMap<String, u64>,
    pub requested: HashMap<String, u64>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct OfferInfo {
    pub tail: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct OfferValidityResponse {
    pub success: bool,
    pub valid: bool,
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub success: bool,
    pub transaction: Option<Transaction>,
    pub transaction_id: String,
    pub error: Option<String>,
    pub traceback: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub additions: Vec<Coin>,
    pub amount: i64,
    pub confirmed: bool,
    pub confirmed_at_height: i64,
    pub created_at_time: i64,
    pub fee_amount: i64,
    pub memos: HashMap<String, String>,
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
