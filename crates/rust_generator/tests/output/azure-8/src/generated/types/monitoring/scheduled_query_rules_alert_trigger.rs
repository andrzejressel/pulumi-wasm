#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ScheduledQueryRulesAlertTrigger {
    /// A `metric_trigger` block as defined above. Trigger condition for metric query rule.
    #[builder(into, default)]
    #[serde(rename = "metricTrigger")]
    pub r#metric_trigger: Box<Option<super::super::types::monitoring::ScheduledQueryRulesAlertTriggerMetricTrigger>>,
    /// Evaluation operation for rule - 'GreaterThan', GreaterThanOrEqual', 'LessThan', or 'LessThanOrEqual'.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Box<String>,
    /// Result or count threshold based on which rule should be triggered. Values must be between 0 and 10000 inclusive.
    #[builder(into)]
    #[serde(rename = "threshold")]
    pub r#threshold: Box<f64>,
}
