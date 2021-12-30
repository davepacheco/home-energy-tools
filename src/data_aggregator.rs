//! Data structure for storing net usage and production data
// TODO There's probably a crate well-suited for this.

use crate::common::EnergyProduced;
use crate::common::NetEnergyUsed;
use crate::common::WattHours;
use anyhow::ensure;
use chrono::Timelike;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::rc::Rc;

/// Label for a data source (will be a filename)
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Source(String);
impl Source {
    pub fn new(label: &str) -> Source {
        Source(label.to_owned())
    }
}

pub struct DataAggregator {
    hourly_data: BTreeMap<chrono::DateTime<chrono::Utc>, HourlyData>,
    sources: BTreeSet<Rc<Source>>,
    pub nerrors: usize,
    pub nwarnings: usize,
    pub nproddupsok: usize,
    pub nprodsources: usize,
    pub nprodrecords: usize,
    pub nusagedupsok: usize,
    pub nusagesources: usize,
    pub nusagerecords: usize,
}

// TODO This feels awfully wasteful when the vast majority of the time these
// Vecs will have 0 or 1 items
struct HourlyData {
    production: BTreeMap<Rc<Source>, WattHours>,
    net_usage: BTreeMap<Rc<Source>, WattHours>,
}

impl DataAggregator {
    pub fn new() -> DataAggregator {
        DataAggregator {
            hourly_data: BTreeMap::new(),
            sources: BTreeSet::new(),
            nerrors: 0,
            nwarnings: 0,
            nproddupsok: 0,
            nprodsources: 0,
            nprodrecords: 0,
            nusagedupsok: 0,
            nusagesources: 0,
            nusagerecords: 0,
        }
    }

    pub fn load_production<I>(
        &mut self,
        source: Source,
        iter: I,
    ) -> Result<(), anyhow::Error>
    where
        I: Iterator<Item = Result<EnergyProduced, anyhow::Error>>,
    {
        let source = Rc::new(source);
        ensure!(
            !self.sources.contains(&source),
            "attempted to load duplicate source: {:?}",
            source
        );
        self.sources.insert(source.clone());

        self.nprodsources += 1;
        for record in iter {
            let result =
                record.and_then(|r| self.load_production_record(&source, r));
            if let Err(error) = result {
                eprintln!("warn: {:#}", error);
                self.nwarnings += 1;
            }
        }

        Ok(())
    }

    fn load_production_record(
        &mut self,
        source: &Rc<Source>,
        energy_produced: EnergyProduced,
    ) -> Result<(), anyhow::Error> {
        let start = &energy_produced.datetime_utc;
        let key_timestamp = start
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap();
        let hourly =
            self.hourly_data.entry(key_timestamp).or_insert_with(|| {
                HourlyData {
                    production: BTreeMap::new(),
                    net_usage: BTreeMap::new(),
                }
            });
        if let Some(old_wh) = hourly.production.get_mut(source) {
            self.nproddupsok += 1;
            *old_wh += energy_produced.energy_wh;
        } else {
            hourly.production.insert(source.clone(), energy_produced.energy_wh);
        }

        self.nprodrecords += 1;

        Ok(())
    }

    // TODO-cleanup commonize with load_production
    pub fn load_net_usage<I>(
        &mut self,
        source: Source,
        iter: I,
    ) -> Result<(), anyhow::Error>
    where
        I: Iterator<Item = Result<NetEnergyUsed, anyhow::Error>>,
    {
        let source = Rc::new(source);
        ensure!(
            !self.sources.contains(&source),
            "attempted to load duplicate source: {:?}",
            source
        );
        self.sources.insert(source.clone());

        self.nusagesources += 1;
        for record in iter {
            let result =
                record.and_then(|r| self.load_net_usage_record(&source, r));
            if let Err(error) = result {
                eprintln!("warn: {:#}", error);
                self.nwarnings += 1;
            }
        }

        Ok(())
    }

    // TODO-cleanup commonize with load_production_record
    fn load_net_usage_record(
        &mut self,
        source: &Rc<Source>,
        energy_used: NetEnergyUsed,
    ) -> Result<(), anyhow::Error> {
        let start = &energy_used.timestamp_start_utc;
        let key_timestamp = start
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap();
        let hourly =
            self.hourly_data.entry(key_timestamp).or_insert_with(|| {
                HourlyData {
                    production: BTreeMap::new(),
                    net_usage: BTreeMap::new(),
                }
            });
        if let Some(old_wh) = hourly.net_usage.get_mut(source) {
            self.nusagedupsok += 1;
            *old_wh += energy_used.net_used_wh;
        } else {
            hourly.net_usage.insert(source.clone(), energy_used.net_used_wh);
        }

        self.nusagerecords += 1;

        Ok(())
    }
}
