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
pub struct FolderMiniAllOf {
    /// The name of the folder.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A numeric identifier that represents the most recent user event that has been applied to this item.  This can be used in combination with the `GET /events`-endpoint to filter out user events that would have occurred before this identifier was read.  An example would be where a Box Drive-like application would fetch an item via the API, and then listen to incoming user events for changes to the item. The application would ignore any user events where the `sequence_id` in the event is smaller than or equal to the `sequence_id` in the originally fetched resource.
    #[serde(rename = "sequence_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sequence_id: Option<Option<String>>,
}

impl FolderMiniAllOf {
    pub fn new() -> FolderMiniAllOf {
        FolderMiniAllOf {
            name: None,
            sequence_id: None,
        }
    }
}

