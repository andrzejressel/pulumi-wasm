#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ConnectPeerConfiguration {
    #[builder(into, default)]
    #[serde(rename = "bgpConfigurations")]
    pub r#bgp_configurations: Box<Option<Vec<super::super::types::networkmanager::ConnectPeerConfigurationBgpConfiguration>>>,
    /// A Connect peer core network address.
    #[builder(into, default)]
    #[serde(rename = "coreNetworkAddress")]
    pub r#core_network_address: Box<Option<String>>,
    /// The inside IP addresses used for BGP peering. Required when the Connect attachment protocol is `GRE`. See `aws.networkmanager.ConnectAttachment` for details.
    #[builder(into, default)]
    #[serde(rename = "insideCidrBlocks")]
    pub r#inside_cidr_blocks: Box<Option<Vec<String>>>,
    /// The Connect peer address.
    /// 
    /// The following arguments are optional:
    #[builder(into, default)]
    #[serde(rename = "peerAddress")]
    pub r#peer_address: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<Option<String>>,
}
