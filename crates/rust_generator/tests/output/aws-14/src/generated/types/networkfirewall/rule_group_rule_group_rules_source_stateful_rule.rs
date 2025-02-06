#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RuleGroupRuleGroupRulesSourceStatefulRule {
    /// Action to take with packets in a traffic flow when the flow matches the stateful rule criteria. For all actions, AWS Network Firewall performs the specified action and discontinues stateful inspection of the traffic flow. Valid values: `ALERT`, `DROP`, `PASS`, or `REJECT`.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// A configuration block containing the stateful 5-tuple inspection criteria for the rule, used to inspect traffic flows. See Header below for details.
    #[builder(into)]
    #[serde(rename = "header")]
    pub r#header: Box<super::super::types::networkfirewall::RuleGroupRuleGroupRulesSourceStatefulRuleHeader>,
    /// Set of configuration blocks containing additional settings for a stateful rule. See Rule Option below for details.
    #[builder(into)]
    #[serde(rename = "ruleOptions")]
    pub r#rule_options: Box<Vec<super::super::types::networkfirewall::RuleGroupRuleGroupRulesSourceStatefulRuleRuleOption>>,
}
