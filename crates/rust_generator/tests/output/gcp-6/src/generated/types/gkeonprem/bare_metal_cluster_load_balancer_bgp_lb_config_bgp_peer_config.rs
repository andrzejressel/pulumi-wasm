#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BareMetalClusterLoadBalancerBgpLbConfigBgpPeerConfig {
    /// BGP autonomous system number (ASN) for the network that contains the
    /// external peer device.
    #[builder(into)]
    #[serde(rename = "asn")]
    pub r#asn: Box<i32>,
    /// The IP address of the control plane node that connects to the external
    /// peer.
    /// If you don't specify any control plane nodes, all control plane nodes
    /// can connect to the external peer. If you specify one or more IP addresses,
    /// only the nodes specified participate in peering sessions.
    #[builder(into, default)]
    #[serde(rename = "controlPlaneNodes")]
    pub r#control_plane_nodes: Box<Option<Vec<String>>>,
    /// The IP address of the external peer device.
    #[builder(into)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Box<String>,
}
