#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterVirtualClusterConfigKubernetesClusterConfigGkeClusterConfigNodePoolTargetNodePoolConfig {
    /// The autoscaler configuration for this node pool. 
    /// The autoscaler is enabled only when a valid configuration is present.
    #[builder(into, default)]
    #[serde(rename = "autoscaling")]
    pub r#autoscaling: Box<Option<super::super::types::dataproc::ClusterVirtualClusterConfigKubernetesClusterConfigGkeClusterConfigNodePoolTargetNodePoolConfigAutoscaling>>,
    /// The node pool configuration.
    #[builder(into, default)]
    #[serde(rename = "config")]
    pub r#config: Box<Option<super::super::types::dataproc::ClusterVirtualClusterConfigKubernetesClusterConfigGkeClusterConfigNodePoolTargetNodePoolConfigConfig>>,
    /// The list of Compute Engine zones where node pool nodes associated 
    /// with a Dataproc on GKE virtual cluster will be located.
    /// - - -
    #[builder(into)]
    #[serde(rename = "locations")]
    pub r#locations: Box<Vec<String>>,
}
