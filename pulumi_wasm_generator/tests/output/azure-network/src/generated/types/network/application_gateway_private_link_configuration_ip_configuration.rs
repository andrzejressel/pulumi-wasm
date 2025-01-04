#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationGatewayPrivateLinkConfigurationIpConfiguration {
    /// The name of the IP configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Is this the Primary IP Configuration?
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: Box<bool>,
    /// The Static IP Address which should be used.
    #[builder(into, default)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Box<Option<String>>,
    /// The allocation method used for the Private IP Address. Possible values are `Dynamic` and `Static`.
    #[builder(into)]
    #[serde(rename = "privateIpAddressAllocation")]
    pub r#private_ip_address_allocation: Box<String>,
    /// The ID of the subnet the private link configuration should connect to.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
}
