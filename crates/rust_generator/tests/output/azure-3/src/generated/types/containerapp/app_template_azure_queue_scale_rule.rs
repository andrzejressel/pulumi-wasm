#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AppTemplateAzureQueueScaleRule {
    /// One or more `authentication` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "authentications")]
    pub r#authentications: Box<Vec<super::super::types::containerapp::AppTemplateAzureQueueScaleRuleAuthentication>>,
    /// The name of the Scaling Rule
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The value of the length of the queue to trigger scaling actions.
    #[builder(into)]
    #[serde(rename = "queueLength")]
    pub r#queue_length: Box<i32>,
    /// The name of the Azure Queue
    #[builder(into)]
    #[serde(rename = "queueName")]
    pub r#queue_name: Box<String>,
}
