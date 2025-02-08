#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PosturePolicySetPolicy {
    /// Mapping for policy to security standards and controls.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "complianceStandards")]
    pub r#compliance_standards: Box<Option<Vec<super::super::types::securityposture::PosturePolicySetPolicyComplianceStandard>>>,
    /// Policy constraint definition.It can have the definition of one of following constraints: orgPolicyConstraint orgPolicyConstraintCustom securityHealthAnalyticsModule securityHealthAnalyticsCustomModule
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "constraint")]
    pub r#constraint: Box<super::super::types::securityposture::PosturePolicySetPolicyConstraint>,
    /// Description of the policy.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// ID of the policy.
    #[builder(into)]
    #[serde(rename = "policyId")]
    pub r#policy_id: Box<String>,
}
