#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RuleGroupRuleGroupRuleVariablesIpSetIpSet {
    /// Set of IP addresses and address ranges, in CIDR notation.
    #[builder(into)]
    #[serde(rename = "definitions")]
    pub r#definitions: Box<Vec<String>>,
}