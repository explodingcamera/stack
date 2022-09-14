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
pub struct ThirdPartyGetUser200ResponseOneOf1 {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl ThirdPartyGetUser200ResponseOneOf1 {
    pub fn new() -> ThirdPartyGetUser200ResponseOneOf1 {
        ThirdPartyGetUser200ResponseOneOf1 {
            status: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "UNKNOWN_USER_ID_ERROR")]
    UserIdError,
    #[serde(rename = "UNKNOWN_THIRD_PARTY_USER_ERROR")]
    ThirdPartyUserError,
}

impl Default for Status {
    fn default() -> Status {
        Self::UserIdError
    }
}
