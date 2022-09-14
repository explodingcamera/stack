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
pub struct PasswordlessListCodes200Response {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::StatusOk>,
    #[serde(rename = "devices", skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<crate::models::PasswordlessListCodes200ResponseDevicesInner>>,
}

impl PasswordlessListCodes200Response {
    pub fn new() -> PasswordlessListCodes200Response {
        PasswordlessListCodes200Response {
            status: None,
            devices: None,
        }
    }
}


