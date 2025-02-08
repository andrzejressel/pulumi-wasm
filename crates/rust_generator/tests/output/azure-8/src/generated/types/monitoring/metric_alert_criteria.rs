#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct MetricAlertCriteria {
    /// The statistic that runs over the metric values. Possible values are `Average`, `Count`, `Minimum`, `Maximum` and `Total`.
    #[builder(into)]
    #[serde(rename = "aggregation")]
    pub r#aggregation: Box<String>,
    /// One or more `dimension` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Box<Option<Vec<super::super::types::monitoring::MetricAlertCriteriaDimension>>>,
    /// One of the metric names to be monitored.
    #[builder(into)]
    #[serde(rename = "metricName")]
    pub r#metric_name: Box<String>,
    /// One of the metric namespaces to be monitored.
    #[builder(into)]
    #[serde(rename = "metricNamespace")]
    pub r#metric_namespace: Box<String>,
    /// The criteria operator. Possible values are `Equals`, `GreaterThan`, `GreaterThanOrEqual`, `LessThan` and `LessThanOrEqual`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Box<String>,
    /// Skip the metric validation to allow creating an alert rule on a custom metric that isn't yet emitted? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "skipMetricValidation")]
    pub r#skip_metric_validation: Box<Option<bool>>,
    /// The criteria threshold value that activates the alert.
    #[builder(into)]
    #[serde(rename = "threshold")]
    pub r#threshold: Box<f64>,
}
