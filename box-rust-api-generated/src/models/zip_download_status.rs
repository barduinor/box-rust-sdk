/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// ZipDownloadStatus : The status of a `zip` archive being downloaded.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ZipDownloadStatus {
    /// The total number of files in the archive.
    #[serde(rename = "total_file_count", skip_serializing_if = "Option::is_none")]
    pub total_file_count: Option<i32>,
    /// The number of files that have already been downloaded.
    #[serde(rename = "downloaded_file_count", skip_serializing_if = "Option::is_none")]
    pub downloaded_file_count: Option<i32>,
    /// The number of files that have been skipped as they could not be downloaded. In many cases this is due to permission issues that have surfaced between the creation of the request for the archive and the archive being downloaded.
    #[serde(rename = "skipped_file_count", skip_serializing_if = "Option::is_none")]
    pub skipped_file_count: Option<i32>,
    /// The number of folders that have been skipped as they could not be downloaded. In many cases this is due to permission issues that have surfaced between the creation of the request for the archive and the archive being downloaded.
    #[serde(rename = "skipped_folder_count", skip_serializing_if = "Option::is_none")]
    pub skipped_folder_count: Option<i32>,
    /// The state of the archive being downloaded.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
}

impl ZipDownloadStatus {
    /// The status of a `zip` archive being downloaded.
    pub fn new() -> ZipDownloadStatus {
        ZipDownloadStatus {
            total_file_count: None,
            downloaded_file_count: None,
            skipped_file_count: None,
            skipped_folder_count: None,
            state: None,
        }
    }
}

/// The state of the archive being downloaded.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "success")]
    Success,
}

impl Default for State {
    fn default() -> State {
        Self::InProgress
    }
}
