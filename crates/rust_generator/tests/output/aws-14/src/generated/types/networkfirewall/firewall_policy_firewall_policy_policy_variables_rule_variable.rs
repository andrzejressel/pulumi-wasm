#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirewallPolicyFirewallPolicyPolicyVariablesRuleVariable {
    /// A configuration block that defines a set of IP addresses. See IP Set below for details.
    #[builder(into)]
    #[serde(rename = "ipSet")]
    pub r#ip_set: Box<super::super::types::networkfirewall::FirewallPolicyFirewallPolicyPolicyVariablesRuleVariableIpSet>,
    /// An alphanumeric string to identify the `ip_set`. Valid values: `HOME_NET`
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
}
