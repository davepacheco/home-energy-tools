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
pub struct ProductionMeterReadingsResponseMeterReadings {
    /// The serial number of the meter.
    #[serde(rename = "serial_number")]
    pub serial_number: String,
    /// The odometer reading, in Watt-hours.
    #[serde(rename = "value")]
    pub value: i32,
    /// The time when the reading was taken, always prior or equal to the requested `end_at`.
    #[serde(rename = "read_at")]
    pub read_at: i64,
}

impl ProductionMeterReadingsResponseMeterReadings {
    pub fn new(serial_number: String, value: i32, read_at: i64) -> ProductionMeterReadingsResponseMeterReadings {
        ProductionMeterReadingsResponseMeterReadings {
            serial_number,
            value,
            read_at,
        }
    }
}


