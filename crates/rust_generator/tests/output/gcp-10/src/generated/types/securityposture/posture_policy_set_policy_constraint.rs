#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PosturePolicySetPolicyConstraint {
    /// Organization policy canned constraint definition.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "orgPolicyConstraint")]
    pub r#org_policy_constraint: Box<Option<super::super::types::securityposture::PosturePolicySetPolicyConstraintOrgPolicyConstraint>>,
    /// Organization policy custom constraint policy definition.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "orgPolicyConstraintCustom")]
    pub r#org_policy_constraint_custom: Box<Option<super::super::types::securityposture::PosturePolicySetPolicyConstraintOrgPolicyConstraintCustom>>,
    /// Definition of Security Health Analytics Custom Module.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "securityHealthAnalyticsCustomModule")]
    pub r#security_health_analytics_custom_module: Box<Option<super::super::types::securityposture::PosturePolicySetPolicyConstraintSecurityHealthAnalyticsCustomModule>>,
    /// Security Health Analytics built-in detector definition.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "securityHealthAnalyticsModule")]
    pub r#security_health_analytics_module: Box<Option<super::super::types::securityposture::PosturePolicySetPolicyConstraintSecurityHealthAnalyticsModule>>,
}
