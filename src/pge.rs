//! Data structures and facilities for reading data from PG&E usage files

use crate::common::NetEnergyUsed;
use crate::common::WattHours;
use anyhow::anyhow;
use anyhow::bail;
use anyhow::Context;
use chrono::TimeZone;
use lazy_static::lazy_static;
use serde::Deserialize;
use serde::Deserializer;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

/// Reads [`PgeElectricityRecord`]s from a PG&E electricity usage file
// NOTE: It might seem unnecessarily bureaucratic to separate the Reader from
// the Iterator returned by `records()`.  But in the future we may want to
// provide access to the metadata at the top of these files, in which case it
// will be handy to have a separate object for talking about the file itself.
pub struct ElectricityUsageReader<R> {
    csv_reader: csv::Reader<BufReader<R>>,
}

impl<R: Read> ElectricityUsageReader<R> {
    pub fn new(input: R) -> Result<ElectricityUsageReader<R>, anyhow::Error> {
        let mut line_reader = BufReader::new(input);
        let mut buf = String::new(); // TODO-robustness it'd be nice to cap this
        const NSKIP: usize = 5;
        // TODO could save (or at least validate) these lines
        for i in 0..NSKIP {
            line_reader
                .read_line(&mut buf)
                .with_context(|| format!("read line {}", i + 1))?;
            buf = String::new();
        }

        Ok(ElectricityUsageReader {
            csv_reader: csv::ReaderBuilder::new().from_reader(line_reader),
        })
    }

    pub fn records(&mut self) -> ElectricityUsageIterator<'_, R> {
        ElectricityUsageIterator::new(&mut self.csv_reader)
    }
}

/// Iterates the [`PgeElectrictyRecord`]s in a PG&E electricity usage file
pub struct ElectricityUsageIterator<'a, R> {
    source: csv::DeserializeRecordsIter<'a, BufReader<R>, PgeElectricityRecord>,
}

impl<'a, R: Read> ElectricityUsageIterator<'a, R> {
    fn new(
        input: &'a mut csv::Reader<BufReader<R>>,
    ) -> ElectricityUsageIterator<'a, R> {
        ElectricityUsageIterator { source: input.deserialize() }
    }
}

impl<'a, R: Read> Iterator for ElectricityUsageIterator<'a, R> {
    type Item = Result<NetEnergyUsed, anyhow::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.source.next().map(|result| {
            result
                .context("reading record from PG&E electricity file")
                .and_then(NetEnergyUsed::try_from)
        })
    }
}

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
    cost: Option<String>,
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

        let timestamp_start_utc =
            chrono::Local.from_local_datetime(&start_time)
                .single()
                .ok_or_else(|| {
                    anyhow!("reading start time for record: {:?}", input_record)
                })?
                .with_timezone(&chrono::Utc);
        Ok(NetEnergyUsed {
            timestamp_start_utc,
            net_used_wh: WattHours::from_kwh(input_record.usage),
        })
    }
}
