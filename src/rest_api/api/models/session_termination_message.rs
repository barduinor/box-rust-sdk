/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// SessionTerminationMessage : A message informing about the termination job status

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SessionTerminationMessage {
    /// The unique identifier for the termination job status
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl SessionTerminationMessage {
    /// A message informing about the termination job status
    pub fn new() -> SessionTerminationMessage {
        SessionTerminationMessage { message: None }
    }
}
