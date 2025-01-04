#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetLbFrontendIpConfiguration {
    /// The id of the Frontend IP Configuration.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Specifies the name of the Load Balancer.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Private IP Address to assign to the Load Balancer.
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Box<String>,
    /// The allocation method for the Private IP Address used by this Load Balancer.
    #[builder(into)]
    #[serde(rename = "privateIpAddressAllocation")]
    pub r#private_ip_address_allocation: Box<String>,
    /// The Private IP Address Version, either `IPv4` or `IPv6`.
    #[builder(into)]
    #[serde(rename = "privateIpAddressVersion")]
    pub r#private_ip_address_version: Box<String>,
    /// The ID of a  Public IP Address which is associated with this Load Balancer.
    #[builder(into)]
    #[serde(rename = "publicIpAddressId")]
    pub r#public_ip_address_id: Box<String>,
    /// The ID of the Subnet which is associated with the IP Configuration.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
    /// A list of Availability Zones which the Load Balancer's IP Addresses should be created in.
    #[builder(into)]
    #[serde(rename = "zones")]
    pub r#zones: Box<Vec<String>>,
}
