/*
 * Core Driver Interface
 *
 * This is the API exposed by the SuperTokens Core. To be consumed by your backend only.
 *
 * The version of the OpenAPI document: 2.15.1
 * Contact: team@supertokens.io
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TryRefreshTokenResponse {
    #[serde(rename = "TRY_REFRESH_TOKEN")]
    TryRefreshToken,

}

impl ToString for TryRefreshTokenResponse {
    fn to_string(&self) -> String {
        match self {
            Self::TryRefreshToken => String::from("TRY_REFRESH_TOKEN"),
        }
    }
}

impl Default for TryRefreshTokenResponse {
    fn default() -> TryRefreshTokenResponse {
        Self::TryRefreshToken
    }
}




