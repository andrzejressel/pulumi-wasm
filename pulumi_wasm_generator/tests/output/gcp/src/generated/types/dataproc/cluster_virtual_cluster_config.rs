#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterVirtualClusterConfig {
    /// Configuration of auxiliary services used by this cluster. 
    /// Structure defined below.
    #[builder(into, default)]
    #[serde(rename = "auxiliaryServicesConfig")]
    pub r#auxiliary_services_config: Box<Option<super::super::types::dataproc::ClusterVirtualClusterConfigAuxiliaryServicesConfig>>,
    /// The configuration for running the Dataproc cluster on Kubernetes.
    /// Structure defined below.
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "kubernetesClusterConfig")]
    pub r#kubernetes_cluster_config: Box<Option<super::super::types::dataproc::ClusterVirtualClusterConfigKubernetesClusterConfig>>,
    /// The Cloud Storage staging bucket used to stage files,
    /// such as Hadoop jars, between client machines and the cluster.
    /// Note: If you don't explicitly specify a `staging_bucket`
    /// then GCP will auto create / assign one for you. However, you are not guaranteed
    /// an auto generated bucket which is solely dedicated to your cluster; it may be shared
    /// with other clusters in the same region/zone also choosing to use the auto generation
    /// option.
    #[builder(into, default)]
    #[serde(rename = "stagingBucket")]
    pub r#staging_bucket: Box<Option<String>>,
}
