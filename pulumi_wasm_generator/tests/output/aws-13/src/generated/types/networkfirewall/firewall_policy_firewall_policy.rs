#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirewallPolicyFirewallPolicy {
    /// . Contains variables that you can use to override default Suricata settings in your firewall policy. See Rule Variables for details.
    #[builder(into, default)]
    #[serde(rename = "policyVariables")]
    pub r#policy_variables: Box<Option<super::super::types::networkfirewall::FirewallPolicyFirewallPolicyPolicyVariables>>,
    /// Set of actions to take on a packet if it does not match any stateful rules in the policy. This can only be specified if the policy has a `stateful_engine_options` block with a `rule_order` value of `STRICT_ORDER`. You can specify one of either or neither values of `aws:drop_strict` or `aws:drop_established`, as well as any combination of `aws:alert_strict` and `aws:alert_established`.
    #[builder(into, default)]
    #[serde(rename = "statefulDefaultActions")]
    pub r#stateful_default_actions: Box<Option<Vec<String>>>,
    /// A configuration block that defines options on how the policy handles stateful rules. See Stateful Engine Options below for details.
    #[builder(into, default)]
    #[serde(rename = "statefulEngineOptions")]
    pub r#stateful_engine_options: Box<Option<super::super::types::networkfirewall::FirewallPolicyFirewallPolicyStatefulEngineOptions>>,
    /// Set of configuration blocks containing references to the stateful rule groups that are used in the policy. See Stateful Rule Group Reference below for details.
    #[builder(into, default)]
    #[serde(rename = "statefulRuleGroupReferences")]
    pub r#stateful_rule_group_references: Box<Option<Vec<super::super::types::networkfirewall::FirewallPolicyFirewallPolicyStatefulRuleGroupReference>>>,
    /// Set of configuration blocks describing the custom action definitions that are available for use in the firewall policy's `stateless_default_actions`. See Stateless Custom Action below for details.
    #[builder(into, default)]
    #[serde(rename = "statelessCustomActions")]
    pub r#stateless_custom_actions: Box<Option<Vec<super::super::types::networkfirewall::FirewallPolicyFirewallPolicyStatelessCustomAction>>>,
    /// Set of actions to take on a packet if it does not match any of the stateless rules in the policy. You must specify one of the standard actions including: `aws:drop`, `aws:pass`, or `aws:forward_to_sfe`.
    /// In addition, you can specify custom actions that are compatible with your standard action choice. If you want non-matching packets to be forwarded for stateful inspection, specify `aws:forward_to_sfe`.
    #[builder(into)]
    #[serde(rename = "statelessDefaultActions")]
    pub r#stateless_default_actions: Box<Vec<String>>,
    /// Set of actions to take on a fragmented packet if it does not match any of the stateless rules in the policy. You must specify one of the standard actions including: `aws:drop`, `aws:pass`, or `aws:forward_to_sfe`.
    /// In addition, you can specify custom actions that are compatible with your standard action choice. If you want non-matching packets to be forwarded for stateful inspection, specify `aws:forward_to_sfe`.
    #[builder(into)]
    #[serde(rename = "statelessFragmentDefaultActions")]
    pub r#stateless_fragment_default_actions: Box<Vec<String>>,
    /// Set of configuration blocks containing references to the stateless rule groups that are used in the policy. See Stateless Rule Group Reference below for details.
    #[builder(into, default)]
    #[serde(rename = "statelessRuleGroupReferences")]
    pub r#stateless_rule_group_references: Box<Option<Vec<super::super::types::networkfirewall::FirewallPolicyFirewallPolicyStatelessRuleGroupReference>>>,
    /// The (ARN) of the TLS Inspection policy to attach to the FW Policy.  This must be added at creation of the resource per AWS documentation. "You can only add a TLS inspection configuration to a new policy, not to an existing policy."  This cannot be removed from a FW Policy.
    #[builder(into, default)]
    #[serde(rename = "tlsInspectionConfigurationArn")]
    pub r#tls_inspection_configuration_arn: Box<Option<String>>,
}
