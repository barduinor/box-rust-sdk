/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// Classification : An instance of the classification metadata template, containing the classification applied to the file or folder.  To get more details about the classification applied to an item, request the classification metadata template.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Classification {
    /// The name of the classification applied to the item.
    #[serde(
        rename = "Box_Security_Classification_Key",
        skip_serializing_if = "Option::is_none"
    )]
    pub box_security_classification_key: Option<String>,
    /// The identifier of the item that this metadata instance has been attached to. This combines the `type` and the `id` of the parent in the form `{type}_{id}`.
    #[serde(rename = "$parent", skip_serializing_if = "Option::is_none")]
    pub dollar_parent: Option<String>,
    /// `securityClassification-6VMVochwUWo`
    #[serde(rename = "$template", skip_serializing_if = "Option::is_none")]
    pub dollar_template: Option<DollarTemplate>,
    /// The scope of the enterprise that this classification has been applied for.  This will be in the format `enterprise_{enterprise_id}`.
    #[serde(rename = "$scope", skip_serializing_if = "Option::is_none")]
    pub dollar_scope: Option<String>,
    /// The version of the metadata instance. This version starts at 0 and increases every time a classification is updated.
    #[serde(rename = "$version", skip_serializing_if = "Option::is_none")]
    pub dollar_version: Option<i32>,
    /// The unique ID of this classification instance. This will be include the name of the classification template and a unique ID.
    #[serde(rename = "$type", skip_serializing_if = "Option::is_none")]
    pub dollar_type: Option<String>,
    /// The version of the metadata template. This version starts at 0 and increases every time the template is updated. This is mostly for internal use.
    #[serde(rename = "$typeVersion", skip_serializing_if = "Option::is_none")]
    pub dollar_type_version: Option<f32>,
    /// Whether an end user can change the classification.
    #[serde(rename = "$canEdit", skip_serializing_if = "Option::is_none")]
    pub dollar_can_edit: Option<bool>,
}

impl Classification {
    /// An instance of the classification metadata template, containing the classification applied to the file or folder.  To get more details about the classification applied to an item, request the classification metadata template.
    pub fn new() -> Classification {
        Classification {
            box_security_classification_key: None,
            dollar_parent: None,
            dollar_template: None,
            dollar_scope: None,
            dollar_version: None,
            dollar_type: None,
            dollar_type_version: None,
            dollar_can_edit: None,
        }
    }
}

/// `securityClassification-6VMVochwUWo`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DollarTemplate {
    #[serde(rename = "securityClassification-6VMVochwUWo")]
    SecurityClassification6VmVochwUwo,
}

impl Default for DollarTemplate {
    fn default() -> DollarTemplate {
        Self::SecurityClassification6VmVochwUwo
    }
}