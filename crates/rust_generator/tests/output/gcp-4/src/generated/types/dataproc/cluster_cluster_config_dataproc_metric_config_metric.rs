#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterClusterConfigDataprocMetricConfigMetric {
    /// One or more [available OSS metrics] (https://cloud.google.com/dataproc/docs/guides/monitoring#available_oss_metrics) to collect for the metric course.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "metricOverrides")]
    pub r#metric_overrides: Box<Option<Vec<String>>>,
    /// A source for the collection of Dataproc OSS metrics (see [available OSS metrics](https://cloud.google.com//dataproc/docs/guides/monitoring#available_oss_metrics)).
    #[builder(into)]
    #[serde(rename = "metricSource")]
    pub r#metric_source: Box<String>,
}
