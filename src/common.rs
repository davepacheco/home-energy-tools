//! Data structures agnostic to the data source

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

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct WattHours(u64);

impl WattHours {
    pub fn from_kwh(kwh: f64) -> WattHours {
        WattHours((kwh * 1000f64).round() as u64)
    }
}

impl TryFrom<i32> for WattHours {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(WattHours(
            u64::try_from(value).with_context(|| {
                format!("converting {:?} to WattHours", value)
            })?,
        ))
    }
}

/// Represents energy produced in a calendar hour
#[derive(Deserialize, Serialize)]
pub struct EnergyProduced {
    pub datetime_utc: chrono::DateTime<chrono::Utc>,
    pub datetime_local: chrono::DateTime<chrono::Local>,
    pub energy_wh: WattHours,
}
