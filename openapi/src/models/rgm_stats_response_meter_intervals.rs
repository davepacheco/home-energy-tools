/*
 * The Enlighten Systems API
 *
 * The Enlighten Systems API is a JSON-based API that provides access to performance data for a PV system. By using the Enlighten Systems API, you agree to the Enphase Energy API License Agreement.  Please note that the Enlighten Systems API does not provide performance data at a panel or microinverter level.
 *
 * The version of the OpenAPI document: 2.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RgmStatsResponseMeterIntervals {
    /// Serial number of the revenue grade meter.
    #[serde(rename = "meter_serial_number")]
    pub meter_serial_number: String,
    /// Serial number of the reporting envoy.
    #[serde(rename = "envoy_serial_number")]
    pub envoy_serial_number: String,
    /// Individual meter level interval.
    #[serde(rename = "intervals")]
    pub intervals: Vec<crate::models::RgmStatsResponseIntervals1>,
}

impl RgmStatsResponseMeterIntervals {
    pub fn new(meter_serial_number: String, envoy_serial_number: String, intervals: Vec<crate::models::RgmStatsResponseIntervals1>) -> RgmStatsResponseMeterIntervals {
        RgmStatsResponseMeterIntervals {
            meter_serial_number,
            envoy_serial_number,
            intervals,
        }
    }
}


