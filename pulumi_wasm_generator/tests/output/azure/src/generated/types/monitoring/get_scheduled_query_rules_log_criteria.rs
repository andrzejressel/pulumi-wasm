#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
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