#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetScheduledQueryRulesAlertTriggerMetricTrigger {
    #[builder(into)]
    #[serde(rename = "metricColumn")]
    pub r#metric_column: Box<String>,
    #[builder(into)]
    #[serde(rename = "metricTriggerType")]
    pub r#metric_trigger_type: Box<String>,
    /// Evaluation operation for rule.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Box<String>,
    /// Result or count threshold based on which rule should be triggered.
    #[builder(into)]
    #[serde(rename = "threshold")]
    pub r#threshold: Box<f64>,
}
