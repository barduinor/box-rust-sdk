/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`delete_legal_hold_policies_id`]
#[derive(Clone, Debug, Default)]
pub struct DeleteLegalHoldPoliciesIdParams {
    /// The ID of the legal hold policy
    pub legal_hold_policy_id: String
}

/// struct for passing parameters to the method [`get_legal_hold_policies`]
#[derive(Clone, Debug, Default)]
pub struct GetLegalHoldPoliciesParams {
    /// Limits results to policies for which the names start with this search term. This is a case-insensitive prefix.
    pub policy_name: Option<String>,
    /// A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested.
    pub fields: Option<Vec<String>>,
    /// Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`.
    pub marker: Option<String>,
    /// The maximum number of items to return per page.
    pub limit: Option<i64>
}

/// struct for passing parameters to the method [`get_legal_hold_policies_id`]
#[derive(Clone, Debug, Default)]
pub struct GetLegalHoldPoliciesIdParams {
    /// The ID of the legal hold policy
    pub legal_hold_policy_id: String
}

/// struct for passing parameters to the method [`post_legal_hold_policies`]
#[derive(Clone, Debug, Default)]
pub struct PostLegalHoldPoliciesParams {
    pub post_legal_hold_policies_request: Option<crate::models::PostLegalHoldPoliciesRequest>
}

/// struct for passing parameters to the method [`put_legal_hold_policies_id`]
#[derive(Clone, Debug, Default)]
pub struct PutLegalHoldPoliciesIdParams {
    /// The ID of the legal hold policy
    pub legal_hold_policy_id: String,
    pub put_legal_hold_policies_id_request: Option<crate::models::PutLegalHoldPoliciesIdRequest>
}


/// struct for typed errors of method [`delete_legal_hold_policies_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteLegalHoldPoliciesIdError {
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_legal_hold_policies`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLegalHoldPoliciesError {
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_legal_hold_policies_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLegalHoldPoliciesIdError {
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_legal_hold_policies`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostLegalHoldPoliciesError {
    Status400(crate::models::ClientError),
    Status409(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_legal_hold_policies_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutLegalHoldPoliciesIdError {
    Status409(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}


/// Delete an existing legal hold policy.  This is an asynchronous process. The policy will not be fully deleted yet when the response returns.
pub async fn delete_legal_hold_policies_id(configuration: &configuration::Configuration, params: DeleteLegalHoldPoliciesIdParams) -> Result<(), Error<DeleteLegalHoldPoliciesIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let legal_hold_policy_id = params.legal_hold_policy_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/legal_hold_policies/{legal_hold_policy_id}", local_var_configuration.base_path, legal_hold_policy_id=crate::apis::urlencode(legal_hold_policy_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteLegalHoldPoliciesIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves a list of legal hold policies that belong to an enterprise.
pub async fn get_legal_hold_policies(configuration: &configuration::Configuration, params: GetLegalHoldPoliciesParams) -> Result<crate::models::LegalHoldPolicies, Error<GetLegalHoldPoliciesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let policy_name = params.policy_name;
    let fields = params.fields;
    let marker = params.marker;
    let limit = params.limit;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/legal_hold_policies", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = policy_name {
        local_var_req_builder = local_var_req_builder.query(&[("policy_name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("fields".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("fields", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = marker {
        local_var_req_builder = local_var_req_builder.query(&[("marker", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetLegalHoldPoliciesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a legal hold policy.
pub async fn get_legal_hold_policies_id(configuration: &configuration::Configuration, params: GetLegalHoldPoliciesIdParams) -> Result<crate::models::LegalHoldPolicy, Error<GetLegalHoldPoliciesIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let legal_hold_policy_id = params.legal_hold_policy_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/legal_hold_policies/{legal_hold_policy_id}", local_var_configuration.base_path, legal_hold_policy_id=crate::apis::urlencode(legal_hold_policy_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetLegalHoldPoliciesIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create a new legal hold policy.
pub async fn post_legal_hold_policies(configuration: &configuration::Configuration, params: PostLegalHoldPoliciesParams) -> Result<crate::models::LegalHoldPolicy, Error<PostLegalHoldPoliciesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let post_legal_hold_policies_request = params.post_legal_hold_policies_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/legal_hold_policies", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&post_legal_hold_policies_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PostLegalHoldPoliciesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update legal hold policy.
pub async fn put_legal_hold_policies_id(configuration: &configuration::Configuration, params: PutLegalHoldPoliciesIdParams) -> Result<crate::models::LegalHoldPolicy, Error<PutLegalHoldPoliciesIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let legal_hold_policy_id = params.legal_hold_policy_id;
    let put_legal_hold_policies_id_request = params.put_legal_hold_policies_id_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/legal_hold_policies/{legal_hold_policy_id}", local_var_configuration.base_path, legal_hold_policy_id=crate::apis::urlencode(legal_hold_policy_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&put_legal_hold_policies_id_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PutLegalHoldPoliciesIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

