#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceTemplateNetworkInterface {
    /// Access configurations, i.e. IPs via which this
    /// instance can be accessed via the Internet. Omit to ensure that the instance
    /// is not accessible from the Internet (this means that ssh provisioners will
    /// not work unless you can send traffic to the instance's
    /// network (e.g. via tunnel or because it is running on another cloud instance
    /// on that network). This block can be specified once per `network_interface`. Structure documented below.
    #[builder(into, default)]
    #[serde(rename = "accessConfigs")]
    pub r#access_configs: Box<Option<Vec<super::super::types::compute::InstanceTemplateNetworkInterfaceAccessConfig>>>,
    /// An
    /// array of alias IP ranges for this network interface. Can only be specified for network
    /// interfaces on subnet-mode networks. Structure documented below.
    #[builder(into, default)]
    #[serde(rename = "aliasIpRanges")]
    pub r#alias_ip_ranges: Box<Option<Vec<super::super::types::compute::InstanceTemplateNetworkInterfaceAliasIpRange>>>,
    /// The prefix length of the primary internal IPv6 range.
    #[builder(into, default)]
    #[serde(rename = "internalIpv6PrefixLength")]
    pub r#internal_ipv_6_prefix_length: Box<Option<i32>>,
    /// An array of IPv6 access configurations for this interface.
    /// Currently, only one IPv6 access config, DIRECT_IPV6, is supported. If there is no ipv6AccessConfig
    /// specified, then this instance will have no external IPv6 Internet access. Structure documented below.
    #[builder(into, default)]
    #[serde(rename = "ipv6AccessConfigs")]
    pub r#ipv_6_access_configs: Box<Option<Vec<super::super::types::compute::InstanceTemplateNetworkInterfaceIpv6AccessConfig>>>,
    /// One of EXTERNAL, INTERNAL to indicate whether the IP can be accessed from the Internet. This field is always inherited from its subnetwork.
    #[builder(into, default)]
    #[serde(rename = "ipv6AccessType")]
    pub r#ipv_6_access_type: Box<Option<String>>,
    /// An IPv6 internal network address for this network interface. If not specified, Google Cloud will automatically assign an internal IPv6 address from the instance's subnetwork.
    #[builder(into, default)]
    #[serde(rename = "ipv6Address")]
    pub r#ipv_6_address: Box<Option<String>>,
    /// The name of the instance template. If you leave
    /// this blank, the provider will auto-generate a unique name.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The name or self_link of the network to attach this interface to.
    /// Use `network` attribute for Legacy or Auto subnetted networks and
    /// `subnetwork` for custom subnetted networks.
    #[builder(into, default)]
    #[serde(rename = "network")]
    pub r#network: Box<Option<String>>,
    /// The URL of the network attachment that this interface should connect to in the following format: projects/{projectNumber}/regions/{region_name}/networkAttachments/{network_attachment_name}.
    #[builder(into, default)]
    #[serde(rename = "networkAttachment")]
    pub r#network_attachment: Box<Option<String>>,
    /// The private IP address to assign to the instance. If
    /// empty, the address will be automatically assigned.
    #[builder(into, default)]
    #[serde(rename = "networkIp")]
    pub r#network_ip: Box<Option<String>>,
    /// The type of vNIC to be used on this interface. Possible values: GVNIC, VIRTIO_NET. In the beta provider the additional values of MRDMA and IRDMA are supported.
    #[builder(into, default)]
    #[serde(rename = "nicType")]
    pub r#nic_type: Box<Option<String>>,
    /// The networking queue count that's specified by users for the network interface. Both Rx and Tx queues will be set to this number. It will be empty if not specified.
    #[builder(into, default)]
    #[serde(rename = "queueCount")]
    pub r#queue_count: Box<Option<i32>>,
    /// The stack type for this network interface to identify whether the IPv6 feature is enabled or not. Values are IPV4_IPV6 or IPV4_ONLY. If not specified, IPV4_ONLY will be used.
    #[builder(into, default)]
    #[serde(rename = "stackType")]
    pub r#stack_type: Box<Option<String>>,
    /// the name of the subnetwork to attach this interface
    /// to. The subnetwork must exist in the same `region` this instance will be
    /// created in. Either `network` or `subnetwork` must be provided.
    #[builder(into, default)]
    #[serde(rename = "subnetwork")]
    pub r#subnetwork: Box<Option<String>>,
    /// The ID of the project in which the subnetwork belongs.
    /// If it is not provided, the provider project is used.
    #[builder(into, default)]
    #[serde(rename = "subnetworkProject")]
    pub r#subnetwork_project: Box<Option<String>>,
}
