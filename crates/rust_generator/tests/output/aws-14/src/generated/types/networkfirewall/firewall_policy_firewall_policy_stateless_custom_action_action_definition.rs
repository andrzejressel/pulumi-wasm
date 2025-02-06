#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirewallPolicyFirewallPolicyStatelessCustomActionActionDefinition {
    /// A configuration block describing the stateless inspection criteria that publishes the specified metrics to Amazon CloudWatch for the matching packet. You can pair this custom action with any of the standard stateless rule actions. See Publish Metric Action below for details.
    #[builder(into)]
    #[serde(rename = "publishMetricAction")]
    pub r#publish_metric_action: Box<super::super::types::networkfirewall::FirewallPolicyFirewallPolicyStatelessCustomActionActionDefinitionPublishMetricAction>,
}
