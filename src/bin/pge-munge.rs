//! Process PG&E usage data into CSV files more suitable for my purposes

use anyhow::Context;
use solar_data::common::NetEnergyUsed;
use solar_data::pge::PgeElectricityRecord;
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::from_args();
    Ok(munge_file(&args)?)
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
        let input_record: PgeElectricityRecord =
            result.with_context(|| format!("read record {}", i + 1))?;
        let output_record = NetEnergyUsed::try_from(input_record)?;
        csv_writer.serialize(&output_record).context("write output")?;
    }

    csv_writer.flush().context("flush output")
}
