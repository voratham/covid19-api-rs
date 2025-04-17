use anyhow::{Ok, Result};
use reqwest::StatusCode;

use super::model::{CovidCase, CovidCaseDataResponse};

#[derive(Debug)]
pub struct CovidClienter {
    base_url: String,
    client: reqwest::Client,
}

#[warn(async_fn_in_trait)]
pub trait CovidClient {
    fn get_covid_case_data(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<CovidCase>>> + Send;
}

impl CovidClienter {
    pub fn new(base_url: String) -> Self {
        let client = reqwest::Client::new();
        Self { base_url, client }
    }
}

impl CovidClient for CovidClienter {
    async fn get_covid_case_data(&self) -> Result<Vec<CovidCase>> {
        let resp = self
            .client
            .get(self.base_url.as_str())
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await
            .expect("🔴 Failed to send request");

        if resp.status() >= StatusCode::BAD_REQUEST {
            return Err(anyhow::anyhow!(
                "🔴 Failed to get data from server: {}",
                resp.status()
            ));
        }

        let resp_json = resp
            .json::<CovidCaseDataResponse>()
            .await
            .expect("🔴 Failed to parse response");
        Ok(resp_json.data)
    }
}
