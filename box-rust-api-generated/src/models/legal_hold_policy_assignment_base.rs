/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// LegalHoldPolicyAssignmentBase : Legal Hold Assignments are used to assign Legal Hold Policies to Users, Folders, Files, or File Versions.  Creating a Legal Hold Assignment puts a hold on the File-Versions that belong to the Assignment's 'apply-to' entity.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LegalHoldPolicyAssignmentBase {
    /// The unique identifier for this legal hold assignment
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// `legal_hold_policy_assignment`
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
}

impl LegalHoldPolicyAssignmentBase {
    /// Legal Hold Assignments are used to assign Legal Hold Policies to Users, Folders, Files, or File Versions.  Creating a Legal Hold Assignment puts a hold on the File-Versions that belong to the Assignment's 'apply-to' entity.
    pub fn new() -> LegalHoldPolicyAssignmentBase {
        LegalHoldPolicyAssignmentBase {
            id: None,
            r#type: None,
        }
    }
}

/// `legal_hold_policy_assignment`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "legal_hold_policy_assignment")]
    LegalHoldPolicyAssignment,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::LegalHoldPolicyAssignment
    }
}

