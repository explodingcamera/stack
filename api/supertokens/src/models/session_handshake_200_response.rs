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
pub struct SessionHandshake200Response {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::StatusOk>,
    #[serde(rename = "jwtSigningPublicKey", skip_serializing_if = "Option::is_none")]
    pub jwt_signing_public_key: Option<String>,
    #[serde(rename = "jwtSigningPublicKeyExpiryTime", skip_serializing_if = "Option::is_none")]
    pub jwt_signing_public_key_expiry_time: Option<f32>,
    #[serde(rename = "jwtSigningPublicKeyList", skip_serializing_if = "Option::is_none")]
    pub jwt_signing_public_key_list: Option<Vec<crate::models::JwtSigningPublicKeyListInner>>,
    #[serde(rename = "accessTokenBlacklistingEnabled", skip_serializing_if = "Option::is_none")]
    pub access_token_blacklisting_enabled: Option<bool>,
    #[serde(rename = "accessTokenValidity", skip_serializing_if = "Option::is_none")]
    pub access_token_validity: Option<f32>,
    #[serde(rename = "refreshTokenValidity", skip_serializing_if = "Option::is_none")]
    pub refresh_token_validity: Option<f32>,
}

impl SessionHandshake200Response {
    pub fn new() -> SessionHandshake200Response {
        SessionHandshake200Response {
            status: None,
            jwt_signing_public_key: None,
            jwt_signing_public_key_expiry_time: None,
            jwt_signing_public_key_list: None,
            access_token_blacklisting_enabled: None,
            access_token_validity: None,
            refresh_token_validity: None,
        }
    }
}

