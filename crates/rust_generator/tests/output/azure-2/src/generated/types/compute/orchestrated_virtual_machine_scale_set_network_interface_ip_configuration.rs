#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct OrchestratedVirtualMachineScaleSetNetworkInterfaceIpConfiguration {
    /// A list of Backend Address Pools IDs from a Application Gateway which this Virtual Machine Scale Set should be connected to.
    #[builder(into, default)]
    #[serde(rename = "applicationGatewayBackendAddressPoolIds")]
    pub r#application_gateway_backend_address_pool_ids: Box<Option<Vec<String>>>,
    /// A list of Application Security Group IDs which this Virtual Machine Scale Set should be connected to.
    #[builder(into, default)]
    #[serde(rename = "applicationSecurityGroupIds")]
    pub r#application_security_group_ids: Box<Option<Vec<String>>>,
    /// A list of Backend Address Pools IDs from a Load Balancer which this Virtual Machine Scale Set should be connected to.
    /// 
    /// > **Note:** When using this field you'll also need to configure a Rule for the Load Balancer, and use a depends_on between this resource and the Load Balancer Rule.
    #[builder(into, default)]
    #[serde(rename = "loadBalancerBackendAddressPoolIds")]
    pub r#load_balancer_backend_address_pool_ids: Box<Option<Vec<String>>>,
    /// The Name which should be used for this IP Configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Is this the Primary IP Configuration for this Network Interface? Possible values are `true` and `false`. Defaults to `false`.
    /// 
    /// > **Note:** One `ip_configuration` block must be marked as Primary for each Network Interface.
    #[builder(into, default)]
    #[serde(rename = "primary")]
    pub r#primary: Box<Option<bool>>,
    /// A `public_ip_address` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "publicIpAddresses")]
    pub r#public_ip_addresses: Box<Option<Vec<super::super::types::compute::OrchestratedVirtualMachineScaleSetNetworkInterfaceIpConfigurationPublicIpAddress>>>,
    /// The ID of the Subnet which this IP Configuration should be connected to.
    /// 
    /// > **Note:** `subnet_id` is required if version is set to `IPv4`.
    #[builder(into, default)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<Option<String>>,
    /// The Internet Protocol Version which should be used for this IP Configuration. Possible values are `IPv4` and `IPv6`. Defaults to `IPv4`.
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
