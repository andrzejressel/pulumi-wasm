#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetBackendServiceLocalityLbPolicy {
    /// The configuration for a custom policy implemented by the user and
    /// deployed with the client.
    #[builder(into)]
    #[serde(rename = "customPolicies")]
    pub r#custom_policies: Box<Vec<super::super::types::compute::GetBackendServiceLocalityLbPolicyCustomPolicy>>,
    /// The configuration for a built-in load balancing policy.
    #[builder(into)]
    #[serde(rename = "policies")]
    pub r#policies: Box<Vec<super::super::types::compute::GetBackendServiceLocalityLbPolicyPolicy>>,
}
