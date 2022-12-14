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
pub enum UnauthorisedResponse {
    #[serde(rename = "UNAUTHORISED")]
    Unauthorised,

}

impl ToString for UnauthorisedResponse {
    fn to_string(&self) -> String {
        match self {
            Self::Unauthorised => String::from("UNAUTHORISED"),
        }
    }
}

impl Default for UnauthorisedResponse {
    fn default() -> UnauthorisedResponse {
        Self::Unauthorised
    }
}




