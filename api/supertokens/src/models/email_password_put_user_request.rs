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
pub struct EmailPasswordPutUserRequest {
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl EmailPasswordPutUserRequest {
    pub fn new() -> EmailPasswordPutUserRequest {
        EmailPasswordPutUserRequest {
            user_id: None,
            email: None,
            password: None,
        }
    }
}

