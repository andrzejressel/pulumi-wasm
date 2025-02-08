#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterClusterConfigDataprocMetricConfig {
    /// Metrics sources to enable.
    #[builder(into)]
    #[serde(rename = "metrics")]
    pub r#metrics: Box<Vec<super::super::types::dataproc::ClusterClusterConfigDataprocMetricConfigMetric>>,
}
