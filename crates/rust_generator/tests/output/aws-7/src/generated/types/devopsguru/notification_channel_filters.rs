#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NotificationChannelFilters {
    /// Events to receive notifications for. Valid values are `NEW_INSIGHT`, `CLOSED_INSIGHT`, `NEW_ASSOCIATION`, `SEVERITY_UPGRADED`, and `NEW_RECOMMENDATION`.
    #[builder(into, default)]
    #[serde(rename = "messageTypes")]
    pub r#message_types: Box<Option<Vec<String>>>,
    /// Severity levels to receive notifications for. Valid values are `LOW`, `MEDIUM`, and `HIGH`.
    #[builder(into, default)]
    #[serde(rename = "severities")]
    pub r#severities: Box<Option<Vec<String>>>,
}
