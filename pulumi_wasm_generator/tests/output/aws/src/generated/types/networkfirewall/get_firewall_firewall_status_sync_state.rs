#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetFirewallFirewallStatusSyncState {
    /// Nested list describing the attachment status of the firewall's association with a single VPC subnet.
    #[builder(into)]
    #[serde(rename = "attachments")]
    pub r#attachments: Box<Vec<super::super::types::networkfirewall::GetFirewallFirewallStatusSyncStateAttachment>>,
    /// The Availability Zone where the subnet is configured.
    #[builder(into)]
    #[serde(rename = "availabilityZone")]
    pub r#availability_zone: Box<String>,
}
