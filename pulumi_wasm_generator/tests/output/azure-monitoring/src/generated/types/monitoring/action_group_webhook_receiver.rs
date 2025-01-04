#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ActionGroupWebhookReceiver {
    /// The `aad_auth` block as defined below.
    /// 
    /// > **NOTE:** Before adding a secure webhook receiver by setting `aad_auth`, please read [the configuration instruction of the AAD application](https://docs.microsoft.com/azure/azure-monitor/platform/action-groups#secure-webhook).
    #[builder(into, default)]
    #[serde(rename = "aadAuth")]
    pub r#aad_auth: Box<Option<super::super::types::monitoring::ActionGroupWebhookReceiverAadAuth>>,
    /// The name of the webhook receiver. Names must be unique (case-insensitive) across all receivers within an action group.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The URI where webhooks should be sent.
    #[builder(into)]
    #[serde(rename = "serviceUri")]
    pub r#service_uri: Box<String>,
    /// Enables or disables the common alert schema.
    #[builder(into, default)]
    #[serde(rename = "useCommonAlertSchema")]
    pub r#use_common_alert_schema: Box<Option<bool>>,
}
