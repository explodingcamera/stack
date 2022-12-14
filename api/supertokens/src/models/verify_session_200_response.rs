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
pub struct VerifySession200Response {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::TryRefreshTokenResponse>,
    #[serde(rename = "session", skip_serializing_if = "Option::is_none")]
    pub session: Option<Box<crate::models::Session>>,
    #[serde(rename = "accessToken", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<Box<crate::models::CookieInfo>>,
    #[serde(rename = "jwtSigningPublicKey", skip_serializing_if = "Option::is_none")]
    pub jwt_signing_public_key: Option<String>,
    #[serde(rename = "jwtSigningPublicKeyExpiryTime", skip_serializing_if = "Option::is_none")]
    pub jwt_signing_public_key_expiry_time: Option<f32>,
    #[serde(rename = "jwtSigningPublicKeyList", skip_serializing_if = "Option::is_none")]
    pub jwt_signing_public_key_list: Option<Vec<crate::models::JwtSigningPublicKeyListInner>>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl VerifySession200Response {
    pub fn new() -> VerifySession200Response {
        VerifySession200Response {
            status: None,
            session: None,
            access_token: None,
            jwt_signing_public_key: None,
            jwt_signing_public_key_expiry_time: None,
            jwt_signing_public_key_list: None,
            message: None,
        }
    }
}


