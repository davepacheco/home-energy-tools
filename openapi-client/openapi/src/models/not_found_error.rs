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
pub struct NotFoundError {
    #[serde(rename = "reason")]
    pub reason: String,
    #[serde(rename = "errorMessages")]
    pub error_messages: Vec<String>,
}

impl NotFoundError {
    pub fn new(reason: String, error_messages: Vec<String>) -> NotFoundError {
        NotFoundError {
            reason,
            error_messages,
        }
    }
}


