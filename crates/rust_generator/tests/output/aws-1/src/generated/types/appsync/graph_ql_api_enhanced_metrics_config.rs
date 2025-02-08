#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GraphQlApiEnhancedMetricsConfig {
    /// How data source metrics will be emitted to CloudWatch. Valid values: `FULL_REQUEST_DATA_SOURCE_METRICS`, `PER_DATA_SOURCE_METRICS`
    #[builder(into)]
    #[serde(rename = "dataSourceLevelMetricsBehavior")]
    pub r#data_source_level_metrics_behavior: Box<String>,
    /// How operation metrics will be emitted to CloudWatch. Valid values: `ENABLED`, `DISABLED`
    #[builder(into)]
    #[serde(rename = "operationLevelMetricsConfig")]
    pub r#operation_level_metrics_config: Box<String>,
    /// How resolver metrics will be emitted to CloudWatch. Valid values: `FULL_REQUEST_RESOLVER_METRICS`, `PER_RESOLVER_METRICS`
    #[builder(into)]
    #[serde(rename = "resolverLevelMetricsBehavior")]
    pub r#resolver_level_metrics_behavior: Box<String>,
}
