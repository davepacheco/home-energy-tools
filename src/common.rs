//! Data structures agnostic to the data source

use std::io::Read;
use std::ops::AddAssign;
use std::ops::SubAssign;

use anyhow::Context;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

/// Wraps `anyhow::Error` in something implementing `std::error::Error`
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error(#[from] anyhow::Error);

/// Describes an amount of energy used during one calendar hour
//
// Design notes:
//
// PG&E input data includes start and end timestamp.  We leave out the end
// timestamp and instead declare that the covered interval is one hour.  (PG&E's
// timestamps are only to-the-minute, while we store more precise timestamps.
// So a straight translation of theirs would imply that we cover only 59 minutes
// of every hour.)  We do validate when reading the PG&E input that each record
// covers an hour.
//
// PG&E input data is local time.  How do we know?  I'm not sure it's
// documented.  However, we observe what appears to be a backwards daylight
// savings transition at exactly when we'd expect to at 2021-11-07 01:00 local
// time (Pacific time in the United States).  PG&E reports the 01:00 twice,
// which makes sense because we switched from PDT to PST at 02:00.  We store
// timestamps in UTC to avoid dealing with time zone nonsense.  But we're not
// totally free of it: in cases of daylight savings moving the wall clock
// backward (as in the autumn in the United States), the same exact start end
// time is repeated.  As a result, we will wind up emitting two records with the
// same UTC start timestamp.
#[derive(Debug, Deserialize, Serialize)]
pub struct NetEnergyUsed {
    pub timestamp_start_utc: DateTime<Utc>,
    pub net_used_wh: WattHours,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct WattHours(i64);

impl WattHours {
    pub fn from_kwh(kwh: f64) -> WattHours {
        WattHours((kwh * 1000f64).round() as i64)
    }

    pub fn as_kwh(&self) -> f64 {
        (self.0 as f64) / 1000f64
    }
}

impl From<i32> for WattHours {
    fn from(value: i32) -> Self {
        WattHours(i64::from(value))
    }
}

impl AddAssign for WattHours {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl SubAssign for WattHours {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

/// Represents energy produced in a calendar hour
#[derive(Debug, Deserialize, Serialize)]
pub struct EnergyProduced {
    pub datetime_utc: chrono::DateTime<chrono::Utc>,
    pub datetime_local: chrono::DateTime<chrono::Local>,
    pub energy_wh: WattHours,
}

/// Represents an amount of energy over a given time period
///
/// Similar to [`EnergyProduced`] or [`NetEnergyUsed`], but agnostic to the
/// source.
// TODO-cleanup Should these be combined?
#[derive(Debug)]
pub struct Energy {
    pub datetime: chrono::DateTime<chrono::Utc>,
    pub energy_wh: WattHours,
}

impl From<EnergyProduced> for Energy {
    fn from(e: EnergyProduced) -> Self {
        Energy { datetime: e.datetime_utc, energy_wh: e.energy_wh }
    }
}

impl From<NetEnergyUsed> for Energy {
    fn from(u: NetEnergyUsed) -> Self {
        Energy { datetime: u.timestamp_start_utc, energy_wh: u.net_used_wh }
    }
}

/// Reads our (custom) CSV format describing solar production
pub struct SolarProductionReader<R> {
    csv_reader: csv::Reader<R>,
}

impl<R: Read> SolarProductionReader<R> {
    pub fn new(input: R) -> SolarProductionReader<R> {
        SolarProductionReader {
            csv_reader: csv::ReaderBuilder::new().from_reader(input),
        }
    }

    pub fn records(&mut self) -> SolarProductionIterator<'_, R> {
        SolarProductionIterator::new(&mut self.csv_reader)
    }
}

pub struct SolarProductionIterator<'a, R> {
    source: csv::DeserializeRecordsIter<'a, R, EnergyProduced>,
}

impl<'a, R: Read> SolarProductionIterator<'a, R> {
    fn new(input: &'a mut csv::Reader<R>) -> SolarProductionIterator<'a, R> {
        SolarProductionIterator { source: input.deserialize() }
    }
}

impl<'a, R: Read> Iterator for SolarProductionIterator<'a, R> {
    type Item = Result<EnergyProduced, anyhow::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.source.next().map(|result| {
            result.context("reading record from solar production file")
        })
    }
}
