/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// PutFilesIdRequestParentAllOf : The parent for this item



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PutFilesIdRequestParentAllOf {
    /// The ID of parent item
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl PutFilesIdRequestParentAllOf {
    /// The parent for this item
    pub fn new() -> PutFilesIdRequestParentAllOf {
        PutFilesIdRequestParentAllOf {
            id: None,
        }
    }
}

