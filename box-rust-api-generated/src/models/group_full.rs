/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// GroupFull : Groups contain a set of users, and can be used in place of users in some operations, such as collaborations.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GroupFull {
    /// The unique identifier for this object
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// `group`
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
    /// The name of the group
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The type of the group.
    #[serde(rename = "group_type", skip_serializing_if = "Option::is_none")]
    pub group_type: Option<GroupType>,
    /// When the group object was created
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the group object was last modified
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    /// Keeps track of which external source this group is coming from (e.g. \"Active Directory\", \"Google Groups\", \"Facebook Groups\").  Setting this will also prevent Box users from editing the group name and its members directly via the Box web application. This is desirable for one-way syncing of groups.
    #[serde(rename = "provenance", skip_serializing_if = "Option::is_none")]
    pub provenance: Option<String>,
    /// An arbitrary identifier that can be used by external group sync tools to link this Box Group to an external group. Example values of this field could be an Active Directory Object ID or a Google Group ID.  We recommend you use of this field in order to avoid issues when group names are updated in either Box or external systems.
    #[serde(rename = "external_sync_identifier", skip_serializing_if = "Option::is_none")]
    pub external_sync_identifier: Option<String>,
    /// Human readable description of the group.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Specifies who can invite the group to collaborate on items.  When set to `admins_only` the enterprise admin, co-admins, and the group's admin can invite the group.  When set to `admins_and_members` all the admins listed above and group members can invite the group.  When set to `all_managed_users` all managed users in the enterprise can invite the group.
    #[serde(rename = "invitability_level", skip_serializing_if = "Option::is_none")]
    pub invitability_level: Option<InvitabilityLevel>,
    /// Specifies who can view the members of the group (Get Memberships for Group).  * `admins_only` - the enterprise admin, co-admins, group's   group admin * `admins_and_members` - all admins and group members * `all_managed_users` - all managed users in the   enterprise
    #[serde(rename = "member_viewability_level", skip_serializing_if = "Option::is_none")]
    pub member_viewability_level: Option<MemberViewabilityLevel>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Box<crate::models::GroupFullAllOfPermissions>>,
}

impl GroupFull {
    /// Groups contain a set of users, and can be used in place of users in some operations, such as collaborations.
    pub fn new() -> GroupFull {
        GroupFull {
            id: None,
            r#type: None,
            name: None,
            group_type: None,
            created_at: None,
            modified_at: None,
            provenance: None,
            external_sync_identifier: None,
            description: None,
            invitability_level: None,
            member_viewability_level: None,
            permissions: None,
        }
    }
}

/// `group`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "group")]
    Group,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Group
    }
}
/// The type of the group.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GroupType {
    #[serde(rename = "managed_group")]
    ManagedGroup,
    #[serde(rename = "all_users_group")]
    AllUsersGroup,
}

impl Default for GroupType {
    fn default() -> GroupType {
        Self::ManagedGroup
    }
}
/// Specifies who can invite the group to collaborate on items.  When set to `admins_only` the enterprise admin, co-admins, and the group's admin can invite the group.  When set to `admins_and_members` all the admins listed above and group members can invite the group.  When set to `all_managed_users` all managed users in the enterprise can invite the group.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InvitabilityLevel {
    #[serde(rename = "admins_only")]
    AdminsOnly,
    #[serde(rename = "admins_and_members")]
    AdminsAndMembers,
    #[serde(rename = "all_managed_users")]
    AllManagedUsers,
}

impl Default for InvitabilityLevel {
    fn default() -> InvitabilityLevel {
        Self::AdminsOnly
    }
}
/// Specifies who can view the members of the group (Get Memberships for Group).  * `admins_only` - the enterprise admin, co-admins, group's   group admin * `admins_and_members` - all admins and group members * `all_managed_users` - all managed users in the   enterprise
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MemberViewabilityLevel {
    #[serde(rename = "admins_only")]
    AdminsOnly,
    #[serde(rename = "admins_and_members")]
    AdminsAndMembers,
    #[serde(rename = "all_managed_users")]
    AllManagedUsers,
}

impl Default for MemberViewabilityLevel {
    fn default() -> MemberViewabilityLevel {
        Self::AdminsOnly
    }
}

