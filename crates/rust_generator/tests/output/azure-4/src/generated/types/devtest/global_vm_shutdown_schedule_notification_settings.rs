#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GlobalVmShutdownScheduleNotificationSettings {
    /// E-mail address to which the notification will be sent.
    #[builder(into, default)]
    #[serde(rename = "email")]
    pub r#email: Box<Option<String>>,
    /// Whether to enable pre-shutdown notifications. Possible values are `true` and `false`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Time in minutes between 15 and 120 before a shutdown event at which a notification will be sent. Defaults to `30`.
    #[builder(into, default)]
    #[serde(rename = "timeInMinutes")]
    pub r#time_in_minutes: Box<Option<i32>>,
    /// The webhook URL to which the notification will be sent.
    #[builder(into, default)]
    #[serde(rename = "webhookUrl")]
    pub r#webhook_url: Box<Option<String>>,
}
