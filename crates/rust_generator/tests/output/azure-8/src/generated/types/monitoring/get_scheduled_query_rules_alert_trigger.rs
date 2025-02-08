#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetScheduledQueryRulesAlertTrigger {
    #[builder(into)]
    #[serde(rename = "metricTriggers")]
    pub r#metric_triggers: Box<Vec<super::super::types::monitoring::GetScheduledQueryRulesAlertTriggerMetricTrigger>>,
    /// Evaluation operation for rule.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Box<String>,
    /// Result or count threshold based on which rule should be triggered.
    #[builder(into)]
    #[serde(rename = "threshold")]
    pub r#threshold: Box<f64>,
}
