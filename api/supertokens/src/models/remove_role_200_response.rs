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
pub struct RemoveRole200Response {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::StatusOk>,
    #[serde(rename = "didRoleExist", skip_serializing_if = "Option::is_none")]
    pub did_role_exist: Option<bool>,
}

impl RemoveRole200Response {
    pub fn new() -> RemoveRole200Response {
        RemoveRole200Response {
            status: None,
            did_role_exist: None,
        }
    }
}


