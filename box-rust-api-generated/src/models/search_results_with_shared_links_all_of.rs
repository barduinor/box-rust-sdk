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
pub struct SearchResultsWithSharedLinksAllOf {
    /// The search results for the query provided, including the additional information about any shared links through which the item has been shared with the user.
    #[serde(rename = "entries", skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<crate::models::SearchResultWithSharedLink>>,
}

impl SearchResultsWithSharedLinksAllOf {
    pub fn new() -> SearchResultsWithSharedLinksAllOf {
        SearchResultsWithSharedLinksAllOf {
            entries: None,
        }
    }
}


