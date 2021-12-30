//! Summarize the local data

use anyhow::bail;
use anyhow::Context;
use solar_data::common::SolarProductionReader;
use solar_data::data_aggregator::{DataAggregator, Source};
use solar_data::pge::ElectricityUsageReader;
use std::fs;
use std::path::Path;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "report")]
#[structopt(no_version)]
#[structopt(about = "summarize local data")]
struct Args {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::from_args();
    Ok(report(&args)?)
}

fn report(_: &Args) -> Result<(), anyhow::Error> {
    let mut aggr = DataAggregator::new();
    load_production_data(&mut aggr, Path::new("local-data/production"))
        .context("loading solar data")?;
    load_pge_data(&mut aggr, Path::new("local-data/pge"))
        .context("loading PG&E data")?;

    let nwarnings = aggr.nwarnings;

    eprintln!("warnings: {}", aggr.nwarnings);
    eprintln!("production sources: {}", aggr.nprodsources);
    eprintln!("production records: {}", aggr.nprodrecords);
    eprintln!("production records combined: {}", aggr.nproddupsok);
    eprintln!("net usage  sources: {}", aggr.nusagesources);
    eprintln!("net usage  records: {}", aggr.nusagerecords);
    eprintln!("net usage  records combined: {}", aggr.nusagedupsok);

    if nwarnings > 0 {
        bail!(
            "bailing after {} warning{}",
            nwarnings,
            if nwarnings == 1 { "" } else { "s" }
        );
    }

    Ok(())
}

fn load_production_data(
    aggr: &mut DataAggregator,
    path: &Path,
) -> Result<(), anyhow::Error> {
    eprintln!("loading production data from {:?}", path);
    let dirents = fs::read_dir(path)
        .with_context(|| format!("readdir {:?}", path.display()))?;
    for maybe_item in dirents {
        let item = maybe_item
            .with_context(|| format!("readdir {:?} entry", path.display()))?;
        let filepath = item.path();
        eprintln!("loading production data from {:?}", filepath);
        let file = fs::File::open(&filepath)
            .with_context(|| format!("read {:?}", filepath))?;
        let mut solar_reader = SolarProductionReader::new(file);
        aggr.load_production(
            Source::new(&filepath.display().to_string()),
            solar_reader.records(),
        )
        .with_context(|| format!("loading data from {:?}", filepath))?;
    }

    Ok(())
}

// TODO-cleanup commonize with load_production_data
fn load_pge_data(
    aggr: &mut DataAggregator,
    path: &Path,
) -> Result<(), anyhow::Error> {
    eprintln!("loading PG&E data from {:?}", path);
    let dirents = fs::read_dir(path)
        .with_context(|| format!("readdir {:?}", path.display()))?;
    for maybe_item in dirents {
        let item = maybe_item
            .with_context(|| format!("readdir {:?} entry", path.display()))?;
        let filepath = item.path();
        let name = item.file_name();
        let name_str = name.to_string_lossy();
        if !name_str.starts_with("pge_electric_interval_data_")
            || !name_str.ends_with(".csv")
        {
            eprintln!("skipping {:?}", filepath);
            continue;
        }
        eprintln!("loading PG&E data from {:?}", filepath);
        let file = fs::File::open(&filepath)
            .with_context(|| format!("read {:?}", filepath))?;
        let mut data_reader = ElectricityUsageReader::new(file)
            .with_context(|| format!("load initial {:?}", filepath))?;
        aggr.load_net_usage(
            Source::new(&filepath.display().to_string()),
            data_reader.records(),
        )
        .with_context(|| format!("loading data from {:?}", filepath))?;
    }

    Ok(())
}
