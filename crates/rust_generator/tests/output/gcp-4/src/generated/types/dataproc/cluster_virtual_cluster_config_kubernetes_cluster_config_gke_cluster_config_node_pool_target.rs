#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterVirtualClusterConfigKubernetesClusterConfigGkeClusterConfigNodePoolTarget {
    /// The target GKE node pool.
    #[builder(into)]
    #[serde(rename = "nodePool")]
    pub r#node_pool: Box<String>,
    /// The configuration for the GKE node pool. 
    /// If specified, Dataproc attempts to create a node pool with the specified shape.
    /// If one with the same name already exists, it is verified against all specified fields.
    /// If a field differs, the virtual cluster creation will fail.
    #[builder(into, default)]
    #[serde(rename = "nodePoolConfig")]
    pub r#node_pool_config: Box<Option<super::super::types::dataproc::ClusterVirtualClusterConfigKubernetesClusterConfigGkeClusterConfigNodePoolTargetNodePoolConfig>>,
    /// The roles associated with the GKE node pool. 
    /// One of `"DEFAULT"`, `"CONTROLLER"`, `"SPARK_DRIVER"` or `"SPARK_EXECUTOR"`.
    #[builder(into)]
    #[serde(rename = "roles")]
    pub r#roles: Box<Vec<String>>,
}
