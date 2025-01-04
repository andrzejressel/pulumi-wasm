#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SystemTopicEventSubscriptionWebhookEndpoint {
    /// The Azure Active Directory Application ID or URI to get the access token that will be included as the bearer token in delivery requests.
    #[builder(into, default)]
    #[serde(rename = "activeDirectoryAppIdOrUri")]
    pub r#active_directory_app_id_or_uri: Box<Option<String>>,
    /// The Azure Active Directory Tenant ID to get the access token that will be included as the bearer token in delivery requests.
    #[builder(into, default)]
    #[serde(rename = "activeDirectoryTenantId")]
    pub r#active_directory_tenant_id: Box<Option<String>>,
    /// The base url of the webhook where the Event Subscription will receive events.
    #[builder(into, default)]
    #[serde(rename = "baseUrl")]
    pub r#base_url: Box<Option<String>>,
    /// Maximum number of events per batch.
    #[builder(into, default)]
    #[serde(rename = "maxEventsPerBatch")]
    pub r#max_events_per_batch: Box<Option<i32>>,
    /// Preferred batch size in Kilobytes.
    #[builder(into, default)]
    #[serde(rename = "preferredBatchSizeInKilobytes")]
    pub r#preferred_batch_size_in_kilobytes: Box<Option<i32>>,
    /// Specifies the url of the webhook where the Event Subscription will receive events.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Box<String>,
}
