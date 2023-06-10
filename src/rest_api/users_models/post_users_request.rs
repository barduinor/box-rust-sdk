/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

use crate::rest_api::api_base_models::tracking_code::TrackingCode;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PostUsersRequest {
    /// The name of the user
    #[serde(rename = "name")]
    pub name: String,
    /// The email address the user uses to log in  Required, unless `is_platform_access_only` is set to `true`.
    #[serde(rename = "login", skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    /// Specifies that the user is an app user.
    #[serde(
        rename = "is_platform_access_only",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_platform_access_only: Option<bool>,
    /// The user’s enterprise role
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    /// The language of the user, formatted in modified version of the [ISO 639-1](/guides/api-calls/language-codes) format.
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Whether the user can use Box Sync
    #[serde(rename = "is_sync_enabled", skip_serializing_if = "Option::is_none")]
    pub is_sync_enabled: Option<bool>,
    /// The user’s job title
    #[serde(rename = "job_title", skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    /// The user’s phone number
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// The user’s address
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The user’s total available space in bytes. Set this to `-1` to indicate unlimited storage.
    #[serde(rename = "space_amount", skip_serializing_if = "Option::is_none")]
    pub space_amount: Option<i64>,
    /// Tracking codes allow an admin to generate reports from the admin console and assign an attribute to a specific group of users. This setting must be enabled for an enterprise before it can be used.
    #[serde(rename = "tracking_codes", skip_serializing_if = "Option::is_none")]
    pub tracking_codes: Option<Vec<TrackingCode>>,
    /// Whether the user can see other enterprise users in their contact list
    #[serde(
        rename = "can_see_managed_users",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_see_managed_users: Option<bool>,
    /// The user's timezone
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Whether the user is allowed to collaborate with users outside their enterprise
    #[serde(
        rename = "is_external_collab_restricted",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_external_collab_restricted: Option<bool>,
    /// Whether to exempt the user from enterprise device limits
    #[serde(
        rename = "is_exempt_from_device_limits",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_exempt_from_device_limits: Option<bool>,
    /// Whether the user must use two-factor authentication
    #[serde(
        rename = "is_exempt_from_login_verification",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_exempt_from_login_verification: Option<bool>,
    /// The user's account status
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// An external identifier for an app user, which can be used to look up the user. This can be used to tie user IDs from external identity providers to Box users.
    #[serde(
        rename = "external_app_user_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_app_user_id: Option<String>,
}

impl PostUsersRequest {
    pub fn new(name: String) -> PostUsersRequest {
        PostUsersRequest {
            name,
            login: None,
            is_platform_access_only: None,
            role: None,
            language: None,
            is_sync_enabled: None,
            job_title: None,
            phone: None,
            address: None,
            space_amount: None,
            tracking_codes: None,
            can_see_managed_users: None,
            timezone: None,
            is_external_collab_restricted: None,
            is_exempt_from_device_limits: None,
            is_exempt_from_login_verification: None,
            status: None,
            external_app_user_id: None,
        }
    }
}

/// The user’s enterprise role
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "coadmin")]
    Coadmin,
    #[serde(rename = "user")]
    User,
}

impl Default for Role {
    fn default() -> Role {
        Self::Coadmin
    }
}
/// The user's account status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "cannot_delete_edit")]
    CannotDeleteEdit,
    #[serde(rename = "cannot_delete_edit_upload")]
    CannotDeleteEditUpload,
}

impl Default for Status {
    fn default() -> Status {
        Self::Active
    }
}
