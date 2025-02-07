#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRoleManagementPolicyEligibleAssignmentRule {
    /// (Boolean) Must an assignment have an expiry date.
    #[builder(into)]
    #[serde(rename = "expirationRequired")]
    pub r#expiration_required: Box<bool>,
    /// (String) The maximum length of time an assignment can be valid, as an ISO8601 duration.
    #[builder(into)]
    #[serde(rename = "expireAfter")]
    pub r#expire_after: Box<String>,
}
