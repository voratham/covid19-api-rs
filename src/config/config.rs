use anyhow::{Ok, Result};

#[derive(Debug, Clone)]
pub struct DotEnvyConfig {
    pub env: String,
    pub covid_client_base_url: String,
}

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();

    Ok(DotEnvyConfig {
        env: std::env::var("ENV")
            .unwrap_or_else(|_| "development".to_string())
            .parse()?,
        covid_client_base_url: std::env::var("COVID_CLIENT_BASE_URL")
            .expect("COVID_CLIENT_BASE_URL is invalid")
            .parse()?,
    })
}
