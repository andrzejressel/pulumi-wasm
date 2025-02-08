#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterRemoteNetworkConfig {
    /// Configuration block with remote node network configuration for EKS Hybrid Nodes. Detailed below.
    #[builder(into)]
    #[serde(rename = "remoteNodeNetworks")]
    pub r#remote_node_networks: Box<super::super::types::eks::ClusterRemoteNetworkConfigRemoteNodeNetworks>,
    /// Configuration block with remote pod network configuration for EKS Hybrid Nodes. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "remotePodNetworks")]
    pub r#remote_pod_networks: Box<Option<super::super::types::eks::ClusterRemoteNetworkConfigRemotePodNetworks>>,
}
