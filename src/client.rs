use log::debug;
use std::time::Duration;

use crate::{common, model};

pub(crate) struct Client {
    client: reqwest::blocking::Client,
}

impl Client {
    pub fn new() -> reqwest::Result<Self> {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(3)) // TODO: make configurable
            .build()?;

        Ok(Client { client })
    }

    pub fn initialize(&self) -> reqwest::Result<()> {
        let url = common::START_SPRING_IO_URL;
        debug!("Fetching {url:?}...");
        let res = self.client.get(url).send()?;
        debug!("Response: {:?} {}", res.version(), res.status());
        debug!("Headers: {:#?}", res.headers());

        // let body = res.json()?;
        let body = res.json::<model::StartSpringIoModel>()?;

        println!("{body:?}");

        Ok(())
    }
}
