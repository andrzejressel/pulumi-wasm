#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsStatelessRuleRuleDefinition {
    /// Set of actions to take on a packet that matches one of the stateless rule definition's `match_attributes`. For every rule you must specify 1 standard action, and you can add custom actions. Standard actions include: `aws:pass`, `aws:drop`, `aws:forward_to_sfe`.
    #[builder(into)]
    #[serde(rename = "actions")]
    pub r#actions: Box<Vec<String>>,
    /// A configuration block containing criteria for AWS Network Firewall to use to inspect an individual packet in stateless rule inspection. See Match Attributes below for details.
    #[builder(into)]
    #[serde(rename = "matchAttributes")]
    pub r#match_attributes: Box<super::super::types::networkfirewall::RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsStatelessRuleRuleDefinitionMatchAttributes>,
}
