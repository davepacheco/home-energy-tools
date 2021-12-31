//! Process PG&E usage data into CSV files more suitable for my purposes

use anyhow::Context;
use home_energy_tools::pge::ElectricityUsageReader;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "pge-munge")]
#[structopt(no_version)]
#[structopt(
    about = "convert PG&E electricity usage data file into a proper CSV \
    with UTC timestamps"
)]
struct Args {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::from_args();
    Ok(munge_file(&args)?)
}

fn munge_file(_: &Args) -> Result<(), anyhow::Error> {
    let file = std::io::stdin();
    if atty::is(atty::Stream::Stdin) {
        eprintln!("note: reading from stdin");
    }
    let mut reader = ElectricityUsageReader::new(file)
        .with_context(|| format!("setup reader from stdin"))?;
    let mut csv_writer = csv::Writer::from_writer(std::io::stdout());
    for record in reader.records() {
        csv_writer
            .serialize(&record.context("read record")?)
            .context("write output")?;
    }
    csv_writer.flush().context("flush output")
}
