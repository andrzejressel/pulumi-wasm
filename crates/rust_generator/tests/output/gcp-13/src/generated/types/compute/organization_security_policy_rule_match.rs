#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct OrganizationSecurityPolicyRuleMatch {
    /// The configuration options for matching the rule.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "config")]
    pub r#config: Box<super::super::types::compute::OrganizationSecurityPolicyRuleMatchConfig>,
    /// A description of the rule.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Preconfigured versioned expression. For organization security policy rules,
    /// the only supported type is "FIREWALL".
    /// Default value is `FIREWALL`.
    /// Possible values are: `FIREWALL`.
    #[builder(into, default)]
    #[serde(rename = "versionedExpr")]
    pub r#versioned_expr: Box<Option<String>>,
}
