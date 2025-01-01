#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetFirewallManagementIpConfiguration {
    /// The name of the Azure Firewall.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The private IP address associated with the Azure Firewall.
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Box<String>,
    /// The ID of the Public IP address of the Azure Firewall.
    #[builder(into)]
    #[serde(rename = "publicIpAddressId")]
    pub r#public_ip_address_id: Box<String>,
    /// The ID of the Subnet where the Azure Firewall is deployed.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
}
