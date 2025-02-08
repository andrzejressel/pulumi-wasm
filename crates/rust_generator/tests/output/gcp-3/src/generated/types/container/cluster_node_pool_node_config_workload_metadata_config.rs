#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterNodePoolNodeConfigWorkloadMetadataConfig {
    /// How to expose the node metadata to the workload running on the node.
    /// Accepted values are:
    /// * UNSPECIFIED: Not Set
    /// * GCE_METADATA: Expose all Compute Engine metadata to pods.
    /// * GKE_METADATA: Run the GKE Metadata Server on this node. The GKE Metadata Server exposes a metadata API to workloads that is compatible with the V1 Compute Metadata APIs exposed by the Compute Engine and App Engine Metadata Servers. This feature can only be enabled if [workload identity](https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity) is enabled at the cluster level.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
}
