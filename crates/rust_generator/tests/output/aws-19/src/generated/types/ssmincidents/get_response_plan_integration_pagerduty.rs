#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetResponsePlanIntegrationPagerduty {
    /// The name of the PagerDuty configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The ID of the AWS Secrets Manager secret that stores your PagerDuty key &mdash; either a General Access REST API Key or User Token REST API Key &mdash; and other user credentials.
    #[builder(into)]
    #[serde(rename = "secretId")]
    pub r#secret_id: Box<String>,
    /// The ID of the PagerDuty service that the response plan associates with an incident when it launches.
    #[builder(into)]
    #[serde(rename = "serviceId")]
    pub r#service_id: Box<String>,
}
