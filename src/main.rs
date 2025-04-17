use covid19_api_rs::{
    config::{self, config::DotEnvyConfig},
    covid_client::{
        client::{CovidClient, CovidClienter},
        model::CovidCasesExt,
    },
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🚀 start program 🚀");

    let dotenvy_env: DotEnvyConfig = match config::config::load() {
        Ok(env) => env,
        Err(e) => {
            println!("❌ error loading env: {:#?}", e);
            return Ok(());
        }
    };

    let client = CovidClienter::new(dotenvy_env.covid_client_base_url);

    let data = client.get_covid_case_data().await?;

    println!("✅ total case: {:#?}", data.total_case());
    println!("✅ summary case by province: {:#?}", data.summary());

    Ok(())
}
