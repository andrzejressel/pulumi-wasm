#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WebAclRuleStatementRuleGroupReferenceStatement {
    /// The Amazon Resource Name (ARN) of the `aws.wafv2.RuleGroup` resource.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Box<String>,
    /// Action settings to use in the place of the rule actions that are configured inside the rule group. You specify one override for each rule whose action you want to change. See `rule_action_override` below for details.
    #[builder(into, default)]
    #[serde(rename = "ruleActionOverrides")]
    pub r#rule_action_overrides: Box<Option<Vec<super::super::types::wafv2::WebAclRuleStatementRuleGroupReferenceStatementRuleActionOverride>>>,
}
