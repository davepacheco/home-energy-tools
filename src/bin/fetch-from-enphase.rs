//! Basic tool for fetching data about solar energy system from Enlighten API

use anyhow::anyhow;
use anyhow::Context;
use chrono::TimeZone;
use openapi::{
    self,
    apis::configuration::{ApiKey, Configuration},
};
use home_energy_tools::common::EnergyProduced;
use home_energy_tools::common::WattHours;
use std::time::Duration;

/// Describes the config for this tool
#[derive(serde::Deserialize)]
struct Config {
    enlighten_key: String,
    enlighten_user_id: String,
    start_date: chrono::NaiveDate,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO config should be runtime
    let config: Config =
        toml::from_str(include_str!("../../enphase_creds.toml"))
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

    if response.systems.len() != 1 {
        // XXX This is absurd.  But how else do we invoke the conversion?
        // .into() didn't do it.
        Err(anyhow!(
            "expected exactly one system, but found {}",
            response.systems.len()
        ))?;
    }

    let system_id = response.systems[0].system_id;
    let mut date = config.start_date;
    let last_date = chrono::NaiveDateTime::from_timestamp(
        chrono::Utc::now().timestamp(),
        0,
    )
    .date();
    let mut writer = csv::Writer::from_writer(std::io::stdout());
    while date < last_date {
        eprintln!("{}: date: {}", chrono::Utc::now(), date);
        let next_date = date.succ();
        let stats = enlighten::stats(
            &enlighten_config,
            user_id,
            system_id,
            Some(date.and_hms(0, 0, 0).timestamp()),
            Some(next_date.and_hms(0, 0, 0).timestamp()),
        )
        .await
        .with_context(|| format!("fetch stats for {}", date))?;

        for data in stats.intervals {
            let data_end_time = chrono::Utc.timestamp(data.end_at, 0);
            let data_start_time = data_end_time
                .checked_sub_signed(
                    chrono::Duration::from_std(Duration::from_secs(300))
                        .unwrap(),
                )
                .unwrap(); // XXX
            let date_start_local =
                data_start_time.with_timezone(&chrono::Local);
            let energy_wh = WattHours::try_from(data.enwh)?;
            writer
                .serialize(EnergyProduced {
                    datetime_utc: data_start_time,
                    datetime_local: date_start_local,
                    energy_wh,
                })
                .context("writing record")?;
        }

        writer.flush().context("flushing writer")?;
        date = next_date;

        /*
         * We only get 10 requests per minute.  Sleep 6 seconds between
         * requests.  (TODO We could do better here by parsing the error
         * responses.)
         */
        tokio::time::sleep(Duration::from_secs(7)).await;
    }

    writer.flush().context("flushing writer")?;

    Ok(())
}
