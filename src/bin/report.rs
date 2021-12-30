//! Summarize the local data

use anyhow::bail;
use anyhow::Context;
use solar_data::common::SolarProductionReader;
use solar_data::data_aggregator::{DataAggregator, Source};
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

    let nwarnings = aggr.nwarnings;

    eprintln!("warnings: {}", aggr.nwarnings);
    eprintln!("production sources: {}", aggr.nprodsources);
    eprintln!("production records: {}", aggr.nprodrecords);
    eprintln!("production records aggregated: {}", aggr.nproddupsok);

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
