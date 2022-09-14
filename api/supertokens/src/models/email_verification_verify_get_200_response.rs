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
pub struct EmailVerificationVerifyGet200Response {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::StatusOk>,
    #[serde(rename = "isVerified", skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
}

impl EmailVerificationVerifyGet200Response {
    pub fn new() -> EmailVerificationVerifyGet200Response {
        EmailVerificationVerifyGet200Response {
            status: None,
            is_verified: None,
        }
    }
}


