#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetServiceNatIpConfiguration {
    /// The name of the private link service.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Value that indicates if the IP configuration is the primary configuration or not.
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: Box<bool>,
    /// The private IP address of the NAT IP configuration.
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Box<String>,
    /// The version of the IP Protocol.
    #[builder(into)]
    #[serde(rename = "privateIpAddressVersion")]
    pub r#private_ip_address_version: Box<String>,
    /// The ID of the subnet to be used by the service.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
}
