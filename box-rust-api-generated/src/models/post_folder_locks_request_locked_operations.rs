/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// PostFolderLocksRequestLockedOperations : The operations to lock for the folder. If `locked_operations` is included in the request, both `move` and `delete` must also be included and both set to `true`.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PostFolderLocksRequestLockedOperations {
    /// Whether moving the folder should be locked.
    #[serde(rename = "move")]
    pub r#move: bool,
    /// Whether deleting the folder should be locked.
    #[serde(rename = "delete")]
    pub delete: bool,
}

impl PostFolderLocksRequestLockedOperations {
    /// The operations to lock for the folder. If `locked_operations` is included in the request, both `move` and `delete` must also be included and both set to `true`.
    pub fn new(r#move: bool, delete: bool) -> PostFolderLocksRequestLockedOperations {
        PostFolderLocksRequestLockedOperations {
            r#move,
            delete,
        }
    }
}


