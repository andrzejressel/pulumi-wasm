#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRegionInstanceTemplateNetworkInterface {
    #[builder(into)]
    #[serde(rename = "accessConfigs")]
    pub r#access_configs: Box<Vec<super::super::types::compute::GetRegionInstanceTemplateNetworkInterfaceAccessConfig>>,
    /// An
    /// array of alias IP ranges for this network interface. Can only be specified for network
    /// interfaces on subnet-mode networks. Structure documented below.
    #[builder(into)]
    #[serde(rename = "aliasIpRanges")]
    pub r#alias_ip_ranges: Box<Vec<super::super::types::compute::GetRegionInstanceTemplateNetworkInterfaceAliasIpRange>>,
    /// The prefix length of the primary internal IPv6 range.
    #[builder(into)]
    #[serde(rename = "internalIpv6PrefixLength")]
    pub r#internal_ipv_6_prefix_length: Box<i32>,
    /// An array of IPv6 access configurations for this interface. Currently, only one IPv6 access config, DIRECT_IPV6, is supported. If there is no ipv6AccessConfig specified, then this instance will have no external IPv6 Internet access.
    #[builder(into)]
    #[serde(rename = "ipv6AccessConfigs")]
    pub r#ipv_6_access_configs: Box<Vec<super::super::types::compute::GetRegionInstanceTemplateNetworkInterfaceIpv6AccessConfig>>,
    /// One of EXTERNAL, INTERNAL to indicate whether the IP can be accessed from the Internet. This field is always inherited from its subnetwork.
    #[builder(into)]
    #[serde(rename = "ipv6AccessType")]
    pub r#ipv_6_access_type: Box<String>,
    /// An IPv6 internal network address for this network interface. If not specified, Google Cloud will automatically assign an internal IPv6 address from the instance's subnetwork.
    #[builder(into)]
    #[serde(rename = "ipv6Address")]
    pub r#ipv_6_address: Box<String>,
    /// The name of the instance template. One of `name` or `filter` must be provided.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The name or self_link of the network to attach this interface to.
    /// Use `network` attribute for Legacy or Auto subnetted networks and
    /// `subnetwork` for custom subnetted networks.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Box<String>,
    /// The private IP address to assign to the instance. If
    /// empty, the address will be automatically assigned.
    #[builder(into)]
    #[serde(rename = "networkIp")]
    pub r#network_ip: Box<String>,
    /// The type of vNIC to be used on this interface. Possible values:GVNIC, VIRTIO_NET, MRDMA, and IRDMA
    #[builder(into)]
    #[serde(rename = "nicType")]
    pub r#nic_type: Box<String>,
    /// The networking queue count that's specified by users for the network interface. Both Rx and Tx queues will be set to this number. It will be empty if not specified.
    #[builder(into)]
    #[serde(rename = "queueCount")]
    pub r#queue_count: Box<i32>,
    /// The stack type for this network interface to identify whether the IPv6 feature is enabled or not. If not specified, IPV4_ONLY will be used.
    #[builder(into)]
    #[serde(rename = "stackType")]
    pub r#stack_type: Box<String>,
    /// the name of the subnetwork to attach this interface
    /// to. The subnetwork must exist in the same `region` this instance will be
    /// created in. Either `network` or `subnetwork` must be provided.
    #[builder(into)]
    #[serde(rename = "subnetwork")]
    pub r#subnetwork: Box<String>,
    /// The ID of the project in which the subnetwork belongs.
    /// If it is not provided, the provider project is used.
    #[builder(into)]
    #[serde(rename = "subnetworkProject")]
    pub r#subnetwork_project: Box<String>,
}
