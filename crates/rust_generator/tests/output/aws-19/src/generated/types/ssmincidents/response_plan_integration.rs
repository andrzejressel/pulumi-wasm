#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ResponsePlanIntegration {
    /// Details about the PagerDuty configuration for a response plan. The following values are supported:
    #[builder(into, default)]
    #[serde(rename = "pagerduties")]
    pub r#pagerduties: Box<Option<Vec<super::super::types::ssmincidents::ResponsePlanIntegrationPagerduty>>>,
}
