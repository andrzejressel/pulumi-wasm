#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterVirtualClusterConfigAuxiliaryServicesConfig {
    /// The Hive Metastore configuration for this workload.
    #[builder(into, default)]
    #[serde(rename = "metastoreConfig")]
    pub r#metastore_config: Box<Option<super::super::types::dataproc::ClusterVirtualClusterConfigAuxiliaryServicesConfigMetastoreConfig>>,
    /// The Spark History Server configuration for the workload.
    #[builder(into, default)]
    #[serde(rename = "sparkHistoryServerConfig")]
    pub r#spark_history_server_config: Box<Option<super::super::types::dataproc::ClusterVirtualClusterConfigAuxiliaryServicesConfigSparkHistoryServerConfig>>,
}
