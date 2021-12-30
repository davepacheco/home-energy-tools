//! Data structure for storing net usage and production data
// TODO There's probably a crate well-suited for this.

use crate::common::EnergyProduced;
use crate::common::NetEnergyUsed;
use crate::common::WattHours;
use anyhow::ensure;
use chrono::Datelike;
use chrono::NaiveDateTime;
use chrono::Timelike;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::iter::Peekable;
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
#[derive(Debug, PartialEq, Eq)]
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

    pub fn months(&self) -> MonthIterator<'_> {
        MonthIterator::new(&self)
    }
}

pub struct IntervalEnergy {
    pub interval_start: chrono::NaiveDateTime,
    pub produced: WattHours,
    pub net_used: WattHours,
    pub consumed: WattHours,
}

pub struct MonthIterator<'a> {
    iter: Peekable<
        std::collections::btree_map::Iter<
            'a,
            chrono::DateTime<chrono::Utc>,
            HourlyData,
        >,
    >,
}

impl<'a> MonthIterator<'a> {
    fn new(aggr: &'a DataAggregator) -> MonthIterator<'a> {
        MonthIterator { iter: aggr.hourly_data.iter().peekable() }
    }
}

// TODO-coverage write tests
fn summarize_hourly_energy(
    hourly_energy: &HourlyData,
) -> Option<(WattHours, WattHours)> {
    // TODO-cleanup all of this could be moved to "finalize" step on the
    // aggregator that produces a tighter representation
    let mut prod_sources = hourly_energy.production.iter();
    let produced = if let Some((source, whproduced)) = prod_sources.next() {
        for (osource, owhproduced) in prod_sources {
            if owhproduced != whproduced {
                // XXX bump warning counter
                eprintln!(
                    "production: multiple sources disagree \
                        ({:?} reports {:?} Wh, while {:?} reports {:?} Wh)",
                    source, whproduced, osource, owhproduced
                );

                return None;
            }
        }

        *whproduced
    } else {
        // TODO validate that we didn't expect data here
        WattHours::from(0i32)
    };

    // TODO-cleanup commonize with above
    let mut net_used_sources = hourly_energy.net_usage.iter();
    let net_used = if let Some((source, whused)) = net_used_sources.next() {
        for (osource, owhused) in net_used_sources {
            if owhused != whused {
                // XXX bump warning counter
                eprintln!(
                    "net usage: multiple sources disagree \
                        ({:?} reports {:?} Wh, while {:?} reports {:?} Wh)",
                    source, whused, osource, owhused
                );

                return None;
            }
        }

        *whused
    } else {
        // TODO validate that we didn't expect data here
        WattHours::from(0i32)
    };

    Some((produced, net_used))
}

// TODO-coverage write tests
fn hour_bucket_local_month(
    start_utc: &chrono::DateTime<chrono::Utc>,
) -> NaiveDateTime {
    let start_local = start_utc.with_timezone(&chrono::Local);
    let start_naive = start_local.naive_local();
    let start_month = start_naive.with_day(1).unwrap().with_hour(0).unwrap();
    assert_eq!(start_month.minute(), 0);
    assert_eq!(start_month.second(), 0);
    assert_eq!(start_month.nanosecond(), 0);
    start_month
}

impl<'a> Iterator for MonthIterator<'a> {
    type Item = IntervalEnergy;

    fn next(&mut self) -> Option<Self::Item> {
        let (hour_start, hourly_energy) = match self.iter.next() {
            Some(n) => n,
            None => return None,
        };

        // XXX TODO-cleanup this whole thing could be written much cleaner
        let start_month = hour_bucket_local_month(hour_start);
        // XXX unwrap
        let (produced, net_used) =
            summarize_hourly_energy(hourly_energy).unwrap();
        let mut rv = IntervalEnergy {
            interval_start: start_month,
            produced,
            net_used,
            consumed: WattHours::from(0i32),
        };

        rv.produced += produced;
        rv.net_used += net_used;

        while let Some((peek_start, _)) = self.iter.peek() {
            let peek_month = hour_bucket_local_month(*peek_start);
            if peek_month != start_month {
                break;
            }

            let (start, energy) = self.iter.next().unwrap();
            assert_eq!(hour_bucket_local_month(start), start_month);
            if let Some((produced, net_used)) = summarize_hourly_energy(energy)
            {
                rv.produced += produced;
                rv.net_used += net_used;
            }
        }

        rv.consumed += rv.net_used;
        rv.consumed += rv.produced;

        Some(rv)
    }
}
