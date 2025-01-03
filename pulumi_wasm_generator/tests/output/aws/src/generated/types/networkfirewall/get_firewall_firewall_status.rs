#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetFirewallFirewallStatus {
    /// Aggregated count of all resources used by reference sets in a firewall.
    #[builder(into)]
    #[serde(rename = "capacityUsageSummaries")]
    pub r#capacity_usage_summaries: Box<Vec<super::super::types::networkfirewall::GetFirewallFirewallStatusCapacityUsageSummary>>,
    /// Summary of sync states for all availability zones in which the firewall is configured.
    #[builder(into)]
    #[serde(rename = "configurationSyncStateSummary")]
    pub r#configuration_sync_state_summary: Box<String>,
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
    /// Set of subnets configured for use by the firewall.
    #[builder(into)]
    #[serde(rename = "syncStates")]
    pub r#sync_states: Box<Vec<super::super::types::networkfirewall::GetFirewallFirewallStatusSyncState>>,
}
