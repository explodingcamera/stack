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
pub enum RequestUnauthorised {
    #[serde(rename = "Invalid API key")]
    InvalidApiKey,

}

impl ToString for RequestUnauthorised {
    fn to_string(&self) -> String {
        match self {
            Self::InvalidApiKey => String::from("Invalid API key"),
        }
    }
}

impl Default for RequestUnauthorised {
    fn default() -> RequestUnauthorised {
        Self::InvalidApiKey
    }
}




