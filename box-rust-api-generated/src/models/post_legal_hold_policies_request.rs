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
pub struct PostLegalHoldPoliciesRequest {
    /// The name of the policy.
    #[serde(rename = "policy_name")]
    pub policy_name: String,
    /// A description for the policy.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The filter start date.  When this policy is applied using a `custodian` legal hold assignments, it will only apply to file versions created or uploaded inside of the date range. Other assignment types, such as folders and files, will ignore the date filter.  Required if `is_ongoing` is set to `false`.
    #[serde(rename = "filter_started_at", skip_serializing_if = "Option::is_none")]
    pub filter_started_at: Option<String>,
    /// The filter end date.  When this policy is applied using a `custodian` legal hold assignments, it will only apply to file versions created or uploaded inside of the date range. Other assignment types, such as folders and files, will ignore the date filter.  Required if `is_ongoing` is set to `false`.
    #[serde(rename = "filter_ended_at", skip_serializing_if = "Option::is_none")]
    pub filter_ended_at: Option<String>,
    /// Whether new assignments under this policy should continue applying to files even after initialization.  When this policy is applied using a legal hold assignment, it will continue applying the policy to any new file versions even after it has been applied.  For example, if a legal hold assignment is placed on a user today, and that user uploads a file tomorrow, that file will get held. This will continue until the policy is retired.  Required if no filter dates are set.
    #[serde(rename = "is_ongoing", skip_serializing_if = "Option::is_none")]
    pub is_ongoing: Option<bool>,
}

impl PostLegalHoldPoliciesRequest {
    pub fn new(policy_name: String) -> PostLegalHoldPoliciesRequest {
        PostLegalHoldPoliciesRequest {
            policy_name,
            description: None,
            filter_started_at: None,
            filter_ended_at: None,
            is_ongoing: None,
        }
    }
}


