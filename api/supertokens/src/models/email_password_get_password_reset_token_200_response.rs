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
pub struct EmailPasswordGetPasswordResetToken200Response {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

impl EmailPasswordGetPasswordResetToken200Response {
    pub fn new() -> EmailPasswordGetPasswordResetToken200Response {
        EmailPasswordGetPasswordResetToken200Response {
            status: None,
            token: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "UNKNOWN_USER_ID_ERROR")]
    UnknownUserIdError,
}

impl Default for Status {
    fn default() -> Status {
        Self::UnknownUserIdError
    }
}

