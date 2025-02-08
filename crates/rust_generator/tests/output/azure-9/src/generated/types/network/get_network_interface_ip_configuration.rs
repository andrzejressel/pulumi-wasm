#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetNetworkInterfaceIpConfiguration {
    /// A list of Backend Address Pool IDs within a Application Gateway that this Network Interface is connected to.
    #[builder(into)]
    #[serde(rename = "applicationGatewayBackendAddressPoolsIds")]
    pub r#application_gateway_backend_address_pools_ids: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "applicationSecurityGroupIds")]
    pub r#application_security_group_ids: Box<Vec<String>>,
    /// The Frontend IP Configuration ID of a Gateway SKU Load Balancer the Network Interface is consuming.
    #[builder(into)]
    #[serde(rename = "gatewayLoadBalancerFrontendIpConfigurationId")]
    pub r#gateway_load_balancer_frontend_ip_configuration_id: Box<String>,
    /// A list of Backend Address Pool IDs within a Load Balancer that this Network Interface is connected to.
    #[builder(into)]
    #[serde(rename = "loadBalancerBackendAddressPoolsIds")]
    pub r#load_balancer_backend_address_pools_ids: Box<Vec<String>>,
    /// A list of Inbound NAT Rule IDs within a Load Balancer that this Network Interface is connected to.
    #[builder(into)]
    #[serde(rename = "loadBalancerInboundNatRulesIds")]
    pub r#load_balancer_inbound_nat_rules_ids: Box<Vec<String>>,
    /// Specifies the name of the Network Interface.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// is this the Primary IP Configuration for this Network Interface?
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: Box<bool>,
    /// The Private IP Address assigned to this Network Interface.
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Box<String>,
    /// The IP Address allocation type for the Private address, such as `Dynamic` or `Static`.
    #[builder(into)]
    #[serde(rename = "privateIpAddressAllocation")]
    pub r#private_ip_address_allocation: Box<String>,
    #[builder(into)]
    #[serde(rename = "privateIpAddressVersion")]
    pub r#private_ip_address_version: Box<String>,
    /// The ID of the Public IP Address which is connected to this Network Interface.
    #[builder(into)]
    #[serde(rename = "publicIpAddressId")]
    pub r#public_ip_address_id: Box<String>,
    /// The ID of the Subnet which the Network Interface is connected to.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
}
