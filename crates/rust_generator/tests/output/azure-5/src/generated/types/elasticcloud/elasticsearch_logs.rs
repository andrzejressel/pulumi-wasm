#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ElasticsearchLogs {
    /// A list of `filtering_tag` blocks as defined above.
    #[builder(into, default)]
    #[serde(rename = "filteringTags")]
    pub r#filtering_tags: Box<Option<Vec<super::super::types::elasticcloud::ElasticsearchLogsFilteringTag>>>,
    /// Specifies if the Azure Activity Logs should be sent to the Elasticsearch cluster. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "sendActivityLogs")]
    pub r#send_activity_logs: Box<Option<bool>>,
    /// Specifies if the AzureAD Logs should be sent to the Elasticsearch cluster. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "sendAzureadLogs")]
    pub r#send_azuread_logs: Box<Option<bool>>,
    /// Specifies if the Azure Subscription Logs should be sent to the Elasticsearch cluster. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "sendSubscriptionLogs")]
    pub r#send_subscription_logs: Box<Option<bool>>,
}
