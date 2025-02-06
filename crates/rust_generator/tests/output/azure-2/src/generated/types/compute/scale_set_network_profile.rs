#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ScaleSetNetworkProfile {
    /// Specifies whether to enable accelerated networking or not.
    #[builder(into, default)]
    #[serde(rename = "acceleratedNetworking")]
    pub r#accelerated_networking: Box<Option<bool>>,
    /// A `dns_settings` block as documented below.
    #[builder(into, default)]
    #[serde(rename = "dnsSettings")]
    pub r#dns_settings: Box<Option<super::super::types::compute::ScaleSetNetworkProfileDnsSettings>>,
    /// An `ip_configuration` block as documented below.
    #[builder(into)]
    #[serde(rename = "ipConfigurations")]
    pub r#ip_configurations: Box<Vec<super::super::types::compute::ScaleSetNetworkProfileIpConfiguration>>,
    /// Whether IP forwarding is enabled on this NIC. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "ipForwarding")]
    pub r#ip_forwarding: Box<Option<bool>>,
    /// Specifies the name of the network interface configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specifies the identifier for the network security group.
    #[builder(into, default)]
    #[serde(rename = "networkSecurityGroupId")]
    pub r#network_security_group_id: Box<Option<String>>,
    /// Indicates whether network interfaces created from the network interface configuration will be the primary NIC of the VM.
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: Box<bool>,
}
