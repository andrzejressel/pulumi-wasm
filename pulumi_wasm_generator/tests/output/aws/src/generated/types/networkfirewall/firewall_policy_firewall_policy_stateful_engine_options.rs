#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirewallPolicyFirewallPolicyStatefulEngineOptions {
    /// Amount of time that can pass without any traffic sent through the firewall before the firewall determines that the connection is idle.
    #[builder(into, default)]
    #[serde(rename = "flowTimeouts")]
    pub r#flow_timeouts: Box<Option<super::super::types::networkfirewall::FirewallPolicyFirewallPolicyStatefulEngineOptionsFlowTimeouts>>,
    /// Indicates how to manage the order of stateful rule evaluation for the policy. Default value: `DEFAULT_ACTION_ORDER`. Valid values: `DEFAULT_ACTION_ORDER`, `STRICT_ORDER`.
    #[builder(into, default)]
    #[serde(rename = "ruleOrder")]
    pub r#rule_order: Box<Option<String>>,
    /// Describes how to treat traffic which has broken midstream. Default value: `DROP`. Valid values: `DROP`, `CONTINUE`, `REJECT`.
    #[builder(into, default)]
    #[serde(rename = "streamExceptionPolicy")]
    pub r#stream_exception_policy: Box<Option<String>>,
}
