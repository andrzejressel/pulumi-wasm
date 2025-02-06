#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MetricStreamStatisticsConfiguration {
    /// The additional statistics to stream for the metrics listed in `include_metrics`.
    #[builder(into)]
    #[serde(rename = "additionalStatistics")]
    pub r#additional_statistics: Box<Vec<String>>,
    /// An array that defines the metrics that are to have additional statistics streamed. See details below.
    #[builder(into)]
    #[serde(rename = "includeMetrics")]
    pub r#include_metrics: Box<Vec<super::super::types::cloudwatch::MetricStreamStatisticsConfigurationIncludeMetric>>,
}
