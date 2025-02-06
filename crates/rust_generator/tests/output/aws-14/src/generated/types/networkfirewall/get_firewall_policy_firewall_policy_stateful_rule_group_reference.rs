#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetFirewallPolicyFirewallPolicyStatefulRuleGroupReference {
    #[builder(into, default)]
    #[serde(rename = "overrides")]
    pub r#overrides: Box<Option<Vec<super::super::types::networkfirewall::GetFirewallPolicyFirewallPolicyStatefulRuleGroupReferenceOverride>>>,
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Box<i32>,
    #[builder(into)]
    #[serde(rename = "resourceArn")]
    pub r#resource_arn: Box<String>,
}
