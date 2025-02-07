#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ScheduleNotificationSettings {
    /// The status of the notification. Possible values are `Enabled` and `Disabled`. Defaults to `Disabled`
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
    /// Time in minutes before event at which notification will be sent.
    #[builder(into, default)]
    #[serde(rename = "timeInMinutes")]
    pub r#time_in_minutes: Box<Option<i32>>,
    /// The webhook URL to which the notification will be sent.
    #[builder(into, default)]
    #[serde(rename = "webhookUrl")]
    pub r#webhook_url: Box<Option<String>>,
}
