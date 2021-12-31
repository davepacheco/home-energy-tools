//! Summarize the local data

use anyhow::bail;
use anyhow::Context;
use home_energy_tools::common::SolarProductionReader;
use home_energy_tools::data_aggregator::DataIterator;
use home_energy_tools::data_aggregator::{DataLoader, Source};
use home_energy_tools::pge::ElectricityUsageReader;
use std::fs;
use std::fs::OpenOptions;
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
    let mut aggr = DataLoader::new();
    load_production_data(&mut aggr, Path::new("local-data/production"))
        .context("loading solar data")?;
    load_pge_data(&mut aggr, Path::new("local-data/pge"))
        .context("loading PG&E data")?;

    let nwarnings = aggr.nwarnings;

    eprintln!("warnings: {}", aggr.nwarnings);
    eprintln!("production sources: {}", aggr.nprodsources);
    eprintln!("production records: {}", aggr.nprodrecords);
    eprintln!("production duplicate records skipped: {}", aggr.nproddupsok);
    eprintln!("net usage  sources: {}", aggr.nusagesources);
    eprintln!("net usage  records: {}", aggr.nusagerecords);
    eprintln!("net usage  duplicate records skipped: {}", aggr.nusagedupsok);

    if nwarnings > 0 {
        bail!(
            "bailing after {} warning{}",
            nwarnings,
            if nwarnings == 1 { "" } else { "s" }
        );
    }

    let output_dir = Path::new("generated-reports");
    fs::create_dir(output_dir)
        .with_context(|| format!("mkdir {:?}", output_dir.display()))?;
    make_report(output_dir, "yearly", aggr.years())
        .context("creating yearly report")?;
    make_report(output_dir, "monthly", aggr.months())
        .context("creating monthly report")?;
    make_report(output_dir, "daily", aggr.days())
        .context("creating daily report")?;
    make_report(output_dir, "hourly", aggr.hours())
        .context("creating hourly report")?;
    Ok(())
}

fn make_report(
    parent_dir: &Path,
    label: &str,
    iter: DataIterator<'_>,
) -> Result<(), anyhow::Error> {
    eprint!("creating {} report ... ", label);
    let filename = parent_dir.join(format!("{}.csv", label));
    let file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&filename)
        .with_context(|| format!("create {:?}", filename.display()))?;
    let mut writer = csv::Writer::from_writer(file);
    for record in iter {
        writer.serialize(record).context("write record")?;
    }
    writer.flush().context("flush")?;
    eprintln!("done.");
    Ok(())
}

fn load_production_data(
    aggr: &mut DataLoader,
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
    aggr: &mut DataLoader,
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
