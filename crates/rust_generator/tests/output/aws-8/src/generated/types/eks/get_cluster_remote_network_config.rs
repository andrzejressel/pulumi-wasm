#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterRemoteNetworkConfig {
    /// The networks that can contain hybrid nodes.
    #[builder(into)]
    #[serde(rename = "remoteNodeNetworks")]
    pub r#remote_node_networks: Box<Vec<super::super::types::eks::GetClusterRemoteNetworkConfigRemoteNodeNetwork>>,
    /// The networks that can contain pods that run Kubernetes webhooks on hybrid nodes.
    #[builder(into)]
    #[serde(rename = "remotePodNetworks")]
    pub r#remote_pod_networks: Box<Vec<super::super::types::eks::GetClusterRemoteNetworkConfigRemotePodNetwork>>,
}
