/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// UserAvatarPicUrls : Represents an object with user avatar URLs.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserAvatarPicUrls {
    /// The location of a small-sized avatar.
    #[serde(rename = "small", skip_serializing_if = "Option::is_none")]
    pub small: Option<String>,
    /// The location of a large-sized avatar.
    #[serde(rename = "large", skip_serializing_if = "Option::is_none")]
    pub large: Option<String>,
    /// The location of the avatar preview.
    #[serde(rename = "preview", skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,
}

impl UserAvatarPicUrls {
    /// Represents an object with user avatar URLs.
    pub fn new() -> UserAvatarPicUrls {
        UserAvatarPicUrls {
            small: None,
            large: None,
            preview: None,
        }
    }
}

