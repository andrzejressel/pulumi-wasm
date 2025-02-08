#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterNetworking {
    /// All pods in the cluster are assigned an RFC1918 IPv4 address from these
    /// blocks. Only a single block is supported. This field cannot be changed
    /// after creation.
    #[builder(into)]
    #[serde(rename = "clusterIpv4CidrBlocks")]
    pub r#cluster_ipv_4_cidr_blocks: Box<Vec<String>>,
    /// If specified, dual stack mode is enabled and all pods in the cluster are
    /// assigned an IPv6 address from these blocks alongside from an IPv4
    /// address. Only a single block is supported. This field cannot be changed
    /// after creation.
    #[builder(into, default)]
    #[serde(rename = "clusterIpv6CidrBlocks")]
    pub r#cluster_ipv_6_cidr_blocks: Box<Option<Vec<String>>>,
    /// (Output)
    /// IP addressing type of this cluster i.e. SINGLESTACK_V4 vs DUALSTACK_V4_V6.
    #[builder(into, default)]
    #[serde(rename = "networkType")]
    pub r#network_type: Box<Option<String>>,
    /// All services in the cluster are assigned an RFC1918 IPv4 address from these
    /// blocks. Only a single block is supported. This field cannot be changed
    /// after creation.
    #[builder(into)]
    #[serde(rename = "servicesIpv4CidrBlocks")]
    pub r#services_ipv_4_cidr_blocks: Box<Vec<String>>,
    /// If specified, dual stack mode is enabled and all services in the cluster are
    /// assigned an IPv6 address from these blocks alongside from an IPv4
    /// address. Only a single block is supported. This field cannot be changed
    /// after creation.
    #[builder(into, default)]
    #[serde(rename = "servicesIpv6CidrBlocks")]
    pub r#services_ipv_6_cidr_blocks: Box<Option<Vec<String>>>,
}
