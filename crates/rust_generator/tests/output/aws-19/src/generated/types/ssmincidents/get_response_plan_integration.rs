#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetResponsePlanIntegration {
    /// Details about the PagerDuty configuration for a response plan. The following values are supported:
    #[builder(into)]
    #[serde(rename = "pagerduties")]
    pub r#pagerduties: Box<Vec<super::super::types::ssmincidents::GetResponsePlanIntegrationPagerduty>>,
}
