#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetFirewallFirewallStatusCapacityUsageSummary {
    /// Capacity usage of CIDR blocks used by IP set references in a firewall.
    #[builder(into)]
    #[serde(rename = "cidrs")]
    pub r#cidrs: Box<Vec<super::super::types::networkfirewall::GetFirewallFirewallStatusCapacityUsageSummaryCidr>>,
}