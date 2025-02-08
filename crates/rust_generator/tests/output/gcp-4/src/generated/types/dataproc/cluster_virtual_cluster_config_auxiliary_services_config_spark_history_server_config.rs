#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterVirtualClusterConfigAuxiliaryServicesConfigSparkHistoryServerConfig {
    /// Resource name of an existing Dataproc Cluster to act as a Spark History Server for the workload.
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "dataprocCluster")]
    pub r#dataproc_cluster: Box<Option<String>>,
}
