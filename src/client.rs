use log::debug;
use std::time::Duration;

use crate::{common, model};

pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> reqwest::Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(3)) // TODO: make configurable
            .build()?;

        Ok(Client { client })
    }

    pub async fn initialize(&self) -> reqwest::Result<model::StartSpringIoModel> {
        let url = common::START_SPRING_IO_URL;

        debug!("Fetching {url:?}...");
        let res = self.client.get(url).send().await?;

        debug!("Response: {:?} {}", res.version(), res.status());
        debug!("Headers: {:#?}", res.headers());

        let model = res.json::<model::StartSpringIoModel>().await?;

        Ok(model)
    }
}
