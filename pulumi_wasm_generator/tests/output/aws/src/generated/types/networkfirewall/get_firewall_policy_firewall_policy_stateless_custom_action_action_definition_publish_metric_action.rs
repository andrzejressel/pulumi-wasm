#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetFirewallPolicyFirewallPolicyStatelessCustomActionActionDefinitionPublishMetricAction {
    #[builder(into)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Box<Vec<super::super::types::networkfirewall::GetFirewallPolicyFirewallPolicyStatelessCustomActionActionDefinitionPublishMetricActionDimension>>,
}