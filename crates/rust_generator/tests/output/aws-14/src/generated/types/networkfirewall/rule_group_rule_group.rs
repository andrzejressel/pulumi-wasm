#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RuleGroupRuleGroup {
    /// A configuration block that defines the IP Set References for the rule group. See Reference Sets below for details. Please notes that there can only be a maximum of 5 `reference_sets` in a `rule_group`. See the [AWS documentation](https://docs.aws.amazon.com/network-firewall/latest/developerguide/rule-groups-ip-set-references.html#rule-groups-ip-set-reference-limits) for details.
    #[builder(into, default)]
    #[serde(rename = "referenceSets")]
    pub r#reference_sets: Box<Option<super::super::types::networkfirewall::RuleGroupRuleGroupReferenceSets>>,
    /// A configuration block that defines additional settings available to use in the rules defined in the rule group. Can only be specified for **stateful** rule groups. See Rule Variables below for details.
    #[builder(into, default)]
    #[serde(rename = "ruleVariables")]
    pub r#rule_variables: Box<Option<super::super::types::networkfirewall::RuleGroupRuleGroupRuleVariables>>,
    /// A configuration block that defines the stateful or stateless rules for the rule group. See Rules Source below for details.
    #[builder(into)]
    #[serde(rename = "rulesSource")]
    pub r#rules_source: Box<super::super::types::networkfirewall::RuleGroupRuleGroupRulesSource>,
    /// A configuration block that defines stateful rule options for the rule group. See Stateful Rule Options below for details.
    #[builder(into, default)]
    #[serde(rename = "statefulRuleOptions")]
    pub r#stateful_rule_options: Box<Option<super::super::types::networkfirewall::RuleGroupRuleGroupStatefulRuleOptions>>,
}
