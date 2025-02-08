#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterKubernetesNetworkConfig {
    /// Configuration block with elastic load balancing configuration for the cluster. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "elasticLoadBalancing")]
    pub r#elastic_load_balancing: Box<Option<super::super::types::eks::ClusterKubernetesNetworkConfigElasticLoadBalancing>>,
    /// The IP family used to assign Kubernetes pod and service addresses. Valid values are `ipv4` (default) and `ipv6`. You can only specify an IP family when you create a cluster, changing this value will force a new cluster to be created.
    #[builder(into, default)]
    #[serde(rename = "ipFamily")]
    pub r#ip_family: Box<Option<String>>,
    /// The CIDR block to assign Kubernetes pod and service IP addresses from. If you don't specify a block, Kubernetes assigns addresses from either the 10.100.0.0/16 or 172.20.0.0/16 CIDR blocks. We recommend that you specify a block that does not overlap with resources in other networks that are peered or connected to your VPC. You can only specify a custom CIDR block when you create a cluster, changing this value will force a new cluster to be created. The block must meet the following requirements:
    /// 
    /// * Within one of the following private IP address blocks: 10.0.0.0/8, 172.16.0.0/12, or 192.168.0.0/16.
    /// 
    /// * Doesn't overlap with any CIDR block assigned to the VPC that you selected for VPC.
    /// 
    /// * Between /24 and /12.
    #[builder(into, default)]
    #[serde(rename = "serviceIpv4Cidr")]
    pub r#service_ipv_4_cidr: Box<Option<String>>,
    /// The CIDR block that Kubernetes pod and service IP addresses are assigned from if you specified `ipv6` for `ip_family` when you created the cluster. Kubernetes assigns service addresses from the unique local address range (fc00::/7) because you can't specify a custom IPv6 CIDR block when you create the cluster.
    #[builder(into, default)]
    #[serde(rename = "serviceIpv6Cidr")]
    pub r#service_ipv_6_cidr: Box<Option<String>>,
}
