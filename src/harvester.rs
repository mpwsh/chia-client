use crate::prelude::*;

pub struct Rpc {
    pub client: Client,
}

impl Rpc {
    pub fn init(client: Client) -> Self {
        Self { client }
    }

    pub async fn get_plots(&self) -> Result<Plots, Error> {
        let response = self.client.cmd("get_plots", None).await?;
        let response: GetPlotsResponse = response.json().await?;
        Ok(response.into())
    }

    pub async fn get_routes(&self) -> Result<Vec<String>> {
        let res: RoutesResponse = self.client.cmd("get_routes", None).await?.json().await?;
        match res.routes {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:#?}", res.error)),
        }
    }
}