#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SmartDetectorAlertRuleActionGroup {
    /// Specifies a custom email subject if Email Receiver is specified in Monitor Action Group resource.
    #[builder(into, default)]
    #[serde(rename = "emailSubject")]
    pub r#email_subject: Box<Option<String>>,
    /// Specifies the action group ids.
    #[builder(into)]
    #[serde(rename = "ids")]
    pub r#ids: Box<Vec<String>>,
    /// A JSON String which Specifies the custom webhook payload if Webhook Receiver is specified in Monitor Action Group resource.
    #[builder(into, default)]
    #[serde(rename = "webhookPayload")]
    pub r#webhook_payload: Box<Option<String>>,
}
