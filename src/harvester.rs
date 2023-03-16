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

    pub async fn get_plots(&self) -> Result<Plots, Error> {
        let response = self.cmd("get_plots").await?;
        let response: GetPlotsResponse = response.json().await?;
        Ok(response.into())
    }

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
    }
}
