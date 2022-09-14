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
pub struct CreateSignedJwt200Response {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "jwt", skip_serializing_if = "Option::is_none")]
    pub jwt: Option<String>,
}

impl CreateSignedJwt200Response {
    pub fn new() -> CreateSignedJwt200Response {
        CreateSignedJwt200Response {
            status: None,
            jwt: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "UNSUPPORTED_ALGORITHM_ERROR")]
    UnsupportedAlgorithmError,
}

impl Default for Status {
    fn default() -> Status {
        Self::UnsupportedAlgorithmError
    }
}

