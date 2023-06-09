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
pub struct ShieldInformationBarrierSegmentMemberAllOf {
    #[serde(rename = "shield_information_barrier", skip_serializing_if = "Option::is_none")]
    pub shield_information_barrier: Option<Box<crate::models::ShieldInformationBarrierBase>>,
    #[serde(rename = "shield_information_barrier_segment", skip_serializing_if = "Option::is_none")]
    pub shield_information_barrier_segment: Option<Box<crate::models::ShieldInformationBarrierSegmentMemberAllOfShieldInformationBarrierSegment>>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::ShieldInformationBarrierSegmentMemberAllOfUser>>,
    /// ISO date time string when this shield information barrier object was created.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Box<crate::models::ShieldInformationBarrierSegmentMemberAllOfCreatedBy>>,
    /// ISO date time string when this shield information barrier segment Member was updated.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "updated_by", skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<Box<crate::models::ShieldInformationBarrierSegmentMemberAllOfUpdatedBy>>,
}

impl ShieldInformationBarrierSegmentMemberAllOf {
    pub fn new() -> ShieldInformationBarrierSegmentMemberAllOf {
        ShieldInformationBarrierSegmentMemberAllOf {
            shield_information_barrier: None,
            shield_information_barrier_segment: None,
            user: None,
            created_at: None,
            created_by: None,
            updated_at: None,
            updated_by: None,
        }
    }
}


