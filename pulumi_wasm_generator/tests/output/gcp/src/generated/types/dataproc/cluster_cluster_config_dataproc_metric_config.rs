#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterClusterConfigDataprocMetricConfig {
    /// Metrics sources to enable.
    #[builder(into)]
    #[serde(rename = "metrics")]
    pub r#metrics: Box<Vec<super::super::types::dataproc::ClusterClusterConfigDataprocMetricConfigMetric>>,
}
