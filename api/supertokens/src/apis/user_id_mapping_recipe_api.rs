/*
 * Core Driver Interface
 *
 * This is the API exposed by the SuperTokens Core. To be consumed by your backend only.
 *
 * The version of the OpenAPI document: 2.15.1
 * Contact: team@supertokens.io
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`user_id_mapping_create_mapping`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserIdMappingCreateMappingError {
    Status400(String),
    Status401(crate::models::RequestUnauthorised),
    Status404(crate::models::NotFound),
    Status500(crate::models::InternalError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`user_id_mapping_get_mapping`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserIdMappingGetMappingError {
    Status400(String),
    Status401(crate::models::RequestUnauthorised),
    Status404(crate::models::NotFound),
    Status500(crate::models::InternalError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`user_id_mapping_remove_mapping`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserIdMappingRemoveMappingError {
    Status400(String),
    Status401(crate::models::RequestUnauthorised),
    Status404(crate::models::NotFound),
    Status500(crate::models::InternalError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`user_id_mapping_upate_external_user_id_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserIdMappingUpateExternalUserIdInfoError {
    Status400(String),
    Status401(crate::models::RequestUnauthorised),
    Status404(crate::models::NotFound),
    Status500(crate::models::InternalError),
    UnknownValue(serde_json::Value),
}


/// Create a mapping between a SuperTokens userId and an external userId. 
pub async fn user_id_mapping_create_mapping(configuration: &configuration::Configuration, cdi_version: &str, rid: Option<&str>, api_key: Option<&str>, user_id_mapping_create_mapping_request: Option<crate::models::UserIdMappingCreateMappingRequest>) -> Result<crate::models::UserIdMappingCreateMapping200Response, Error<UserIdMappingCreateMappingError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/recipe/userid/map", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = rid {
        local_var_req_builder = local_var_req_builder.header("rid", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = api_key {
        local_var_req_builder = local_var_req_builder.header("api-key", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.header("cdi-version", cdi_version.to_string());
    local_var_req_builder = local_var_req_builder.json(&user_id_mapping_create_mapping_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UserIdMappingCreateMappingError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a UserIdMapping 
pub async fn user_id_mapping_get_mapping(configuration: &configuration::Configuration, cdi_version: &str, user_id: &str, rid: Option<&str>, api_key: Option<&str>, user_id_type: Option<&str>) -> Result<crate::models::UserIdMappingGetMapping200Response, Error<UserIdMappingGetMappingError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/recipe/userid/map", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("userId", &user_id.to_string())]);
    if let Some(ref local_var_str) = user_id_type {
        local_var_req_builder = local_var_req_builder.query(&[("userIdType", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = rid {
        local_var_req_builder = local_var_req_builder.header("rid", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = api_key {
        local_var_req_builder = local_var_req_builder.header("api-key", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.header("cdi-version", cdi_version.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UserIdMappingGetMappingError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete a mapping between a SuperTokens userId and an external userId. 
pub async fn user_id_mapping_remove_mapping(configuration: &configuration::Configuration, cdi_version: &str, rid: Option<&str>, api_key: Option<&str>, user_id_mapping_remove_mapping_request: Option<crate::models::UserIdMappingRemoveMappingRequest>) -> Result<crate::models::UserIdMappingRemoveMapping200Response, Error<UserIdMappingRemoveMappingError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/recipe/userid/map/remove", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = rid {
        local_var_req_builder = local_var_req_builder.header("rid", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = api_key {
        local_var_req_builder = local_var_req_builder.header("api-key", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.header("cdi-version", cdi_version.to_string());
    local_var_req_builder = local_var_req_builder.json(&user_id_mapping_remove_mapping_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UserIdMappingRemoveMappingError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update or delete externalUserIdInfo 
pub async fn user_id_mapping_upate_external_user_id_info(configuration: &configuration::Configuration, cdi_version: &str, rid: Option<&str>, api_key: Option<&str>, user_id_mapping_upate_external_user_id_info_request: Option<crate::models::UserIdMappingUpateExternalUserIdInfoRequest>) -> Result<crate::models::UserIdMappingUpateExternalUserIdInfo200Response, Error<UserIdMappingUpateExternalUserIdInfoError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/recipe/userid/external-user-id-info", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = rid {
        local_var_req_builder = local_var_req_builder.header("rid", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = api_key {
        local_var_req_builder = local_var_req_builder.header("api-key", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.header("cdi-version", cdi_version.to_string());
    local_var_req_builder = local_var_req_builder.json(&user_id_mapping_upate_external_user_id_info_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UserIdMappingUpateExternalUserIdInfoError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

