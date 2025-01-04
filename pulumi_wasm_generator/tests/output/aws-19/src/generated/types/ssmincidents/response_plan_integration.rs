#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ResponsePlanIntegration {
    /// Details about the PagerDuty configuration for a response plan. The following values are supported:
    #[builder(into, default)]
    #[serde(rename = "pagerduties")]
    pub r#pagerduties: Box<Option<Vec<super::super::types::ssmincidents::ResponsePlanIntegrationPagerduty>>>,
}
