<<<<<<< HEAD
use crate::prelude::*;

pub struct Rpc {
    pub client: Client,
||||||| parent of a2d8e39 (introduce datalayer api, migrate models and rename crate to chia-client)
use std::path::Path;
//use std::collections::HashMap;
use anyhow::{anyhow, Result};
use reqwest::ClientBuilder;
use reqwest::Response;
use serde_json::json;

use crate::util::load_pem_pair;
use crate::Error;
use chia_models::common::*;

pub use chia_models::wallet::*;

#[derive(Debug, Clone)]
pub struct Client {
    host: String,
    port: u16,
    http: reqwest::Client,
=======
use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Client {
    host: String,
    port: u16,
    http: reqwest::Client,
>>>>>>> a2d8e39 (introduce datalayer api, migrate models and rename crate to chia-client)
}

impl Rpc {
    pub fn init(client: Client) -> Self {
        Self { client }
    }
    pub async fn get_routes(&self) -> Result<Vec<String>> {
        let res: RoutesResponse = self.client.cmd("get_routes", None).await?.json().await?;
        match res.routes {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }
    pub async fn get_offer_summary(&self, offer: &str) -> Result<OfferSummary> {
        let json = json!({
        "offer": offer,
        });
        let res: OfferSummaryResponse = self
            .client
            .cmd("get_offer_summary", Some(json.to_string()))
            .await?
            .json()
            .await?;
        match res.summary {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }
    pub async fn check_offer_validity(&self, offer: &str) -> Result<OfferValidityResponse> {
        let json = json!({
        "offer": offer,
        });
        Ok(self
            .client
            .cmd("check_offer_validity", Some(json.to_string()))
            .await?
            .json()
            .await?)
    }

<<<<<<< HEAD
    pub async fn get_healthz(&self) -> Result<bool, Error> {
        let res: HealthzResponse = self.client.cmd("healthz", None).await?.json().await?;
||||||| parent of a2d8e39 (introduce datalayer api, migrate models and rename crate to chia-client)
    pub async fn get_healthz(&self) -> Result<String, Error> {
        let res: HealthzResponse = self.cmd("healthz", None).await?.json().await?;
=======
    pub async fn get_healthz(&self) -> Result<bool, Error> {
        let res: HealthzResponse = self.cmd("healthz", None).await?.json().await?;
>>>>>>> a2d8e39 (introduce datalayer api, migrate models and rename crate to chia-client)
        Ok(res.success)
    }
}
