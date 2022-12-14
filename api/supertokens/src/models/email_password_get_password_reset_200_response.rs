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
pub struct EmailPasswordGetPasswordReset200Response {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl EmailPasswordGetPasswordReset200Response {
    pub fn new() -> EmailPasswordGetPasswordReset200Response {
        EmailPasswordGetPasswordReset200Response {
            status: None,
            user_id: None,
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

