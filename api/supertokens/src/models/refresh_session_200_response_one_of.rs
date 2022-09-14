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
pub struct RefreshSession200ResponseOneOf {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::StatusOk>,
    #[serde(rename = "session", skip_serializing_if = "Option::is_none")]
    pub session: Option<Box<crate::models::Session>>,
    #[serde(rename = "accessToken", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<Box<crate::models::CookieInfo>>,
    #[serde(rename = "refreshToken", skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<Box<crate::models::CookieInfo>>,
    #[serde(rename = "idRefreshToken", skip_serializing_if = "Option::is_none")]
    pub id_refresh_token: Option<Box<crate::models::CookieInfo>>,
    #[serde(rename = "antiCsrfToken", skip_serializing_if = "Option::is_none")]
    pub anti_csrf_token: Option<String>,
}

impl RefreshSession200ResponseOneOf {
    pub fn new() -> RefreshSession200ResponseOneOf {
        RefreshSession200ResponseOneOf {
            status: None,
            session: None,
            access_token: None,
            refresh_token: None,
            id_refresh_token: None,
            anti_csrf_token: None,
        }
    }
}

