#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FirewallFirewallStatusSyncState {
    /// Nested list describing the attachment status of the firewall's association with a single VPC subnet.
    #[builder(into, default)]
    #[serde(rename = "attachments")]
    pub r#attachments: Box<Option<Vec<super::super::types::networkfirewall::FirewallFirewallStatusSyncStateAttachment>>>,
    /// The Availability Zone where the subnet is configured.
    #[builder(into, default)]
    #[serde(rename = "availabilityZone")]
    pub r#availability_zone: Box<Option<String>>,
}
