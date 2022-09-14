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
pub struct RefreshSession200ResponseOneOf1 {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::TokenTheftResponse>,
    #[serde(rename = "session", skip_serializing_if = "Option::is_none")]
    pub session: Option<Box<crate::models::RefreshSession200ResponseOneOf1Session>>,
}

impl RefreshSession200ResponseOneOf1 {
    pub fn new() -> RefreshSession200ResponseOneOf1 {
        RefreshSession200ResponseOneOf1 {
            status: None,
            session: None,
        }
    }
}

