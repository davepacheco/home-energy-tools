//! Data structure for storing net usage and production data
// TODO There's probably a crate well-suited for this.

use crate::common::Energy;
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
        let (source_map, nrecords, nwarnings) =
            load_records(iter.map(|r| r.map(Energy::from)));
        self.nprodrecords += nrecords;
        self.nwarnings += nwarnings;
        let ndupsok = self.merge_source(source, source_map, |hourly| {
            &mut hourly.production
        })?;
        self.nproddupsok += ndupsok;
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
        self.nusagesources += 1;
        let (source_map, nrecords, nwarnings) =
            load_records(iter.map(|r| r.map(Energy::from)));
        self.nusagerecords += nrecords;
        self.nwarnings += nwarnings;
        let ndupsok = self
            .merge_source(source, source_map, |hourly| &mut hourly.net_usage)?;
        self.nusagedupsok += ndupsok;
        Ok(())
    }

    fn merge_source<F>(
        &mut self,
        source: Source,
        source_map: BTreeMap<chrono::DateTime<chrono::Utc>, WattHours>,
        which: F,
    ) -> Result<usize, anyhow::Error>
    where
        F: Fn(&mut HourlyData) -> &mut Option<(Rc<Source>, WattHours)>,
    {
        let source = Rc::new(source);
        // TODO-optimization it might be slightly faster to walk both trees in
        // sorted order, instead of walking one and doing lookups in the other.
        let mut ndupsok = 0;
        for (hour, energy_wh) in source_map.into_iter() {
            let hourly = self
                .hourly_data
                .entry(hour)
                .or_insert(HourlyData { production: None, net_usage: None });

            let datum = which(hourly);
            if let Some((ref other_source, other_energy_wh)) = datum {
                if *other_energy_wh == energy_wh {
                    // The user provided overlapping data that matches
                    // exactly.  No problem -- ignore the new data point.
                    ndupsok += 1;
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
                    other_energy_wh,
                    source,
                    energy_wh
                );
            } else {
                *datum = Some((source.clone(), energy_wh));
            }
        }

        Ok(ndupsok)
    }

    pub fn years(&self) -> DataIterator<'_> {
        DataIterator::new(&self, hour_bucket_local_year)
    }

    pub fn months(&self) -> DataIterator<'_> {
        DataIterator::new(&self, hour_bucket_local_month)
    }

    pub fn days(&self) -> DataIterator<'_> {
        DataIterator::new(&self, hour_bucket_local_day)
    }

    pub fn hours(&self) -> DataIterator<'_> {
        DataIterator::new(&self, hour_bucket_utc_hour)
    }
}

// TODO-cleanup use a struct here
pub fn load_records<I>(
    iter: I,
) -> (BTreeMap<chrono::DateTime<chrono::Utc>, WattHours>, usize, usize)
where
    I: Iterator<Item = Result<Energy, anyhow::Error>>,
{
    let mut source_map = BTreeMap::new();
    let mut nrecords = 0;
    let mut nwarnings = 0;
    for record in iter {
        match record {
            Ok(r) => {
                let start = &r.datetime;
                let key_timestamp = start
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap()
                    .with_nanosecond(0)
                    .unwrap();
                // TODO-optimization In almost all cases, there will only ever
                // be one record in the input for a given hour, and it comes in
                // sorted.  (The only time we currently see a dup here is for
                // the one hour per year when DST goes backwards.)  So we don't
                // really need to build a whole map and then merge it into the
                // canonical one.  We could merge entries one-by-one into the
                // canonical one.  This approach ensures that we (1) correctly
                // record the extra DST-backwards record, (2) correctly ignore
                // totally duplicate data (e.g., when someone has provided CSVs
                // with overlapping dates with the same data), and (3) correctly
                // identify overlapping, non-identical data.
                let hourly = source_map
                    .entry(key_timestamp)
                    .or_insert_with(|| WattHours::from(0));
                *hourly += r.energy_wh;
                nrecords += 1;
            }
            Err(error) => {
                eprintln!("warn: {:#}", error);
                nwarnings += 1;
            }
        }
    }

    (source_map, nrecords, nwarnings)
}

#[derive(serde::Serialize)]
pub struct IntervalEnergy {
    pub interval_start: chrono::NaiveDateTime,
    pub produced: WattHours,
    pub net_used: WattHours,
    pub consumed: WattHours,
}

pub struct DataIterator<'a> {
    iter: Peekable<
        std::collections::btree_map::Iter<
            'a,
            chrono::DateTime<chrono::Utc>,
            HourlyData,
        >,
    >,
    bucket_time: fn(&chrono::DateTime<chrono::Utc>) -> NaiveDateTime,
}

impl<'a> DataIterator<'a> {
    fn new(
        aggr: &'a DataLoader,
        bucket_time: fn(&chrono::DateTime<chrono::Utc>) -> NaiveDateTime,
    ) -> DataIterator<'a> {
        DataIterator { iter: aggr.hourly_data.iter().peekable(), bucket_time }
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
fn hour_bucket_local_year(
    start_utc: &chrono::DateTime<chrono::Utc>,
) -> NaiveDateTime {
    start_utc
        .with_timezone(&chrono::Local)
        .naive_local()
        .with_day(1)
        .unwrap()
        .with_hour(0)
        .unwrap()
        .with_month(1)
        .unwrap()
}

// TODO-coverage write tests
fn hour_bucket_local_month(
    start_utc: &chrono::DateTime<chrono::Utc>,
) -> NaiveDateTime {
    start_utc
        .with_timezone(&chrono::Local)
        .naive_local()
        .with_day(1)
        .unwrap()
        .with_hour(0)
        .unwrap()
}

// TODO-coverage write tests
fn hour_bucket_local_day(
    start_utc: &chrono::DateTime<chrono::Utc>,
) -> NaiveDateTime {
    start_utc.with_timezone(&chrono::Local).naive_local().with_hour(0).unwrap()
}

// TODO-coverage write tests
fn hour_bucket_utc_hour(
    start_utc: &chrono::DateTime<chrono::Utc>,
) -> NaiveDateTime {
    assert_eq!(start_utc.minute(), 0);
    assert_eq!(start_utc.second(), 0);
    assert_eq!(start_utc.nanosecond(), 0);
    start_utc.naive_local()
}

impl<'a> Iterator for DataIterator<'a> {
    type Item = IntervalEnergy;

    fn next(&mut self) -> Option<Self::Item> {
        let (hour_start, hourly_energy) = match self.iter.next() {
            Some(n) => n,
            None => return None,
        };

        // XXX TODO-cleanup this whole thing could be written much cleaner
        let start_bucket = (self.bucket_time)(hour_start);
        let (produced, net_used) = summarize_hourly_energy(hourly_energy);
        let mut rv = IntervalEnergy {
            interval_start: start_bucket,
            produced,
            net_used,
            consumed: WattHours::from(0i32),
        };

        rv.produced += produced;
        rv.net_used += net_used;

        while let Some((peek_start, _)) = self.iter.peek() {
            let peek_start_bucket = (self.bucket_time)(*peek_start);
            if peek_start_bucket != start_bucket {
                break;
            }

            let (start, energy) = self.iter.next().unwrap();
            assert_eq!((self.bucket_time)(start), start_bucket);
            let (produced, net_used) = summarize_hourly_energy(energy);
            rv.produced += produced;
            rv.net_used += net_used;
        }

        rv.consumed += rv.net_used;
        rv.consumed += rv.produced;

        Some(rv)
    }
}
