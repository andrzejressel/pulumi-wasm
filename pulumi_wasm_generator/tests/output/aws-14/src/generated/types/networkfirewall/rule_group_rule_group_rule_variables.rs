#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RuleGroupRuleGroupRuleVariables {
    /// Set of configuration blocks that define IP address information. See IP Sets below for details.
    #[builder(into, default)]
    #[serde(rename = "ipSets")]
    pub r#ip_sets: Box<Option<Vec<super::super::types::networkfirewall::RuleGroupRuleGroupRuleVariablesIpSet>>>,
    /// Set of configuration blocks that define port range information. See Port Sets below for details.
    #[builder(into, default)]
    #[serde(rename = "portSets")]
    pub r#port_sets: Box<Option<Vec<super::super::types::networkfirewall::RuleGroupRuleGroupRuleVariablesPortSet>>>,
}
