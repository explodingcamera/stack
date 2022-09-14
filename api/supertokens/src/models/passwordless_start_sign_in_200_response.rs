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
pub struct PasswordlessStartSignIn200Response {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// A random identifier that can be used to identify the login attempt/device.
    #[serde(rename = "preAuthSessionId", skip_serializing_if = "Option::is_none")]
    pub pre_auth_session_id: Option<String>,
    /// Uniquely identifies a code
    #[serde(rename = "codeId", skip_serializing_if = "Option::is_none")]
    pub code_id: Option<String>,
    /// A random identifier that should be stored on the device that started the sign in process.
    #[serde(rename = "deviceId", skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// This can be used to authenticate the user when paired with the deviceId
    #[serde(rename = "userInputCode", skip_serializing_if = "Option::is_none")]
    pub user_input_code: Option<String>,
    /// URL-safe string that can be used to authenticate the user, without the deviceId
    #[serde(rename = "linkCode", skip_serializing_if = "Option::is_none")]
    pub link_code: Option<String>,
    #[serde(rename = "timeCreated", skip_serializing_if = "Option::is_none")]
    pub time_created: Option<f32>,
    /// The maximum lifetime of the code in milliseconds
    #[serde(rename = "codeLifetime", skip_serializing_if = "Option::is_none")]
    pub code_lifetime: Option<f32>,
}

impl PasswordlessStartSignIn200Response {
    pub fn new() -> PasswordlessStartSignIn200Response {
        PasswordlessStartSignIn200Response {
            status: None,
            pre_auth_session_id: None,
            code_id: None,
            device_id: None,
            user_input_code: None,
            link_code: None,
            time_created: None,
            code_lifetime: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "RESTART_FLOW_ERROR")]
    RestartFlowError,
    #[serde(rename = "USER_INPUT_CODE_ALREADY_USED_ERROR")]
    UserInputCodeAlreadyUsedError,
}

impl Default for Status {
    fn default() -> Status {
        Self::RestartFlowError
    }
}
