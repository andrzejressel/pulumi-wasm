#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PosturePolicySetPolicyConstraintOrgPolicyConstraintPolicyRule {
    /// Setting this to true means that all values are allowed. This field can be set only in policies for list constraints.
    #[builder(into, default)]
    #[serde(rename = "allowAll")]
    pub r#allow_all: Box<Option<bool>>,
    /// Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language.
    /// This page details the objects and attributes that are used to the build the CEL expressions for
    /// custom access levels - https://cloud.google.com/access-context-manager/docs/custom-access-level-spec.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "condition")]
    pub r#condition: Box<Option<super::super::types::securityposture::PosturePolicySetPolicyConstraintOrgPolicyConstraintPolicyRuleCondition>>,
    /// Setting this to true means that all values are denied. This field can be set only in policies for list constraints.
    #[builder(into, default)]
    #[serde(rename = "denyAll")]
    pub r#deny_all: Box<Option<bool>>,
    /// If `true`, then the policy is enforced. If `false`, then any configuration is acceptable.
    /// This field can be set only in policies for boolean constraints.
    #[builder(into, default)]
    #[serde(rename = "enforce")]
    pub r#enforce: Box<Option<bool>>,
    /// List of values to be used for this policy rule. This field can be set only in policies for list constraints.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "values")]
    pub r#values: Box<Option<super::super::types::securityposture::PosturePolicySetPolicyConstraintOrgPolicyConstraintPolicyRuleValues>>,
}
