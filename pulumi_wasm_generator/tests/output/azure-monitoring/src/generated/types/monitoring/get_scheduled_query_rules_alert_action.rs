#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetScheduledQueryRulesAlertAction {
    /// List of action group reference resource IDs.
    #[builder(into)]
    #[serde(rename = "actionGroups")]
    pub r#action_groups: Box<Vec<String>>,
    /// Custom payload to be sent for all webhook URI in Azure action group.
    #[builder(into)]
    #[serde(rename = "customWebhookPayload")]
    pub r#custom_webhook_payload: Box<String>,
    /// Custom subject override for all email IDs in Azure action group.
    #[builder(into)]
    #[serde(rename = "emailSubject")]
    pub r#email_subject: Box<String>,
}