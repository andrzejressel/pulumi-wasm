#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetScheduledQueryRulesLogCriteria {
    /// A `dimension` block as defined below.
    #[builder(into)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Box<Vec<super::super::types::monitoring::GetScheduledQueryRulesLogCriteriaDimension>>,
    /// Name of the metric.
    #[builder(into)]
    #[serde(rename = "metricName")]
    pub r#metric_name: Box<String>,
}
