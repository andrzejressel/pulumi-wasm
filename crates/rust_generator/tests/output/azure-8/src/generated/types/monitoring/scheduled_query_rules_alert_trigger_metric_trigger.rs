#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ScheduledQueryRulesAlertTriggerMetricTrigger {
    /// Evaluation of metric on a particular column.
    #[builder(into, default)]
    #[serde(rename = "metricColumn")]
    pub r#metric_column: Box<Option<String>>,
    /// Metric Trigger Type - 'Consecutive' or 'Total'.
    #[builder(into)]
    #[serde(rename = "metricTriggerType")]
    pub r#metric_trigger_type: Box<String>,
    /// Evaluation operation for rule - 'Equal', 'GreaterThan', GreaterThanOrEqual', 'LessThan', or 'LessThanOrEqual'.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Box<String>,
    /// The threshold of the metric trigger. Values must be between 0 and 10000 inclusive.
    #[builder(into)]
    #[serde(rename = "threshold")]
    pub r#threshold: Box<f64>,
}
