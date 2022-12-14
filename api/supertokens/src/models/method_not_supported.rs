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
pub enum MethodNotSupported {
    #[serde(rename = "Method not supported")]
    MethodNotSupported,

}

impl ToString for MethodNotSupported {
    fn to_string(&self) -> String {
        match self {
            Self::MethodNotSupported => String::from("Method not supported"),
        }
    }
}

impl Default for MethodNotSupported {
    fn default() -> MethodNotSupported {
        Self::MethodNotSupported
    }
}




