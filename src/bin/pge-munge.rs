//! Process PG&E usage data into CSV files more suitable for my purposes

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Context;
use chrono::TimeZone;
use lazy_static::lazy_static;
use serde::Deserialize;
use serde::Deserializer;
use std::fs::File;
use std::io::BufRead;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "pge-munge")]
#[structopt(no_version)]
#[structopt(about = "turn raw PG&E data file into something more usable")]
struct Args {
    #[structopt(parse(from_os_str), help = "path to raw PG&E file")]
    input: PathBuf,
}

/// Represents an input record
#[derive(Debug, serde::Deserialize)]
struct ElectricInputRecord {
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

/// Represents an output record
///
/// We deliberately leave out the end timestamps because it's clearer to say
/// it's implied.  Each record is an hour.  (We validate this.)
///
/// We store timestamps only in UTC to avoid dealing with time zone nonsense.
/// Note that the input data is in local time.  In cases of daylight savings
/// moving the wall clock backward (as in the autumn in the United States), the
/// same exact start and end times are repeated.  As a result, we will wind up
/// emitting two records with the same UTC start timestamp.
#[derive(Debug, serde::Serialize)]
struct OutputRecord {
    timestamp_start_utc: chrono::DateTime<chrono::Utc>,
    net_used_wh: u64,
}

// XXX copied from fetch-from-enphase
/// Wraps `anyhow::Error` in something implementing `std::error::Error`
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
struct AnyhowWrap(#[from] anyhow::Error);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::from_args();
    Ok(munge_file(&args)?)
}

lazy_static! {
    static ref EXPECTED_RECORD_INTERVAL: chrono::Duration =
        chrono::Duration::minutes(59);
}

fn munge_file(args: &Args) -> Result<(), anyhow::Error> {
    let file = File::open(&args.input)
        .with_context(|| format!("open {:?}", args.input.display()))?;
    let mut line_reader = std::io::BufReader::new(file);
    let mut buf = String::new(); // TODO-robustness it'd be nice to cap this
    const NSKIP: usize = 5;
    // TODO could validate these lines
    for i in 0..NSKIP {
        line_reader
            .read_line(&mut buf)
            .with_context(|| format!("read line {}", i + 1))?;
        buf = String::new();
    }
    let mut csv_reader = csv::ReaderBuilder::new().from_reader(line_reader);
    let mut csv_writer = csv::Writer::from_writer(std::io::stdout());
    for (i, result) in csv_reader.deserialize().enumerate() {
        let input_record: ElectricInputRecord =
            result.with_context(|| format!("read record {}", i + 1))?;
        let timestamp_start_utc = chrono::Utc
            .from_local_datetime(
                &input_record.date.and_time(input_record.start_time),
            )
            .single()
            .ok_or_else(|| anyhow!("reading time for record {}", i + 1))?;
        let timestamp_end_utc = chrono::Utc
            .from_local_datetime(
                &input_record.date.and_time(input_record.end_time),
            )
            .single()
            .ok_or_else(|| anyhow!("reading time for record {}", i + 1))?;
        let record_interval = timestamp_end_utc - timestamp_start_utc;
        if record_interval != EXPECTED_RECORD_INTERVAL {
            bail!(
                "unexpected interval covered by record \
                (expected {:?}, found {:?}, input record: {:?})",
                EXPECTED_RECORD_INTERVAL,
                record_interval,
                input_record,
            );
        }
        let output_record = OutputRecord {
            timestamp_start_utc,
            net_used_wh: (input_record.usage * 1000f64).round() as u64,
        };

        csv_writer.serialize(&output_record).context("write output")?;
    }

    csv_writer.flush().context("flush output")
}
