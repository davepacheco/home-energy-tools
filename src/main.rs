use anyhow::anyhow;
use anyhow::Context;
use openapi::{
    self,
    apis::configuration::{ApiKey, Configuration},
};

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
struct AnyhowWrap(#[from] anyhow::Error);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Configuration {
        base_path: String::from("https://api.enphaseenergy.com/api/v2"), // XXX
        user_agent: None,
        client: reqwest::Client::new(),
        basic_auth: None,
        oauth_access_token: None,
        bearer_access_token: None,
        api_key: Some(ApiKey {
            prefix: None,
            key: String::from(env!("ENLIGHTEN_KEY")),
        }),
    };

    use openapi::apis::default_api as enlighten;
    let user_id = env!("ENLIGHTEN_USER_ID");
    let response = enlighten::systems(
        &config, user_id, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None,
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
        enlighten::stats(&config, user_id, system_id, None, None)
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
