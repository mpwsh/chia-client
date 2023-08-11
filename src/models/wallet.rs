//use chrono::DateTime;
//use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
//use crate::util::deserialize_optional_timestamp;

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
