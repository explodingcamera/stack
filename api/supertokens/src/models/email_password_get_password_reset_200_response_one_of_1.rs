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
pub struct EmailPasswordGetPasswordReset200ResponseOneOf1 {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl EmailPasswordGetPasswordReset200ResponseOneOf1 {
    pub fn new() -> EmailPasswordGetPasswordReset200ResponseOneOf1 {
        EmailPasswordGetPasswordReset200ResponseOneOf1 {
            status: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "RESET_PASSWORD_INVALID_TOKEN_ERROR")]
    ResetPasswordInvalidTokenError,
}

impl Default for Status {
    fn default() -> Status {
        Self::ResetPasswordInvalidTokenError
    }
}

