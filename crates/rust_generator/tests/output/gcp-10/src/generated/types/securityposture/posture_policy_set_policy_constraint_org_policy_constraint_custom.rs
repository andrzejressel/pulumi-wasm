#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PosturePolicySetPolicyConstraintOrgPolicyConstraintCustom {
    /// Organization policy custom constraint definition.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "customConstraint")]
    pub r#custom_constraint: Box<Option<super::super::types::securityposture::PosturePolicySetPolicyConstraintOrgPolicyConstraintCustomCustomConstraint>>,
    /// Definition of policy rules
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "policyRules")]
    pub r#policy_rules: Box<Vec<super::super::types::securityposture::PosturePolicySetPolicyConstraintOrgPolicyConstraintCustomPolicyRule>>,
}
