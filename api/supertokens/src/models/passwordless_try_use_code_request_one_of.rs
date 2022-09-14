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
pub struct PasswordlessTryUseCodeRequestOneOf {
    /// A random identifier that can be used to identify the login attempt/device.
    #[serde(rename = "preAuthSessionId", skip_serializing_if = "Option::is_none")]
    pub pre_auth_session_id: Option<String>,
    /// URL-safe string that can be used to authenticate the user, without the deviceId
    #[serde(rename = "linkCode")]
    pub link_code: String,
}

impl PasswordlessTryUseCodeRequestOneOf {
    pub fn new(link_code: String) -> PasswordlessTryUseCodeRequestOneOf {
        PasswordlessTryUseCodeRequestOneOf {
            pre_auth_session_id: None,
            link_code,
        }
    }
}


