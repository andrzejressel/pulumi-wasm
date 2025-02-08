#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct VMwareClusterNetworkConfig {
    /// Configuration for control plane V2 mode.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "controlPlaneV2Config")]
    pub r#control_plane_v_2_config: Box<Option<super::super::types::gkeonprem::VMwareClusterNetworkConfigControlPlaneV2Config>>,
    /// Configuration settings for a DHCP IP configuration.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "dhcpIpConfig")]
    pub r#dhcp_ip_config: Box<Option<super::super::types::gkeonprem::VMwareClusterNetworkConfigDhcpIpConfig>>,
    /// Represents common network settings irrespective of the host's IP address.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "hostConfig")]
    pub r#host_config: Box<Option<super::super::types::gkeonprem::VMwareClusterNetworkConfigHostConfig>>,
    /// All pods in the cluster are assigned an RFC1918 IPv4 address from these ranges.
    /// Only a single range is supported. This field cannot be changed after creation.
    #[builder(into)]
    #[serde(rename = "podAddressCidrBlocks")]
    pub r#pod_address_cidr_blocks: Box<Vec<String>>,
    /// All services in the cluster are assigned an RFC1918 IPv4 address
    /// from these ranges. Only a single range is supported.. This field
    /// cannot be changed after creation.
    #[builder(into)]
    #[serde(rename = "serviceAddressCidrBlocks")]
    pub r#service_address_cidr_blocks: Box<Vec<String>>,
    /// Configuration settings for a static IP configuration.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "staticIpConfig")]
    pub r#static_ip_config: Box<Option<super::super::types::gkeonprem::VMwareClusterNetworkConfigStaticIpConfig>>,
    /// vcenter_network specifies vCenter network name. Inherited from the admin cluster.
    #[builder(into, default)]
    #[serde(rename = "vcenterNetwork")]
    pub r#vcenter_network: Box<Option<String>>,
}
