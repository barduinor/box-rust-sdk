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
pub struct PostFoldersRequest {
    /// The name for the new folder.  There are some restrictions to the file name. Names containing non-printable ASCII characters, forward and backward slashes (`/`, `\\`), as well as names with trailing spaces are prohibited.  Additionally, the names `.` and `..` are not allowed either.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "parent")]
    pub parent: Box<crate::models::PostFoldersRequestParent>,
    #[serde(rename = "folder_upload_email", skip_serializing_if = "Option::is_none")]
    pub folder_upload_email: Option<Box<crate::models::PostFoldersRequestFolderUploadEmail>>,
    /// Specifies whether a folder should be synced to a user's device or not. This is used by Box Sync (discontinued) and is not used by Box Drive.
    #[serde(rename = "sync_state", skip_serializing_if = "Option::is_none")]
    pub sync_state: Option<SyncState>,
}

impl PostFoldersRequest {
    pub fn new(name: String, parent: crate::models::PostFoldersRequestParent) -> PostFoldersRequest {
        PostFoldersRequest {
            name,
            parent: Box::new(parent),
            folder_upload_email: None,
            sync_state: None,
        }
    }
}

/// Specifies whether a folder should be synced to a user's device or not. This is used by Box Sync (discontinued) and is not used by Box Drive.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyncState {
    #[serde(rename = "synced")]
    Synced,
    #[serde(rename = "not_synced")]
    NotSynced,
    #[serde(rename = "partially_synced")]
    PartiallySynced,
}

impl Default for SyncState {
    fn default() -> SyncState {
        Self::Synced
    }
}

