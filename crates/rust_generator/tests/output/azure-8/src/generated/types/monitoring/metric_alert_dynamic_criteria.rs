#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MetricAlertDynamicCriteria {
    /// The statistic that runs over the metric values. Possible values are `Average`, `Count`, `Minimum`, `Maximum` and `Total`.
    #[builder(into)]
    #[serde(rename = "aggregation")]
    pub r#aggregation: Box<String>,
    /// The extent of deviation required to trigger an alert. Possible values are `Low`, `Medium` and `High`.
    #[builder(into)]
    #[serde(rename = "alertSensitivity")]
    pub r#alert_sensitivity: Box<String>,
    /// One or more `dimension` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Box<Option<Vec<super::super::types::monitoring::MetricAlertDynamicCriteriaDimension>>>,
    /// The number of violations to trigger an alert. Should be smaller or equal to `evaluation_total_count`. Defaults to `4`.
    #[builder(into, default)]
    #[serde(rename = "evaluationFailureCount")]
    pub r#evaluation_failure_count: Box<Option<i32>>,
    /// The number of aggregated lookback points. The lookback time window is calculated based on the aggregation granularity (`window_size`) and the selected number of aggregated points. Defaults to `4`.
    #[builder(into, default)]
    #[serde(rename = "evaluationTotalCount")]
    pub r#evaluation_total_count: Box<Option<i32>>,
    /// The [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) date from which to start learning the metric historical data and calculate the dynamic thresholds.
    #[builder(into, default)]
    #[serde(rename = "ignoreDataBefore")]
    pub r#ignore_data_before: Box<Option<String>>,
    /// One of the metric names to be monitored.
    #[builder(into)]
    #[serde(rename = "metricName")]
    pub r#metric_name: Box<String>,
    /// One of the metric namespaces to be monitored.
    #[builder(into)]
    #[serde(rename = "metricNamespace")]
    pub r#metric_namespace: Box<String>,
    /// The criteria operator. Possible values are `LessThan`, `GreaterThan` and `GreaterOrLessThan`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Box<String>,
    /// Skip the metric validation to allow creating an alert rule on a custom metric that isn't yet emitted?
    #[builder(into, default)]
    #[serde(rename = "skipMetricValidation")]
    pub r#skip_metric_validation: Box<Option<bool>>,
}
