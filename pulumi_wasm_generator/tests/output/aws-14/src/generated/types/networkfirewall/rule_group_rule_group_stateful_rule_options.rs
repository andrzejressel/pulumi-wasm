#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RuleGroupRuleGroupStatefulRuleOptions {
    /// Indicates how to manage the order of the rule evaluation for the rule group. Default value: `DEFAULT_ACTION_ORDER`. Valid values: `DEFAULT_ACTION_ORDER`, `STRICT_ORDER`.
    #[builder(into)]
    #[serde(rename = "ruleOrder")]
    pub r#rule_order: Box<String>,
}
