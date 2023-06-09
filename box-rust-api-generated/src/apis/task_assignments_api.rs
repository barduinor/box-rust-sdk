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

/// struct for passing parameters to the method [`delete_task_assignments_id`]
#[derive(Clone, Debug, Default)]
pub struct DeleteTaskAssignmentsIdParams {
    /// The ID of the task assignment.
    pub task_assignment_id: String
}

/// struct for passing parameters to the method [`get_task_assignments_id`]
#[derive(Clone, Debug, Default)]
pub struct GetTaskAssignmentsIdParams {
    /// The ID of the task assignment.
    pub task_assignment_id: String
}

/// struct for passing parameters to the method [`get_tasks_id_assignments`]
#[derive(Clone, Debug, Default)]
pub struct GetTasksIdAssignmentsParams {
    /// The ID of the task.
    pub task_id: String
}

/// struct for passing parameters to the method [`post_task_assignments`]
#[derive(Clone, Debug, Default)]
pub struct PostTaskAssignmentsParams {
    pub post_task_assignments_request: Option<crate::models::PostTaskAssignmentsRequest>
}

/// struct for passing parameters to the method [`put_task_assignments_id`]
#[derive(Clone, Debug, Default)]
pub struct PutTaskAssignmentsIdParams {
    /// The ID of the task assignment.
    pub task_assignment_id: String,
    pub put_task_assignments_id_request: Option<crate::models::PutTaskAssignmentsIdRequest>
}


/// struct for typed errors of method [`delete_task_assignments_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTaskAssignmentsIdError {
    Status404(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_task_assignments_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTaskAssignmentsIdError {
    Status404(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_tasks_id_assignments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTasksIdAssignmentsError {
    Status404(crate::models::ClientError),
    Status500(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_task_assignments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostTaskAssignmentsError {
    Status403(crate::models::ClientError),
    Status404(crate::models::ClientError),
    Status500(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_task_assignments_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutTaskAssignmentsIdError {
    Status400(crate::models::ClientError),
    Status404(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}


/// Deletes a specific task assignment.
pub async fn delete_task_assignments_id(configuration: &configuration::Configuration, params: DeleteTaskAssignmentsIdParams) -> Result<(), Error<DeleteTaskAssignmentsIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let task_assignment_id = params.task_assignment_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/task_assignments/{task_assignment_id}", local_var_configuration.base_path, task_assignment_id=crate::apis::urlencode(task_assignment_id));
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
        let local_var_entity: Option<DeleteTaskAssignmentsIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves information about a task assignment.
pub async fn get_task_assignments_id(configuration: &configuration::Configuration, params: GetTaskAssignmentsIdParams) -> Result<crate::models::TaskAssignment, Error<GetTaskAssignmentsIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let task_assignment_id = params.task_assignment_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/task_assignments/{task_assignment_id}", local_var_configuration.base_path, task_assignment_id=crate::apis::urlencode(task_assignment_id));
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
        let local_var_entity: Option<GetTaskAssignmentsIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Lists all of the assignments for a given task.
pub async fn get_tasks_id_assignments(configuration: &configuration::Configuration, params: GetTasksIdAssignmentsParams) -> Result<crate::models::TaskAssignments, Error<GetTasksIdAssignmentsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let task_id = params.task_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tasks/{task_id}/assignments", local_var_configuration.base_path, task_id=crate::apis::urlencode(task_id));
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
        let local_var_entity: Option<GetTasksIdAssignmentsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Assigns a task to a user.  A task can be assigned to more than one user by creating multiple assignments.
pub async fn post_task_assignments(configuration: &configuration::Configuration, params: PostTaskAssignmentsParams) -> Result<crate::models::TaskAssignment, Error<PostTaskAssignmentsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let post_task_assignments_request = params.post_task_assignments_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/task_assignments", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&post_task_assignments_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PostTaskAssignmentsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates a task assignment. This endpoint can be used to update the state of a task assigned to a user.
pub async fn put_task_assignments_id(configuration: &configuration::Configuration, params: PutTaskAssignmentsIdParams) -> Result<crate::models::TaskAssignment, Error<PutTaskAssignmentsIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let task_assignment_id = params.task_assignment_id;
    let put_task_assignments_id_request = params.put_task_assignments_id_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/task_assignments/{task_assignment_id}", local_var_configuration.base_path, task_assignment_id=crate::apis::urlencode(task_assignment_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&put_task_assignments_id_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PutTaskAssignmentsIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
