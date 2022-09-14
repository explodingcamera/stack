/*
 * Core Driver Interface
 *
 * This is the API exposed by the SuperTokens Core. To be consumed by your backend only.
 *
 * The version of the OpenAPI document: 2.15.1
 * Contact: team@supertokens.io
 * Generated by: https://openapi-generator.tech
 */

/// Jwk : A JWK that can be used to verify a JWT



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Jwk {
    #[serde(rename = "alg", skip_serializing_if = "Option::is_none")]
    pub alg: Option<crate::models::CreateJwtAlgorithm>,
    #[serde(rename = "kty", skip_serializing_if = "Option::is_none")]
    pub kty: Option<String>,
    #[serde(rename = "use", skip_serializing_if = "Option::is_none")]
    pub r#use: Option<String>,
    /// Unique identifier for the JWK
    #[serde(rename = "kid", skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
    /// X.509 Certificate Chain
    #[serde(rename = "x5c", skip_serializing_if = "Option::is_none")]
    pub x5c: Option<Vec<String>>,
}

impl Jwk {
    /// A JWK that can be used to verify a JWT
    pub fn new() -> Jwk {
        Jwk {
            alg: None,
            kty: None,
            r#use: None,
            kid: None,
            x5c: None,
        }
    }
}


