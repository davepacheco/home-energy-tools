//! Summarize the local data

use anyhow::bail;
use anyhow::Context;
use solar_data::common::SolarProductionReader;
use solar_data::data_aggregator::{DataAggregator, Source};
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
    let file = std::io::stdin();
    if atty::is(atty::Stream::Stdin) {
        eprintln!("note: reading from stdin");
    }
    let mut solar_reader = SolarProductionReader::new(file);
    let mut aggr = DataAggregator::new();
    aggr.load_production(Source::new("stdin"), solar_reader.records())
        .context("loading data")?;
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
