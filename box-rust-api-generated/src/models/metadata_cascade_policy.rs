/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// MetadataCascadePolicy : A metadata cascade policy automatically applies a metadata template instance to all the files and folders within the targeted folder.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MetadataCascadePolicy {
    /// The ID of the metadata cascade policy object
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// `metadata_cascade_policy`
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
    #[serde(rename = "owner_enterprise", skip_serializing_if = "Option::is_none")]
    pub owner_enterprise: Option<Box<crate::models::MetadataCascadePolicyOwnerEnterprise>>,
    #[serde(rename = "parent", skip_serializing_if = "Option::is_none")]
    pub parent: Option<Box<crate::models::MetadataCascadePolicyParent>>,
    /// The scope of the of the template that is cascaded down to the folder's children.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
    /// The key of the template that is cascaded down to the folder's children.  In many cases the template key is automatically derived of its display name, for example `Contract Template` would become `contractTemplate`. In some cases the creator of the template will have provided its own template key.  Please [list the templates for an enterprise][list], or get all instances on a [file][file] or [folder][folder] to inspect a template's key.  [list]: e://get-metadata-templates-enterprise [file]: e://get-files-id-metadata [folder]: e://get-folders-id-metadata
    #[serde(rename = "templateKey", skip_serializing_if = "Option::is_none")]
    pub template_key: Option<String>,
}

impl MetadataCascadePolicy {
    /// A metadata cascade policy automatically applies a metadata template instance to all the files and folders within the targeted folder.
    pub fn new() -> MetadataCascadePolicy {
        MetadataCascadePolicy {
            id: None,
            r#type: None,
            owner_enterprise: None,
            parent: None,
            scope: None,
            template_key: None,
        }
    }
}

/// `metadata_cascade_policy`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "metadata_cascade_policy")]
    MetadataCascadePolicy,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::MetadataCascadePolicy
    }
}
/// The scope of the of the template that is cascaded down to the folder's children.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Scope {
    #[serde(rename = "global")]
    Global,
    #[serde(rename = "enterprise_*")]
    EnterpriseStar,
}

impl Default for Scope {
    fn default() -> Scope {
        Self::Global
    }
}

