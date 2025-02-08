#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct InstanceNetworkInterface {
    /// Access configurations, i.e. IPs via which this instance can be accessed via the Internet.
    #[builder(into, default)]
    #[serde(rename = "accessConfigs")]
    pub r#access_configs: Box<Option<Vec<super::super::types::compute::InstanceNetworkInterfaceAccessConfig>>>,
    /// An
    /// array of alias IP ranges for this network interface. Can only be specified for network
    /// interfaces on subnet-mode networks. Structure documented below.
    #[builder(into, default)]
    #[serde(rename = "aliasIpRanges")]
    pub r#alias_ip_ranges: Box<Option<Vec<super::super::types::compute::InstanceNetworkInterfaceAliasIpRange>>>,
    /// The prefix length of the primary internal IPv6 range.
    #[builder(into, default)]
    #[serde(rename = "internalIpv6PrefixLength")]
    pub r#internal_ipv_6_prefix_length: Box<Option<i32>>,
    /// An array of IPv6 access configurations for this interface.
    /// Currently, only one IPv6 access config, DIRECT_IPV6, is supported. If there is no ipv6AccessConfig
    /// specified, then this instance will have no external IPv6 Internet access. Structure documented below.
    #[builder(into, default)]
    #[serde(rename = "ipv6AccessConfigs")]
    pub r#ipv_6_access_configs: Box<Option<Vec<super::super::types::compute::InstanceNetworkInterfaceIpv6AccessConfig>>>,
    /// One of EXTERNAL, INTERNAL to indicate whether the IP can be accessed from the Internet.
    /// This field is always inherited from its subnetwork.
    #[builder(into, default)]
    #[serde(rename = "ipv6AccessType")]
    pub r#ipv_6_access_type: Box<Option<String>>,
    /// An IPv6 internal network address for this network interface. If not specified, Google Cloud will automatically assign an internal IPv6 address from the instance's subnetwork.
    #[builder(into, default)]
    #[serde(rename = "ipv6Address")]
    pub r#ipv_6_address: Box<Option<String>>,
    /// A unique name for the resource, required by GCE.
    /// Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The name or self_link of the network to attach this interface to.
    /// Either `network` or `subnetwork` must be provided. If network isn't provided it will
    /// be inferred from the subnetwork.
    #[builder(into, default)]
    #[serde(rename = "network")]
    pub r#network: Box<Option<String>>,
    /// The URL of the network attachment that this interface should connect to in the following format: `projects/{projectNumber}/regions/{region_name}/networkAttachments/{network_attachment_name}`.
    #[builder(into, default)]
    #[serde(rename = "networkAttachment")]
    pub r#network_attachment: Box<Option<String>>,
    /// The private IP address to assign to the instance. If
    /// empty, the address will be automatically assigned.
    #[builder(into, default)]
    #[serde(rename = "networkIp")]
    pub r#network_ip: Box<Option<String>>,
    /// The type of vNIC to be used on this interface. Possible values: GVNIC, VIRTIO_NET, IDPF. In the beta provider the additional values of MRDMA and IRDMA are supported.
    #[builder(into, default)]
    #[serde(rename = "nicType")]
    pub r#nic_type: Box<Option<String>>,
    /// The networking queue count that's specified by users for the network interface. Both Rx and Tx queues will be set to this number. It will be empty if not specified.
    #[builder(into, default)]
    #[serde(rename = "queueCount")]
    pub r#queue_count: Box<Option<i32>>,
    /// A full or partial URL to a security policy to add to this instance. If this field is set to an empty string it will remove the associated security policy.
    #[builder(into, default)]
    #[serde(rename = "securityPolicy")]
    pub r#security_policy: Box<Option<String>>,
    /// The stack type for this network interface to identify whether the IPv6 feature is enabled or not. Values are IPV4_IPV6 or IPV4_ONLY. If not specified, IPV4_ONLY will be used.
    #[builder(into, default)]
    #[serde(rename = "stackType")]
    pub r#stack_type: Box<Option<String>>,
    /// The name or self_link of the subnetwork to attach this
    /// interface to. Either `network` or `subnetwork` must be provided. If network isn't provided
    /// it will be inferred from the subnetwork. The subnetwork must exist in the same region this
    /// instance will be created in. If the network resource is in
    /// [legacy](https://cloud.google.com/vpc/docs/legacy) mode, do not specify this field. If the
    /// network is in auto subnet mode, specifying the subnetwork is optional. If the network is
    /// in custom subnet mode, specifying the subnetwork is required.
    #[builder(into, default)]
    #[serde(rename = "subnetwork")]
    pub r#subnetwork: Box<Option<String>>,
    /// The project in which the subnetwork belongs.
    /// If the `subnetwork` is a self_link, this field is set to the project
    /// defined in the subnetwork self_link. If the `subnetwork` is a name and this
    /// field is not provided, the provider project is used.
    #[builder(into, default)]
    #[serde(rename = "subnetworkProject")]
    pub r#subnetwork_project: Box<Option<String>>,
}
