#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RuleGroupRuleGroupRuleVariablesPortSet {
    /// An unique alphanumeric string to identify the `port_set`.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// A configuration block that defines a set of port ranges. See Port Set below for details.
    #[builder(into)]
    #[serde(rename = "portSet")]
    pub r#port_set: Box<super::super::types::networkfirewall::RuleGroupRuleGroupRuleVariablesPortSetPortSet>,
}