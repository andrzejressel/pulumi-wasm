#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirewallFirewallStatus {
    /// Set of subnets configured for use by the firewall.
    #[builder(into, default)]
    #[serde(rename = "syncStates")]
    pub r#sync_states: Box<Option<Vec<super::super::types::networkfirewall::FirewallFirewallStatusSyncState>>>,
}
