//! Process PG&E usage data into CSV files more suitable for my purposes

use anyhow::Context;
use solar_data::pge::ElectricityUsageReader;
use std::fs::File;
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::from_args();
    Ok(munge_file(&args)?)
}

fn munge_file(args: &Args) -> Result<(), anyhow::Error> {
    let file = File::open(&args.input)
        .with_context(|| format!("open {:?}", args.input.display()))?;
    let mut reader = ElectricityUsageReader::new(file)
        .with_context(|| format!("setup {:?}", args.input.display()))?;
    let mut csv_writer = csv::Writer::from_writer(std::io::stdout());
    for record in reader.records() {
        csv_writer
            .serialize(&record.context("read record")?)
            .context("write output")?;
    }
    csv_writer.flush().context("flush output")
}
