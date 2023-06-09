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
pub struct LegalHoldPolicyAllOf {
    /// Name of the legal hold policy.
    #[serde(rename = "policy_name", skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// Description of the legal hold policy. Optional property with a 500 character limit.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// * 'active' - the policy is not in a transition state * 'applying' - that the policy is in the process of   being applied * 'releasing' - that the process is in the process   of being released * 'released' - the policy is no longer active
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "assignment_counts", skip_serializing_if = "Option::is_none")]
    pub assignment_counts: Option<Box<crate::models::LegalHoldPolicyAllOfAssignmentCounts>>,
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Box<crate::models::LegalHoldPolicyAllOfCreatedBy>>,
    /// When the legal hold policy object was created
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the legal hold policy object was modified. Does not update when assignments are added or removed.
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    /// When the policy release request was sent. (Because it can take time for a policy to fully delete, this isn't quite the same time that the policy is fully deleted).  If `null`, the policy was not deleted.
    #[serde(rename = "deleted_at", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    /// User-specified, optional date filter applies to Custodian assignments only
    #[serde(rename = "filter_started_at", skip_serializing_if = "Option::is_none")]
    pub filter_started_at: Option<String>,
    /// User-specified, optional date filter applies to Custodian assignments only
    #[serde(rename = "filter_ended_at", skip_serializing_if = "Option::is_none")]
    pub filter_ended_at: Option<String>,
    /// Optional notes about why the policy was created.
    #[serde(rename = "release_notes", skip_serializing_if = "Option::is_none")]
    pub release_notes: Option<String>,
}

impl LegalHoldPolicyAllOf {
    pub fn new() -> LegalHoldPolicyAllOf {
        LegalHoldPolicyAllOf {
            policy_name: None,
            description: None,
            status: None,
            assignment_counts: None,
            created_by: None,
            created_at: None,
            modified_at: None,
            deleted_at: None,
            filter_started_at: None,
            filter_ended_at: None,
            release_notes: None,
        }
    }
}

/// * 'active' - the policy is not in a transition state * 'applying' - that the policy is in the process of   being applied * 'releasing' - that the process is in the process   of being released * 'released' - the policy is no longer active
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "applying")]
    Applying,
    #[serde(rename = "releasing")]
    Releasing,
    #[serde(rename = "released")]
    Released,
}

impl Default for Status {
    fn default() -> Status {
        Self::Active
    }
}
