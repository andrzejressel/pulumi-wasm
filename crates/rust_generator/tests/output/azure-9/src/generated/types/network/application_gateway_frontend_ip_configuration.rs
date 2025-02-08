#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ApplicationGatewayFrontendIpConfiguration {
    /// The ID of the Rewrite Rule Set
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// The name of the Frontend IP Configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The Private IP Address to use for the Application Gateway.
    #[builder(into, default)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Box<Option<String>>,
    /// The Allocation Method for the Private IP Address. Possible values are `Dynamic` and `Static`. Defaults to `Dynamic`.
    #[builder(into, default)]
    #[serde(rename = "privateIpAddressAllocation")]
    pub r#private_ip_address_allocation: Box<Option<String>>,
    /// The ID of the associated private link configuration.
    #[builder(into, default)]
    #[serde(rename = "privateLinkConfigurationId")]
    pub r#private_link_configuration_id: Box<Option<String>>,
    /// The name of the private link configuration to use for this frontend IP configuration.
    #[builder(into, default)]
    #[serde(rename = "privateLinkConfigurationName")]
    pub r#private_link_configuration_name: Box<Option<String>>,
    /// The ID of a Public IP Address which the Application Gateway should use. The allocation method for the Public IP Address depends on the `sku` of this Application Gateway. Please refer to the [Azure documentation for public IP addresses](https://docs.microsoft.com/azure/virtual-network/public-ip-addresses#application-gateways) for details.
    #[builder(into, default)]
    #[serde(rename = "publicIpAddressId")]
    pub r#public_ip_address_id: Box<Option<String>>,
    /// The ID of the Subnet.
    #[builder(into, default)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<Option<String>>,
}
