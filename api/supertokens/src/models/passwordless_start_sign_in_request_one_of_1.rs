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
pub struct PasswordlessStartSignInRequestOneOf1 {
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
    /// This can be used to authenticate the user when paired with the deviceId
    #[serde(rename = "userInputCode", skip_serializing_if = "Option::is_none")]
    pub user_input_code: Option<String>,
}

impl PasswordlessStartSignInRequestOneOf1 {
    pub fn new(phone_number: String) -> PasswordlessStartSignInRequestOneOf1 {
        PasswordlessStartSignInRequestOneOf1 {
            phone_number,
            user_input_code: None,
        }
    }
}

