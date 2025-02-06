#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ScheduledQueryRulesAlertAction {
    /// List of action group reference resource IDs.
    #[builder(into)]
    #[serde(rename = "actionGroups")]
    pub r#action_groups: Box<Vec<String>>,
    /// Custom payload to be sent for all webhook payloads in alerting action.
    #[builder(into, default)]
    #[serde(rename = "customWebhookPayload")]
    pub r#custom_webhook_payload: Box<Option<String>>,
    /// Custom subject override for all email ids in Azure action group.
    #[builder(into, default)]
    #[serde(rename = "emailSubject")]
    pub r#email_subject: Box<Option<String>>,
}
