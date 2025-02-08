#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ScaleSetNetworkProfileIpConfiguration {
    /// Specifies an array of references to backend address pools of application gateways. A scale set can reference backend address pools of multiple application gateways. Multiple scale sets can use the same application gateway.
    #[builder(into, default)]
    #[serde(rename = "applicationGatewayBackendAddressPoolIds")]
    pub r#application_gateway_backend_address_pool_ids: Box<Option<Vec<String>>>,
    /// Specifies up to `20` application security group IDs.
    #[builder(into, default)]
    #[serde(rename = "applicationSecurityGroupIds")]
    pub r#application_security_group_ids: Box<Option<Vec<String>>>,
    /// Specifies an array of references to backend address pools of load balancers. A scale set can reference backend address pools of one public and one internal load balancer. Multiple scale sets cannot use the same load balancer.
    /// 
    /// > **NOTE:** When using this field you'll also need to configure a Rule for the Load Balancer, and use a `depends_on` between this resource and the Load Balancer Rule.
    #[builder(into, default)]
    #[serde(rename = "loadBalancerBackendAddressPoolIds")]
    pub r#load_balancer_backend_address_pool_ids: Box<Option<Vec<String>>>,
    /// Specifies an array of references to inbound NAT pools for load balancers. A scale set can reference inbound NAT pools of one public and one internal load balancer. Multiple scale sets cannot use the same load balancer.
    /// 
    /// > **NOTE:** When using this field you'll also need to configure a Rule for the Load Balancer, and use a `depends_on` between this resource and the Load Balancer Rule.
    #[builder(into, default)]
    #[serde(rename = "loadBalancerInboundNatRulesIds")]
    pub r#load_balancer_inbound_nat_rules_ids: Box<Option<Vec<String>>>,
    /// Specifies name of the IP configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specifies if this ip_configuration is the primary one.
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: Box<bool>,
    /// Describes a virtual machines scale set IP Configuration's PublicIPAddress configuration. The `public_ip_address_configuration` block is documented below.
    #[builder(into, default)]
    #[serde(rename = "publicIpAddressConfiguration")]
    pub r#public_ip_address_configuration: Box<Option<super::super::types::compute::ScaleSetNetworkProfileIpConfigurationPublicIpAddressConfiguration>>,
    /// Specifies the identifier of the subnet.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
}
