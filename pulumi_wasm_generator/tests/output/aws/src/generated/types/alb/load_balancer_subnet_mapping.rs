#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LoadBalancerSubnetMapping {
    /// Allocation ID of the Elastic IP address for an internet-facing load balancer.
    #[builder(into, default)]
    #[serde(rename = "allocationId")]
    pub r#allocation_id: Box<Option<String>>,
    /// IPv6 address. You associate IPv6 CIDR blocks with your VPC and choose the subnets where you launch both internet-facing and internal Application Load Balancers or Network Load Balancers.
    #[builder(into, default)]
    #[serde(rename = "ipv6Address")]
    pub r#ipv_6_address: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "outpostId")]
    pub r#outpost_id: Box<Option<String>>,
    /// Private IPv4 address for an internal load balancer.
    #[builder(into, default)]
    #[serde(rename = "privateIpv4Address")]
    pub r#private_ipv_4_address: Box<Option<String>>,
    /// ID of the subnet of which to attach to the load balancer. You can specify only one subnet per Availability Zone.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
}
