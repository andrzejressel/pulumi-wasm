#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ActionGroupEventHubReceiver {
    /// The name of the specific Event Hub queue.
    #[builder(into)]
    #[serde(rename = "eventHubName")]
    pub r#event_hub_name: Box<String>,
    /// The namespace name of the Event Hub.
    #[builder(into)]
    #[serde(rename = "eventHubNamespace")]
    pub r#event_hub_namespace: Box<String>,
    /// The name of the EventHub Receiver, must be unique within action group.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The ID for the subscription containing this Event Hub. Default to the subscription ID of the Action Group.
    #[builder(into, default)]
    #[serde(rename = "subscriptionId")]
    pub r#subscription_id: Box<Option<String>>,
    /// The Tenant ID for the subscription containing this Event Hub.
    #[builder(into, default)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<Option<String>>,
    /// Indicates whether to use common alert schema.
    #[builder(into, default)]
    #[serde(rename = "useCommonAlertSchema")]
    pub r#use_common_alert_schema: Box<Option<bool>>,
}
