#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetFirewallPolicyFirewallPolicyStatelessCustomActionActionDefinition {
    #[builder(into)]
    #[serde(rename = "publishMetricActions")]
    pub r#publish_metric_actions: Box<Vec<super::super::types::networkfirewall::GetFirewallPolicyFirewallPolicyStatelessCustomActionActionDefinitionPublishMetricAction>>,
}