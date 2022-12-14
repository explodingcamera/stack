/*
 * Core Driver Interface
 *
 * This is the API exposed by the SuperTokens Core. To be consumed by your backend only.
 *
 * The version of the OpenAPI document: 2.15.1
 * Contact: team@supertokens.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetTelemetry200Response {
    #[serde(rename = "exists", skip_serializing_if = "Option::is_none")]
    pub exists: Option<bool>,
    #[serde(rename = "telemetryId", skip_serializing_if = "Option::is_none")]
    pub telemetry_id: Option<String>,
}

impl GetTelemetry200Response {
    pub fn new() -> GetTelemetry200Response {
        GetTelemetry200Response {
            exists: None,
            telemetry_id: None,
        }
    }
}


