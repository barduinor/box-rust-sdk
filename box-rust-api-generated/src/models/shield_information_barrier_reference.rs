/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// ShieldInformationBarrierReference : A shield information barrier reference for requests and responses



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShieldInformationBarrierReference {
    #[serde(rename = "shield_information_barrier", skip_serializing_if = "Option::is_none")]
    pub shield_information_barrier: Option<Box<crate::models::ShieldInformationBarrierBase>>,
}

impl ShieldInformationBarrierReference {
    /// A shield information barrier reference for requests and responses
    pub fn new() -> ShieldInformationBarrierReference {
        ShieldInformationBarrierReference {
            shield_information_barrier: None,
        }
    }
}


