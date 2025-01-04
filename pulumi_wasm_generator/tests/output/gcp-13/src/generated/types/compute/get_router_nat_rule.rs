#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRouterNatRule {
    /// The action to be enforced for traffic that matches this rule.
    #[builder(into)]
    #[serde(rename = "actions")]
    pub r#actions: Box<Vec<super::super::types::compute::GetRouterNatRuleAction>>,
    /// An optional description of this rule.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// CEL expression that specifies the match condition that egress traffic from a VM is evaluated against.
    /// If it evaluates to true, the corresponding action is enforced.
    /// 
    /// The following examples are valid match expressions for public NAT:
    /// 
    /// "inIpRange(destination.ip, '1.1.0.0/16') || inIpRange(destination.ip, '2.2.0.0/16')"
    /// 
    /// "destination.ip == '1.1.0.1' || destination.ip == '8.8.8.8'"
    /// 
    /// The following example is a valid match expression for private NAT:
    /// 
    /// "nexthop.hub == 'https://networkconnectivity.googleapis.com/v1alpha1/projects/my-project/global/hub/hub-1'"
    #[builder(into)]
    #[serde(rename = "match")]
    pub r#match_: Box<String>,
    /// An integer uniquely identifying a rule in the list.
    /// The rule number must be a positive value between 0 and 65000, and must be unique among rules within a NAT.
    #[builder(into)]
    #[serde(rename = "ruleNumber")]
    pub r#rule_number: Box<i32>,
}
