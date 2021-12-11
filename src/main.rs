//! Basic tool for fetching data about solar energy system from Enlighten API
use anyhow::Context;
use openapi::{
    self,
    apis::configuration::{ApiKey, Configuration},
};

/// Describes the config for this tool
#[derive(serde::Deserialize)]
struct Config {
    enlighten_key: String,
    enlighten_user_id: String,
}

/// Wraps `anyhow::Error` in something implementing `std::error::Error`
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
struct AnyhowWrap(#[from] anyhow::Error);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO config should be runtime
    let config: Config = toml::from_str(include_str!("../enphase_creds.toml"))
        .context("parsing enphase_creds.toml")?;

    let enlighten_config = Configuration {
        base_path: String::from("https://api.enphaseenergy.com/api/v2"),
        user_agent: None,
        client: reqwest::Client::new(),
        basic_auth: None,
        oauth_access_token: None,
        bearer_access_token: None,
        api_key: Some(ApiKey { prefix: None, key: config.enlighten_key }),
    };

    use openapi::apis::default_api as enlighten;
    let user_id = &config.enlighten_user_id;
    let response = enlighten::systems(
        &enlighten_config,
        user_id,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await
    .context("listing systems")?;

    let system_id = response.systems[0].system_id;
    eprintln!("first system id: {:?}", system_id);

    // let inventory = enlighten::inventory(&config, user_id, system_id)
    //     .await
    //     .context("inventory")?;
    // eprintln!("{:?}", inventory);

    eprintln!(
        "{:?}",
        enlighten::stats(&enlighten_config, user_id, system_id, None, None)
            .await
            .context("getting stats")?
    );

    // eprintln!(
    //     "{:?}",
    //     enlighten::energy_lifetime(
    //         &config,
    //         user_id,
    //         system_id,
    //         None,
    //         None,
    //         Some("all")
    //     )
    //     .await
    //     .context("energy_lifetime")?
    // );

    // eprintln!(
    //     "{:?}",
    //     enlighten::summary(&config, user_id, system_id, None)
    //         .await
    //         .context("summary")?
    // );

    Ok(())
}
