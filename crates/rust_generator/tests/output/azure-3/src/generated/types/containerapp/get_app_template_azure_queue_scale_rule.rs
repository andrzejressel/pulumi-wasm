#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetAppTemplateAzureQueueScaleRule {
    #[builder(into)]
    #[serde(rename = "authentications")]
    pub r#authentications: Box<Vec<super::super::types::containerapp::GetAppTemplateAzureQueueScaleRuleAuthentication>>,
    /// The name of the Container App.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[builder(into)]
    #[serde(rename = "queueLength")]
    pub r#queue_length: Box<i32>,
    #[builder(into)]
    #[serde(rename = "queueName")]
    pub r#queue_name: Box<String>,
}
