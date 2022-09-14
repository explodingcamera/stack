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


/// struct for typed errors of method [`third_party_get_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ThirdPartyGetUserError {
    Status400(String),
    Status401(crate::models::RequestUnauthorised),
    Status404(crate::models::NotFound),
    Status500(crate::models::InternalError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`third_party_get_user_count`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ThirdPartyGetUserCountError {
    Status400(String),
    Status401(crate::models::RequestUnauthorised),
    Status404(crate::models::NotFound),
    Status500(crate::models::InternalError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`third_party_get_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ThirdPartyGetUsersError {
    Status400(String),
    Status401(crate::models::RequestUnauthorised),
    Status404(crate::models::NotFound),
    Status500(crate::models::InternalError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`third_party_get_users_by_email`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ThirdPartyGetUsersByEmailError {
    Status400(String),
    Status401(crate::models::RequestUnauthorised),
    Status500(crate::models::InternalError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`third_party_signinup`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ThirdPartySigninupError {
    Status400(String),
    Status401(crate::models::RequestUnauthorised),
    Status404(crate::models::NotFound),
    Status500(crate::models::InternalError),
    UnknownValue(serde_json::Value),
}


/// Get a user's information. Note that there is an invisible character at the end of the path, this was to avoid a conflict with the OpenAPI spec.  
pub async fn third_party_get_user(configuration: &configuration::Configuration, cdi_version: &str, rid: Option<&str>, api_key: Option<&str>, user_id: Option<&str>, third_party_id: Option<&str>, third_party_user_id: Option<&str>) -> Result<crate::models::ThirdPartyGetUser200Response, Error<ThirdPartyGetUserError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/recipe/user⠀", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = user_id {
        local_var_req_builder = local_var_req_builder.query(&[("userId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = third_party_id {
        local_var_req_builder = local_var_req_builder.query(&[("thirdPartyId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = third_party_user_id {
        local_var_req_builder = local_var_req_builder.query(&[("thirdPartyUserId", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ThirdPartyGetUserError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get number of users. Note that there is an invisible character at the end of the path, this was to avoid a conflict with the OpenAPI spec 
pub async fn third_party_get_user_count(configuration: &configuration::Configuration, cdi_version: &str, rid: Option<&str>, api_key: Option<&str>) -> Result<crate::models::EmailPasswordGetUserCount200Response, Error<ThirdPartyGetUserCountError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/recipe/users/count⠀", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
        let local_var_entity: Option<ThirdPartyGetUserCountError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get User Pagination data. Note that there is an invisible character at the end of the path, this was to avoid a conflict with the OpenAPI spec. 
pub async fn third_party_get_users(configuration: &configuration::Configuration, cdi_version: &str, rid: Option<&str>, api_key: Option<&str>, pagination_token: Option<&str>, time_joined_order: Option<&str>, limit: Option<f32>) -> Result<crate::models::ThirdPartyGetUsers200Response, Error<ThirdPartyGetUsersError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/recipe/users⠀", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = pagination_token {
        local_var_req_builder = local_var_req_builder.query(&[("paginationToken", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = time_joined_order {
        local_var_req_builder = local_var_req_builder.query(&[("timeJoinedOrder", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ThirdPartyGetUsersError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get all users accounts associated with given email 
pub async fn third_party_get_users_by_email(configuration: &configuration::Configuration, cdi_version: &str, email: &str, rid: Option<&str>, api_key: Option<&str>) -> Result<crate::models::ThirdPartyGetUsersByEmail200Response, Error<ThirdPartyGetUsersByEmailError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/recipe/users/by-email", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("email", &email.to_string())]);
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
        let local_var_entity: Option<ThirdPartyGetUsersByEmailError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Signin/up a user 
pub async fn third_party_signinup(configuration: &configuration::Configuration, cdi_version: &str, rid: Option<&str>, api_key: Option<&str>, third_party_signinup_request: Option<crate::models::ThirdPartySigninupRequest>) -> Result<crate::models::ThirdPartySigninup200Response, Error<ThirdPartySigninupError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/recipe/signinup", local_var_configuration.base_path);
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
    local_var_req_builder = local_var_req_builder.json(&third_party_signinup_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ThirdPartySigninupError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

