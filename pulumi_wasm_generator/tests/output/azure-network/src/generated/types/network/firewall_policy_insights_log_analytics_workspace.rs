#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirewallPolicyInsightsLogAnalyticsWorkspace {
    /// The location of the Firewalls, that when matches this Log Analytics Workspace will be used to consume their logs.
    #[builder(into)]
    #[serde(rename = "firewallLocation")]
    pub r#firewall_location: Box<String>,
    /// The ID of the Log Analytics Workspace that the Firewalls associated with this Firewall Policy will send their logs to when their locations match the `firewall_location`.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
}
