#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirewallVirtualHub {
    /// The private IP address associated with the Firewall.
    #[builder(into, default)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Box<Option<String>>,
    /// The list of public IP addresses associated with the Firewall.
    #[builder(into, default)]
    #[serde(rename = "publicIpAddresses")]
    pub r#public_ip_addresses: Box<Option<Vec<String>>>,
    /// Specifies the number of public IPs to assign to the Firewall. Defaults to `1`.
    #[builder(into, default)]
    #[serde(rename = "publicIpCount")]
    pub r#public_ip_count: Box<Option<i32>>,
    /// Specifies the ID of the Virtual Hub where the Firewall resides in.
    #[builder(into)]
    #[serde(rename = "virtualHubId")]
    pub r#virtual_hub_id: Box<String>,
}