/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// ShieldInformationBarrierReport : A standard representation of a shield information barrier report object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShieldInformationBarrierReport {
    /// The unique identifier for the shield information barrier report
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The type of the shield information barrier report
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
    #[serde(rename = "shield_information_barrier", skip_serializing_if = "Option::is_none")]
    pub shield_information_barrier: Option<Box<crate::models::ShieldInformationBarrierReportAllOfShieldInformationBarrier>>,
    /// Status of the shield information report
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<Box<crate::models::ShieldInformationBarrierReportAllOfDetails>>,
    /// ISO date time string when this shield information barrier report object was created.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Box<crate::models::ShieldInformationBarrierReportAllOfCreatedBy>>,
    /// ISO date time string when this shield information barrier report was updated.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl ShieldInformationBarrierReport {
    /// A standard representation of a shield information barrier report object
    pub fn new() -> ShieldInformationBarrierReport {
        ShieldInformationBarrierReport {
            id: None,
            r#type: None,
            shield_information_barrier: None,
            status: None,
            details: None,
            created_at: None,
            created_by: None,
            updated_at: None,
        }
    }
}

/// The type of the shield information barrier report
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "shield_information_barrier_report")]
    ShieldInformationBarrierReport,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::ShieldInformationBarrierReport
    }
}
/// Status of the shield information report
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "done")]
    Done,
    #[serde(rename = "cancelled")]
    Cancelled,
}

impl Default for Status {
    fn default() -> Status {
        Self::Pending
    }
}

