use crate::prelude::*;

pub struct Rpc {
    pub client: Client,
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

    pub async fn get_healthz(&self) -> Result<bool, Error> {
        let res: HealthzResponse = self.client.cmd("healthz", None).await?.json().await?;
        Ok(res.success)
    }
}