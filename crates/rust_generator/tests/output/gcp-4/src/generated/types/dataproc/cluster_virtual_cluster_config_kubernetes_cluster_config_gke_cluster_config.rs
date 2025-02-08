#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterVirtualClusterConfigKubernetesClusterConfigGkeClusterConfig {
    /// A target GKE cluster to deploy to. It must be in the same project and region as the Dataproc cluster 
    /// (the GKE cluster can be zonal or regional)
    #[builder(into, default)]
    #[serde(rename = "gkeClusterTarget")]
    pub r#gke_cluster_target: Box<Option<String>>,
    /// GKE node pools where workloads will be scheduled. At least one node pool must be assigned the `DEFAULT` 
    /// GkeNodePoolTarget.Role. If a GkeNodePoolTarget is not specified, Dataproc constructs a `DEFAULT` GkeNodePoolTarget.
    /// Each role can be given to only one GkeNodePoolTarget. All node pools must have the same location settings.
    #[builder(into, default)]
    #[serde(rename = "nodePoolTargets")]
    pub r#node_pool_targets: Box<Option<Vec<super::super::types::dataproc::ClusterVirtualClusterConfigKubernetesClusterConfigGkeClusterConfigNodePoolTarget>>>,
}
