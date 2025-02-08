#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetNotificationChannelFilter {
    /// Events to receive notifications for.
    #[builder(into)]
    #[serde(rename = "messageTypes")]
    pub r#message_types: Box<Vec<String>>,
    /// Severity levels to receive notifications for.
    #[builder(into)]
    #[serde(rename = "severities")]
    pub r#severities: Box<Vec<String>>,
}
