/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// MetadataQueryOrderByInner : An object representing one of the metadata template fields to sort the metadata query results by.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MetadataQueryOrderByInner {
    /// The metadata template field to order by.  The `field_key` represents the `key` value of a field from the metadata template being searched for.
    #[serde(rename = "field_key", skip_serializing_if = "Option::is_none")]
    pub field_key: Option<String>,
    /// The direction to order by, either ascending or descending.  The `ordering` direction must be the same for each item in the array.
    #[serde(rename = "direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
}

impl MetadataQueryOrderByInner {
    /// An object representing one of the metadata template fields to sort the metadata query results by.
    pub fn new() -> MetadataQueryOrderByInner {
        MetadataQueryOrderByInner {
            field_key: None,
            direction: None,
        }
    }
}

/// The direction to order by, either ascending or descending.  The `ordering` direction must be the same for each item in the array.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "ASC")]
    Asc,
    #[serde(rename = "DESC")]
    Desc,
    //
    // TODO: could we use a serde alias with multiple names?
    // #[serde(alias = "ASC", alias = "Asc")]
    //
    // #[serde(rename = "asc")]
    // Asc,
    // #[serde(rename = "desc")]
    // Desc,
}

impl Default for Direction {
    fn default() -> Direction {
        Self::Asc
    }
}
