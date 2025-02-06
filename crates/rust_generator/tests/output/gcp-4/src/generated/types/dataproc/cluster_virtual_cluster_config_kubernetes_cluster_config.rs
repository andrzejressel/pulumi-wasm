#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterVirtualClusterConfigKubernetesClusterConfig {
    /// The configuration for running the Dataproc cluster on GKE.
    #[builder(into)]
    #[serde(rename = "gkeClusterConfig")]
    pub r#gke_cluster_config: Box<super::super::types::dataproc::ClusterVirtualClusterConfigKubernetesClusterConfigGkeClusterConfig>,
    /// A namespace within the Kubernetes cluster to deploy into. 
    /// If this namespace does not exist, it is created.
    /// If it  exists, Dataproc verifies that another Dataproc VirtualCluster is not installed into it.
    /// If not specified, the name of the Dataproc Cluster is used.
    #[builder(into, default)]
    #[serde(rename = "kubernetesNamespace")]
    pub r#kubernetes_namespace: Box<Option<String>>,
    /// The software configuration for this Dataproc cluster running on Kubernetes.
    #[builder(into)]
    #[serde(rename = "kubernetesSoftwareConfig")]
    pub r#kubernetes_software_config: Box<super::super::types::dataproc::ClusterVirtualClusterConfigKubernetesClusterConfigKubernetesSoftwareConfig>,
}
