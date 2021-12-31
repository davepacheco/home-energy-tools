//! Data structure for storing net usage and production data
// TODO There's probably a crate well-suited for this.

use crate::common::EnergyProduced;
use crate::common::NetEnergyUsed;
use crate::common::WattHours;
use anyhow::bail;
use chrono::Datelike;
use chrono::NaiveDateTime;
use chrono::Timelike;
use std::collections::BTreeMap;
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

// TODO Would like a validate() function that finalizes this and does any other
// validation.  The main extra validation would be that we're not missing any
// expected data.  (e.g., have all solar data after some preconfigured date;
// don't have any gaps in net usage data)
pub struct DataLoader {
    hourly_data: BTreeMap<chrono::DateTime<chrono::Utc>, HourlyData>,
    pub nerrors: usize,
    pub nwarnings: usize,
    pub nproddupsok: usize,
    pub nprodsources: usize,
    pub nprodrecords: usize,
    pub nprodconflicts: usize,
    pub nusagedupsok: usize,
    pub nusagesources: usize,
    pub nusagerecords: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct HourlyData {
    production: Option<(Rc<Source>, WattHours)>,
    net_usage: Option<(Rc<Source>, WattHours)>,
}

impl DataLoader {
    pub fn new() -> DataLoader {
        DataLoader {
            hourly_data: BTreeMap::new(),
            nerrors: 0,
            nwarnings: 0,
            nproddupsok: 0,
            nprodsources: 0,
            nprodrecords: 0,
            nprodconflicts: 0,
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
        self.nprodsources += 1;
        let mut source_map = BTreeMap::new();
        for record in iter {
            match record {
                Ok(r) => {
                    self.load_production_record(&mut source_map, r);
                }
                Err(error) => {
                    eprintln!("warn: {:#}", error);
                    self.nwarnings += 1;
                }
            }
        }

        self.merge_production(source, source_map)
    }

    fn load_production_record(
        &mut self,
        source_map: &mut BTreeMap<chrono::DateTime<chrono::Utc>, WattHours>,
        energy_produced: EnergyProduced,
    ) {
        let start = &energy_produced.datetime_utc;
        let key_timestamp = start
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap();
        // TODO-optimization In almost all cases, there will only ever be one
        // record in the input for a given hour, and it comes in sorted.  (The
        // only time we currently see a dup here is for the one hour per year
        // when DST goes backwards.)  So we don't really need to build a whole
        // map and then merge it into the canonical one.  We could merge entries
        // one-by-one into the canonical one.  This approach ensures that we (1)
        // correctly record the extra DST-backwards record, (2) correctly ignore
        // totally duplicate data (e.g., when someone has provided CSVs with
        // overlapping dates with the same data), and (3) correctly identify
        // overlapping, non-identical data.
        let hourly = source_map
            .entry(key_timestamp)
            .or_insert_with(|| WattHours::from(0));
        *hourly += energy_produced.energy_wh;
        self.nprodrecords += 1;
    }

    fn merge_production(
        &mut self,
        source: Source,
        source_map: BTreeMap<chrono::DateTime<chrono::Utc>, WattHours>,
    ) -> Result<(), anyhow::Error> {
        let source = Rc::new(source);
        // TODO-optimization it might be slightly faster to walk both trees in
        // sorted order, instead of walking one and doing lookups in the other.
        for (hour, nproduced) in source_map.into_iter() {
            if let Some(hourly) = self.hourly_data.get_mut(&hour) {
                if let Some((ref other_source, ref other_nproduced)) =
                    hourly.production
                {
                    if *other_nproduced == nproduced {
                        // The user provided overlapping data that matches
                        // exactly.  No problem -- ignore the new data point.
                        self.nproddupsok += 1;
                        continue;
                    }

                    // The user provided overlapping data that does not match.
                    // This is almost certainly a mistake and we don't want to
                    // just add it in and produced garbage data.
                    bail!(
                        "found different data from two different sources for \
                        the same time period (hour = {}, source {:?} reports \
                        {:?} Wh, source {:?} reports {:?} Wh)",
                        hour,
                        other_source,
                        other_nproduced,
                        source,
                        nproduced
                    );
                } else {
                    hourly.production = Some((source.clone(), nproduced));
                }
            } else {
                self.hourly_data.insert(
                    hour,
                    HourlyData {
                        production: Some((source.clone(), nproduced)),
                        net_usage: None,
                    },
                );
            }
        }
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
        let mut source_map = BTreeMap::new();
        self.nusagesources += 1;
        for record in iter {
            match record {
                Ok(r) => {
                    self.load_net_usage_record(&mut source_map, r);
                }
                Err(error) => {
                    eprintln!("warn: {:#}", error);
                    self.nwarnings += 1;
                }
            }
        }

        self.merge_net_usage(source, source_map)
    }

    // TODO-cleanup commonize with load_production_record
    fn load_net_usage_record(
        &mut self,
        source_map: &mut BTreeMap<chrono::DateTime<chrono::Utc>, WattHours>,
        energy_used: NetEnergyUsed,
    ) {
        let start = &energy_used.timestamp_start_utc;
        let key_timestamp = start
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap();
        // TODO See comment in load_production_record().
        let hourly = source_map
            .entry(key_timestamp)
            .or_insert_with(|| WattHours::from(0));
        *hourly += energy_used.net_used_wh;
        self.nusagerecords += 1;
    }

    // TODO-cleanup commonize with merge_production
    fn merge_net_usage(
        &mut self,
        source: Source,
        source_map: BTreeMap<chrono::DateTime<chrono::Utc>, WattHours>,
    ) -> Result<(), anyhow::Error> {
        let source = Rc::new(source);
        // TODO-optimization it might be slightly faster to walk both trees in
        // sorted order, instead of walking one and doing lookups in the other.
        for (hour, nnet_used) in source_map.into_iter() {
            if let Some(hourly) = self.hourly_data.get_mut(&hour) {
                if let Some((ref other_source, other_nnet_used)) =
                    hourly.net_usage
                {
                    if other_nnet_used == nnet_used {
                        // The user provided overlapping data that matches
                        // exactly.  No problem -- ignore the new data point.
                        self.nusagedupsok += 1;
                        continue;
                    }

                    // The user provided overlapping data that does not match.
                    // This is almost certainly a mistake and we don't want to
                    // just add it in and produced garbage data.
                    bail!(
                        "found different data from two different sources for \
                        the same time period (hour = {}, source {:?} reports \
                        {:?} Wh, source {:?} reports {:?} Wh)",
                        hour,
                        other_source,
                        other_nnet_used,
                        source,
                        nnet_used
                    );
                } else {
                    hourly.net_usage = Some((source.clone(), nnet_used));
                }
            } else {
                self.hourly_data.insert(
                    hour,
                    HourlyData {
                        net_usage: Some((source.clone(), nnet_used)),
                        production: None,
                    },
                );
            }
        }

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
    fn new(aggr: &'a DataLoader) -> MonthIterator<'a> {
        MonthIterator { iter: aggr.hourly_data.iter().peekable() }
    }
}

// TODO-coverage write tests
fn summarize_hourly_energy(
    hourly_energy: &HourlyData,
) -> (WattHours, WattHours) {
    let produced = hourly_energy
        .production
        .as_ref()
        .map(|(_, wh)| *wh)
        .unwrap_or_else(|| WattHours::from(0i32));
    let net_used = hourly_energy
        .net_usage
        .as_ref()
        .map(|(_, wh)| *wh)
        .unwrap_or_else(|| WattHours::from(0i32));
    (produced, net_used)
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
        let (produced, net_used) = summarize_hourly_energy(hourly_energy);
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
            let (produced, net_used) = summarize_hourly_energy(energy);
            rv.produced += produced;
            rv.net_used += net_used;
        }

        rv.consumed += rv.net_used;
        rv.consumed += rv.produced;

        Some(rv)
    }
}