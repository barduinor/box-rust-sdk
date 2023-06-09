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
pub struct FileConflictAllOf {
    /// The SHA1 hash of the file.
    #[serde(rename = "sha1", skip_serializing_if = "Option::is_none")]
    pub sha1: Option<String>,
    #[serde(rename = "file_version", skip_serializing_if = "Option::is_none")]
    pub file_version: Option<Box<crate::models::FileVersionMini>>,
}

impl FileConflictAllOf {
    pub fn new() -> FileConflictAllOf {
        FileConflictAllOf {
            sha1: None,
            file_version: None,
        }
    }
}

