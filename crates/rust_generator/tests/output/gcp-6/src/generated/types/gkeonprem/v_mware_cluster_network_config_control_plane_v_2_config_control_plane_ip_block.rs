#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VMwareClusterNetworkConfigControlPlaneV2ConfigControlPlaneIpBlock {
    /// The network gateway used by the VMware User Cluster.
    #[builder(into, default)]
    #[serde(rename = "gateway")]
    pub r#gateway: Box<Option<String>>,
    /// The node's network configurations used by the VMware User Cluster.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "ips")]
    pub r#ips: Box<Option<Vec<super::super::types::gkeonprem::VMwareClusterNetworkConfigControlPlaneV2ConfigControlPlaneIpBlockIp>>>,
    /// The netmask used by the VMware User Cluster.
    #[builder(into, default)]
    #[serde(rename = "netmask")]
    pub r#netmask: Box<Option<String>>,
}
