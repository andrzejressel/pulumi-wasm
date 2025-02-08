#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WebAclRuleStatementManagedRuleGroupStatementRuleActionOverride {
    /// Override action to use, in place of the configured action of the rule in the rule group. See `action` for details.
    #[builder(into)]
    #[serde(rename = "actionToUse")]
    pub r#action_to_use: Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementRuleActionOverrideActionToUse>,
    /// Name of the rule to override. See the [documentation](https://docs.aws.amazon.com/waf/latest/developerguide/aws-managed-rule-groups-list.html) for a list of names in the appropriate rule group in use.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
