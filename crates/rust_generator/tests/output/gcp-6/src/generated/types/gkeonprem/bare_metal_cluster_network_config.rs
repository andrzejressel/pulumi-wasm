#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BareMetalClusterNetworkConfig {
    /// Enables the use of advanced Anthos networking features, such as Bundled
    /// Load Balancing with BGP or the egress NAT gateway.
    /// Setting configuration for advanced networking features will automatically
    /// set this flag.
    #[builder(into, default)]
    #[serde(rename = "advancedNetworking")]
    pub r#advanced_networking: Box<Option<bool>>,
    /// A nested object resource.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "islandModeCidr")]
    pub r#island_mode_cidr: Box<Option<super::super::types::gkeonprem::BareMetalClusterNetworkConfigIslandModeCidr>>,
    /// Configuration for multiple network interfaces.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "multipleNetworkInterfacesConfig")]
    pub r#multiple_network_interfaces_config: Box<Option<super::super::types::gkeonprem::BareMetalClusterNetworkConfigMultipleNetworkInterfacesConfig>>,
    /// Configuration for SR-IOV.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "srIovConfig")]
    pub r#sr_iov_config: Box<Option<super::super::types::gkeonprem::BareMetalClusterNetworkConfigSrIovConfig>>,
}
