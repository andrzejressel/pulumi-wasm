#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LoadBalancerFrontendIpConfiguration {
    /// The Frontend IP Configuration ID of a Gateway SKU Load Balancer.
    #[builder(into, default)]
    #[serde(rename = "gatewayLoadBalancerFrontendIpConfigurationId")]
    pub r#gateway_load_balancer_frontend_ip_configuration_id: Box<Option<String>>,
    /// The id of the Frontend IP Configuration.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// The list of IDs of inbound rules that use this frontend IP.
    #[builder(into, default)]
    #[serde(rename = "inboundNatRules")]
    pub r#inbound_nat_rules: Box<Option<Vec<String>>>,
    /// The list of IDs of load balancing rules that use this frontend IP.
    #[builder(into, default)]
    #[serde(rename = "loadBalancerRules")]
    pub r#load_balancer_rules: Box<Option<Vec<String>>>,
    /// Specifies the name of the frontend IP configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The list of IDs outbound rules that use this frontend IP.
    #[builder(into, default)]
    #[serde(rename = "outboundRules")]
    pub r#outbound_rules: Box<Option<Vec<String>>>,
    /// Private IP Address to assign to the Load Balancer. The last one and first four IPs in any range are reserved and cannot be manually assigned.
    #[builder(into, default)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Box<Option<String>>,
    /// The allocation method for the Private IP Address used by this Load Balancer. Possible values as `Dynamic` and `Static`.
    #[builder(into, default)]
    #[serde(rename = "privateIpAddressAllocation")]
    pub r#private_ip_address_allocation: Box<Option<String>>,
    /// The version of IP that the Private IP Address is. Possible values are `IPv4` or `IPv6`.
    #[builder(into, default)]
    #[serde(rename = "privateIpAddressVersion")]
    pub r#private_ip_address_version: Box<Option<String>>,
    /// The ID of a Public IP Address which should be associated with the Load Balancer.
    #[builder(into, default)]
    #[serde(rename = "publicIpAddressId")]
    pub r#public_ip_address_id: Box<Option<String>>,
    /// The ID of a Public IP Prefix which should be associated with the Load Balancer. Public IP Prefix can only be used with outbound rules.
    #[builder(into, default)]
    #[serde(rename = "publicIpPrefixId")]
    pub r#public_ip_prefix_id: Box<Option<String>>,
    /// The ID of the Subnet which should be associated with the IP Configuration.
    #[builder(into, default)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<Option<String>>,
    /// Specifies a list of Availability Zones in which the IP Address for this Load Balancer should be located.
    /// 
    /// > **NOTE:** Availability Zones are only supported with a [Standard SKU](https://docs.microsoft.com/azure/load-balancer/load-balancer-standard-availability-zones) and [in select regions](https://docs.microsoft.com/azure/availability-zones/az-overview) at this time.
    #[builder(into, default)]
    #[serde(rename = "zones")]
    pub r#zones: Box<Option<Vec<String>>>,
}
