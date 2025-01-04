#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetFirewallVirtualHub {
    /// The private IP address associated with the Azure Firewall.
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Box<String>,
    /// The list of public IP addresses associated with the Azure Firewall.
    #[builder(into)]
    #[serde(rename = "publicIpAddresses")]
    pub r#public_ip_addresses: Box<Vec<String>>,
    /// The number of public IPs assigned to the Azure Firewall.
    #[builder(into)]
    #[serde(rename = "publicIpCount")]
    pub r#public_ip_count: Box<i32>,
    /// The ID of the Virtual Hub where the Azure Firewall resides in.
    #[builder(into)]
    #[serde(rename = "virtualHubId")]
    pub r#virtual_hub_id: Box<String>,
}
