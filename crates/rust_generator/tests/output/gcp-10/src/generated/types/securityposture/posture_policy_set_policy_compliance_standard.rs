#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PosturePolicySetPolicyComplianceStandard {
    /// Mapping of security controls for the policy.
    #[builder(into, default)]
    #[serde(rename = "control")]
    pub r#control: Box<Option<String>>,
    /// Mapping of compliance standards for the policy.
    #[builder(into, default)]
    #[serde(rename = "standard")]
    pub r#standard: Box<Option<String>>,
}
