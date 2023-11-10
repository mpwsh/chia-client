mod error;
use std::net::{IpAddr, Ipv4Addr};

pub use error::Error;
pub mod datalayer;
pub mod fullnode;
pub mod harvester;
pub mod models;
pub mod prelude;
pub mod util;
pub mod wallet;

use crate::prelude::*;

pub struct Config {
    pub addr: SocketAddr,
    pub key_path: PathBuf,
    pub cert_path: PathBuf,
}

impl Config {
    pub fn new(addr: SocketAddr, key_path: &Path, cert_path: &Path) -> Self {
        Self {
            addr,
            key_path: key_path.to_path_buf(),
            cert_path: cert_path.to_path_buf(),
        }
    }
}
pub struct ClientBuilder {
    pub config: Config,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            config: Config {
                addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8555),
                key_path: Path::new("~/.chia/mainnet/config/ssl/full_node/private_full_node.key")
                    .into(),
                cert_path: Path::new("~/.chia/mainnet/config/ssl/full_node/private_full_node.crt")
                    .into(),
            },
        }
    }

    pub fn addr(mut self, ip: &str, port: u16) -> Self {
        let ip: Ipv4Addr = ip.parse().expect("Invalid IPv4 address");
        let addr = SocketAddr::new(ip.into(), port);

        self.config.addr = addr;
        self
    }

    pub fn key_path<P: Into<PathBuf>>(mut self, key_path: P) -> Self {
        self.config.key_path = key_path.into();
        self
    }

    pub fn cert_path<P: Into<PathBuf>>(mut self, cert_path: P) -> Self {
        self.config.cert_path = cert_path.into();
        self
    }

    pub async fn build(self) -> Result<Client> {
        let config = self.config;
        Client::new(&config).await
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    pub addr: SocketAddr,
    pub http: reqwest::Client,
}

impl Client {
    pub async fn new(config: &Config) -> Result<Self> {
        let identity = load_pem_pair(&config.key_path, &config.cert_path).await?;
        let http = reqwest::ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .identity(identity)
            .build()?;
        Ok(Self {
            addr: config.addr,
            http,
        })
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
            },
            None => {
                self.http
                    .post(&url)
                    .header("Content-Type", "application/json")
                    .body("{}")
                    .send()
                    .await
            },
        }
    }

    fn make_url(&self, command: &str) -> String {
        format!("https://{}/{}", &self.addr.to_string(), &command)
    }
}
