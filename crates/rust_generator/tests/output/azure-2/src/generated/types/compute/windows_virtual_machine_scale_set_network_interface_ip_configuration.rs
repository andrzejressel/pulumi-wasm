#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WindowsVirtualMachineScaleSetNetworkInterfaceIpConfiguration {
    /// A list of Backend Address Pools ID's from a Application Gateway which this Virtual Machine Scale Set should be connected to.
    #[builder(into, default)]
    #[serde(rename = "applicationGatewayBackendAddressPoolIds")]
    pub r#application_gateway_backend_address_pool_ids: Box<Option<Vec<String>>>,
    /// A list of Application Security Group ID's which this Virtual Machine Scale Set should be connected to.
    #[builder(into, default)]
    #[serde(rename = "applicationSecurityGroupIds")]
    pub r#application_security_group_ids: Box<Option<Vec<String>>>,
    /// A list of Backend Address Pools ID's from a Load Balancer which this Virtual Machine Scale Set should be connected to.
    /// 
    /// > **Note:**  When the Virtual Machine Scale Set is configured to have public IPs per instance are created with a load balancer, the SKU of the Virtual Machine instance IPs is determined by the SKU of the Virtual Machine Scale Sets Load Balancer (e.g. `Basic` or `Standard`). Alternatively, you may use the `public_ip_prefix_id` field to generate instance-level IPs in a virtual machine scale set as well. The zonal properties of the prefix will be passed to the Virtual Machine instance IPs, though they will not be shown in the output. To view the public IP addresses assigned to the Virtual Machine Scale Sets Virtual Machine instances use the **az vmss list-instance-public-ips --resource-group `ResourceGroupName` --name `VirtualMachineScaleSetName`** CLI command.
    /// 
    /// > **Note:** When using this field you'll also need to configure a Rule for the Load Balancer, and use a `depends_on` between this resource and the Load Balancer Rule.
    #[builder(into, default)]
    #[serde(rename = "loadBalancerBackendAddressPoolIds")]
    pub r#load_balancer_backend_address_pool_ids: Box<Option<Vec<String>>>,
    /// A list of NAT Rule ID's from a Load Balancer which this Virtual Machine Scale Set should be connected to.
    /// 
    /// > **Note:** When using this field you'll also need to configure a Rule for the Load Balancer, and use a `depends_on` between this resource and the Load Balancer Rule.
    #[builder(into, default)]
    #[serde(rename = "loadBalancerInboundNatRulesIds")]
    pub r#load_balancer_inbound_nat_rules_ids: Box<Option<Vec<String>>>,
    /// The Name which should be used for this IP Configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Is this the Primary IP Configuration for this Network Interface? Defaults to `false`.
    /// 
    /// > **Note:** One `ip_configuration` block must be marked as Primary for each Network Interface.
    #[builder(into, default)]
    #[serde(rename = "primary")]
    pub r#primary: Box<Option<bool>>,
    /// A `public_ip_address` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "publicIpAddresses")]
    pub r#public_ip_addresses: Box<Option<Vec<super::super::types::compute::WindowsVirtualMachineScaleSetNetworkInterfaceIpConfigurationPublicIpAddress>>>,
    /// The ID of the Subnet which this IP Configuration should be connected to.
    /// 
    /// > `subnet_id` is required if `version` is set to `IPv4`.
    #[builder(into, default)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<Option<String>>,
    /// The Internet Protocol Version which should be used for this IP Configuration. Possible values are `IPv4` and `IPv6`. Defaults to `IPv4`.
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
