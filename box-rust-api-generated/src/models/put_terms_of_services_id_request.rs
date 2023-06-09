/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PutTermsOfServicesIdRequest {
    /// Whether this terms of service is active.
    #[serde(rename = "status")]
    pub status: Status,
    /// The terms of service text to display to users.  The text can be set to empty if the `status` is set to `disabled`.
    #[serde(rename = "text")]
    pub text: String,
}

impl PutTermsOfServicesIdRequest {
    pub fn new(status: Status, text: String) -> PutTermsOfServicesIdRequest {
        PutTermsOfServicesIdRequest {
            status,
            text,
        }
    }
}

/// Whether this terms of service is active.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

impl Default for Status {
    fn default() -> Status {
        Self::Enabled
    }
}

