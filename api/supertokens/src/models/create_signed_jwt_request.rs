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
pub struct CreateSignedJwtRequest {
    /// The payload of the JWT, should be a JSON object.
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
    #[serde(rename = "algorithm", skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<crate::models::CreateJwtAlgorithm>,
    /// This is used as the value for the issuer claim in the JWT payload.
    #[serde(rename = "jwksDomain", skip_serializing_if = "Option::is_none")]
    pub jwks_domain: Option<String>,
    /// Duration in seconds, used to calculate JWT expiry
    #[serde(rename = "validity", skip_serializing_if = "Option::is_none")]
    pub validity: Option<f32>,
}

impl CreateSignedJwtRequest {
    pub fn new() -> CreateSignedJwtRequest {
        CreateSignedJwtRequest {
            payload: None,
            algorithm: None,
            jwks_domain: None,
            validity: None,
        }
    }
}

