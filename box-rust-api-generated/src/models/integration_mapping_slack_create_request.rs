/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// IntegrationMappingSlackCreateRequest : A request to create a Slack Integration Mapping object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IntegrationMappingSlackCreateRequest {
    #[serde(rename = "partner_item")]
    pub partner_item: Box<crate::models::IntegrationMappingSlackCreateRequestPartnerItem>,
    #[serde(rename = "box_item")]
    pub box_item: Box<crate::models::PutIntegrationMappingsSlackIdRequestBoxItem>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Box<crate::models::PutIntegrationMappingsSlackIdRequestOptions>>,
}

impl IntegrationMappingSlackCreateRequest {
    /// A request to create a Slack Integration Mapping object
    pub fn new(partner_item: crate::models::IntegrationMappingSlackCreateRequestPartnerItem, box_item: crate::models::PutIntegrationMappingsSlackIdRequestBoxItem) -> IntegrationMappingSlackCreateRequest {
        IntegrationMappingSlackCreateRequest {
            partner_item: Box::new(partner_item),
            box_item: Box::new(box_item),
            options: None,
        }
    }
}


