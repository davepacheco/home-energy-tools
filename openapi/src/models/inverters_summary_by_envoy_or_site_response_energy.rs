/*
 * The Enlighten Systems API
 *
 * The Enlighten Systems API is a JSON-based API that provides access to performance data for a PV system. By using the Enlighten Systems API, you agree to the Enphase Energy API License Agreement.  Please note that the Enlighten Systems API does not provide performance data at a panel or microinverter level.
 *
 * The version of the OpenAPI document: 2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InvertersSummaryByEnvoyOrSiteResponseEnergy : Returns the lifetime energy of the Microinverter. If the system has never produced energy, the energy value would be 0.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InvertersSummaryByEnvoyOrSiteResponseEnergy {
    #[serde(rename = "value")]
    pub value: i32,
    #[serde(rename = "units")]
    pub units: Units,
    #[serde(rename = "precision")]
    pub precision: i32,
}

impl InvertersSummaryByEnvoyOrSiteResponseEnergy {
    /// Returns the lifetime energy of the Microinverter. If the system has never produced energy, the energy value would be 0.
    pub fn new(value: i32, units: Units, precision: i32) -> InvertersSummaryByEnvoyOrSiteResponseEnergy {
        InvertersSummaryByEnvoyOrSiteResponseEnergy {
            value,
            units,
            precision,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Units {
    #[serde(rename = "Wh")]
    Wh,
    #[serde(rename = "kJ")]
    KJ,
}

