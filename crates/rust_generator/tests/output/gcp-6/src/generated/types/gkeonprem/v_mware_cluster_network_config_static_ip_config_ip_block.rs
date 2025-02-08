#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VMwareClusterNetworkConfigStaticIpConfigIpBlock {
    /// The network gateway used by the VMware User Cluster.
    #[builder(into)]
    #[serde(rename = "gateway")]
    pub r#gateway: Box<String>,
    /// The node's network configurations used by the VMware User Cluster.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "ips")]
    pub r#ips: Box<Vec<super::super::types::gkeonprem::VMwareClusterNetworkConfigStaticIpConfigIpBlockIp>>,
    /// The netmask used by the VMware User Cluster.
    #[builder(into)]
    #[serde(rename = "netmask")]
    pub r#netmask: Box<String>,
}
