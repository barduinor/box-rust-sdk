/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// PutFoldersIdMetadataEnterpriseSecurityClassification6VmVochwUwoRequestInner : The operation to perform on the classification metadata template instance. In this case, it use used to replace the value stored for the `Box_Security_Classification_Key` field with a new value.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PutFoldersIdMetadataEnterpriseSecurityClassification6VmVochwUwoRequestInner {
    /// `replace`
    #[serde(rename = "op", skip_serializing_if = "Option::is_none")]
    pub op: Option<Op>,
    /// `/Box_Security_Classification_Key`
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<Path>,
    /// The name of the classification to apply to this folder.  To list the available classifications in an enterprise, use the classification API to retrieve the [classification template](e://get_metadata_templates_enterprise_securityClassification-6VMVochwUWo_schema) which lists all available classification keys.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl PutFoldersIdMetadataEnterpriseSecurityClassification6VmVochwUwoRequestInner {
    /// The operation to perform on the classification metadata template instance. In this case, it use used to replace the value stored for the `Box_Security_Classification_Key` field with a new value.
    pub fn new() -> PutFoldersIdMetadataEnterpriseSecurityClassification6VmVochwUwoRequestInner {
        PutFoldersIdMetadataEnterpriseSecurityClassification6VmVochwUwoRequestInner {
            op: None,
            path: None,
            value: None,
        }
    }
}

/// `replace`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Op {
    #[serde(rename = "replace")]
    Replace,
}

impl Default for Op {
    fn default() -> Op {
        Self::Replace
    }
}
/// `/Box_Security_Classification_Key`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Path {
    #[serde(rename = "/Box_Security_Classification_Key")]
    SlashBoxSecurityClassificationKey,
}

impl Default for Path {
    fn default() -> Path {
        Self::SlashBoxSecurityClassificationKey
    }
}
