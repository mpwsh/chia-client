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
}

impl Client {
    pub async fn new(
        host: &str,
        port: u16,
        key_file: impl AsRef<Path>,
        cert_file: impl AsRef<Path>,
    ) -> Result<Self, Error> {
        let identity = load_pem_pair(key_file, cert_file).await?;
        let http = ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            //.danger_accept_invalid_hostnames(true)
            .identity(identity)
            .build()?;
        Ok(Self {
            host: host.to_string(),
            port,
            http,
        })
    }
    pub async fn get_routes(&self) -> Result<Vec<String>> {
        let res: RoutesResponse = self.cmd("get_routes", None).await?.json().await?;
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
            .cmd("check_offer_validity", Some(json.to_string()))
            .await?
            .json()
            .await?)
    }

    pub async fn get_healthz(&self) -> Result<String, Error> {
        let res: HealthzResponse = self.cmd("healthz", None).await?.json().await?;
        Ok(res.success)
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
        format!("https://{}:{}/{}", &self.host, self.port, &command)
    }
}
