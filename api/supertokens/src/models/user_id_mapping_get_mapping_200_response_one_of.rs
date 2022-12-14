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
pub struct UserIdMappingGetMapping200ResponseOneOf {
    #[serde(rename = "status")]
    pub status: crate::models::StatusOk,
    #[serde(rename = "superTokensUserId")]
    pub super_tokens_user_id: String,
    #[serde(rename = "externalUserId")]
    pub external_user_id: String,
    #[serde(rename = "externalUserIdInfo", skip_serializing_if = "Option::is_none")]
    pub external_user_id_info: Option<String>,
}

impl UserIdMappingGetMapping200ResponseOneOf {
    pub fn new(status: crate::models::StatusOk, super_tokens_user_id: String, external_user_id: String) -> UserIdMappingGetMapping200ResponseOneOf {
        UserIdMappingGetMapping200ResponseOneOf {
            status,
            super_tokens_user_id,
            external_user_id,
            external_user_id_info: None,
        }
    }
}


