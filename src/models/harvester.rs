use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;
use serde_with::chrono_0_4::datetime_utc_ts_seconds_from_any;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct GetPlotsResponse {
    pub failed_to_open_filenames: Vec<String>,
    pub not_found_filenames: Vec<String>,
    pub plots: Vec<Plot>,
    pub success: bool,
}

#[derive(Debug, Deserialize)]
pub struct Plots {
    pub failed_to_open_filenames: Vec<String>,
    pub not_found_filenames: Vec<String>,
    pub plots: Vec<Plot>,
}

impl From<GetPlotsResponse> for Plots {
    fn from(other: GetPlotsResponse) -> Self {
        Self {
            failed_to_open_filenames: other.failed_to_open_filenames,
            not_found_filenames: other.not_found_filenames,
            plots: other.plots,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Plot {
    pub file_size: u64,
    pub filename: String,
    #[serde(rename = "plot-seed")]
    pub plot_seed: String,
    pub plot_public_key: String,
    pub pool_contract_puzzle_hash: Value,
    pub pool_public_key: String,
    pub size: u8,
    #[serde(with = "datetime_utc_ts_seconds_from_any")]
    pub time_modified: DateTime<Utc>,
}
