//! Data structures and facilities for reading data from PG&E usage files

use crate::common::NetEnergyUsed;
use crate::common::WattHours;
use anyhow::anyhow;
use anyhow::bail;
use chrono::TimeZone;
use lazy_static::lazy_static;
use serde::Deserialize;
use serde::Deserializer;

/// A record from a PG&E electricity usage data file
#[derive(Debug, Deserialize)]
pub struct PgeElectricityRecord {
    #[serde(rename = "TYPE")]
    usage_type: UsageType,
    #[serde(rename = "DATE")]
    date: chrono::NaiveDate,
    #[serde(rename = "START TIME")]
    #[serde(deserialize_with = "deserialize_time")]
    start_time: chrono::NaiveTime,
    #[serde(rename = "END TIME")]
    #[serde(deserialize_with = "deserialize_time")]
    end_time: chrono::NaiveTime,
    #[serde(rename = "USAGE")]
    usage: f64,
    #[serde(rename = "UNITS")]
    units: Units,
    #[serde(rename = "COST")]
    cost: String,
    #[serde(rename = "NOTES")]
    notes: String,
}

#[derive(Debug, serde::Deserialize)]
enum UsageType {
    #[serde(rename = "Electric usage")]
    ElectricUsage,
}

#[derive(Debug, serde::Deserialize)]
enum Units {
    #[serde(rename = "kWh")]
    KWh,
}

/// Custom deserializer for chrono::NaiveTime for times the form "HH:MM", as we
/// get in PG&E data files
fn deserialize_time<'de, D>(
    deserializer: D,
) -> Result<chrono::NaiveTime, D::Error>
where
    D: Deserializer<'de>,
{
    let time_string: String = Deserialize::deserialize(deserializer)?;
    chrono::NaiveTime::parse_from_str(&time_string, "%H:%M")
        .map_err(serde::de::Error::custom)
}

lazy_static! {
    /// Expected interval covered by each input record
    static ref EXPECTED_RECORD_INTERVAL: chrono::Duration =
        chrono::Duration::minutes(59);
}

impl TryFrom<PgeElectricityRecord> for NetEnergyUsed {
    type Error = anyhow::Error;

    fn try_from(
        input_record: PgeElectricityRecord,
    ) -> Result<Self, Self::Error> {
        let start_time = input_record.date.and_time(input_record.start_time);
        let timestamp_start_utc =
            chrono::Utc.from_local_datetime(&start_time).single().ok_or_else(
                || anyhow!("reading start time for record: {:?}", input_record),
            )?;
        let end_time = input_record.date.and_time(input_record.end_time);
        let record_interval = end_time - start_time;
        if record_interval != *EXPECTED_RECORD_INTERVAL {
            bail!(
                "unexpected interval covered by record \
                (expected {:?}, found {:?}, input record: {:?})",
                *EXPECTED_RECORD_INTERVAL,
                record_interval,
                input_record,
            );
        }
        Ok(NetEnergyUsed {
            timestamp_start_utc,
            net_used_wh: WattHours::from_kwh(input_record.usage),
        })
    }
}
