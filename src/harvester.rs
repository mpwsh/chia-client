<<<<<<< HEAD
use crate::prelude::*;

pub struct Rpc {
    pub client: Client,
||||||| parent of a2d8e39 (introduce datalayer api, migrate models and rename crate to chia-client)
use std::path::Path;

use reqwest::ClientBuilder;
use reqwest::Response;

use crate::util::load_pem_pair;
use crate::Error;

use chia_models::harvester::GetPlotsResponse;
pub use chia_models::harvester::Plots;

pub struct Client {
    host: String,
    port: u16,
    http: reqwest::Client,
=======
use crate::prelude::*;

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

    pub async fn get_plots(&self) -> Result<Plots, Error> {
<<<<<<< HEAD
        let response = self.client.cmd("get_plots", None).await?;
||||||| parent of a2d8e39 (introduce datalayer api, migrate models and rename crate to chia-client)
        let response = self.cmd("get_plots").await?;
=======
        let response = self.cmd("get_plots", None).await?;
>>>>>>> a2d8e39 (introduce datalayer api, migrate models and rename crate to chia-client)
        let response: GetPlotsResponse = response.json().await?;
        Ok(response.into())
    }

<<<<<<< HEAD
    pub async fn get_routes(&self) -> Result<Vec<String>> {
        let res: RoutesResponse = self.client.cmd("get_routes", None).await?.json().await?;
        match res.routes {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
||||||| parent of a2d8e39 (introduce datalayer api, migrate models and rename crate to chia-client)
    async fn cmd(&self, command: &str) -> Result<Response, reqwest::Error> {
        let url = self.make_url(command);
        self.http
            .post(&url)
            .header("Content-Type", "application/json")
            .body("{}")
            .send()
            .await
    }

    fn make_url(&self, command: &str) -> String {
        format!("https://{}:{}/{}", &self.host, self.port, &command)
=======
    pub async fn get_routes(&self) -> Result<Vec<String>> {
        let res: RoutesResponse = self.cmd("get_routes", None).await?.json().await?;
        match res.routes {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
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
        format!("https://{}:{}/{}", &self.host, self.port, &command)
>>>>>>> a2d8e39 (introduce datalayer api, migrate models and rename crate to chia-client)
    }
}
