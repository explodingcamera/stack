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
pub struct UnknownRoleResponse {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl UnknownRoleResponse {
    pub fn new() -> UnknownRoleResponse {
        UnknownRoleResponse {
            status: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "UNKNOWN_ROLE_ERROR")]
    UnknownRoleError,
}

impl Default for Status {
    fn default() -> Status {
        Self::UnknownRoleError
    }
}

