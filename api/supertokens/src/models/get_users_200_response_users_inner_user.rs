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
pub struct GetUsers200ResponseUsersInnerUser {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "timeJoined", skip_serializing_if = "Option::is_none")]
    pub time_joined: Option<f32>,
    #[serde(rename = "thirdparty", skip_serializing_if = "Option::is_none")]
    pub thirdparty: Option<Box<crate::models::ThirdPartyUserThirdparty>>,
    #[serde(rename = "phoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}

impl GetUsers200ResponseUsersInnerUser {
    pub fn new() -> GetUsers200ResponseUsersInnerUser {
        GetUsers200ResponseUsersInnerUser {
            id: None,
            email: None,
            time_joined: None,
            thirdparty: None,
            phone_number: None,
        }
    }
}

