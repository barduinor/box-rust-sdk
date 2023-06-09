/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// FileVersionBase : The bare basic representation of a file version, the minimal amount of fields returned when using the `fields` query parameter.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FileVersionBase {
    /// The unique identifier that represent a file version.
    #[serde(rename = "id")]
    pub id: String,
    /// `file_version`
    #[serde(rename = "type")]
    pub r#type: RHashType,
}

impl FileVersionBase {
    /// The bare basic representation of a file version, the minimal amount of fields returned when using the `fields` query parameter.
    pub fn new(id: String, r#type: RHashType) -> FileVersionBase {
        FileVersionBase {
            id,
            r#type,
        }
    }
}

/// `file_version`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "file_version")]
    FileVersion,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::FileVersion
    }
}

