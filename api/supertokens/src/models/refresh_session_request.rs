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
pub struct RefreshSessionRequest {
    #[serde(rename = "refreshToken", skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(rename = "enableAntiCsrf", skip_serializing_if = "Option::is_none")]
    pub enable_anti_csrf: Option<String>,
    #[serde(rename = "antiCsrfToken", skip_serializing_if = "Option::is_none")]
    pub anti_csrf_token: Option<String>,
}

impl RefreshSessionRequest {
    pub fn new() -> RefreshSessionRequest {
        RefreshSessionRequest {
            refresh_token: None,
            enable_anti_csrf: None,
            anti_csrf_token: None,
        }
    }
}


