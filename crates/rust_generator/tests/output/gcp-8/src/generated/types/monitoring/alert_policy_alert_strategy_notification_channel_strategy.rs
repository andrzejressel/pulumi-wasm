#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AlertPolicyAlertStrategyNotificationChannelStrategy {
    /// The notification channels that these settings apply to. Each of these
    /// correspond to the name field in one of the NotificationChannel objects
    /// referenced in the notification_channels field of this AlertPolicy. The format is
    /// `projects/[PROJECT_ID_OR_NUMBER]/notificationChannels/[CHANNEL_ID]`
    #[builder(into, default)]
    #[serde(rename = "notificationChannelNames")]
    pub r#notification_channel_names: Box<Option<Vec<String>>>,
    /// The frequency at which to send reminder notifications for open incidents.
    #[builder(into, default)]
    #[serde(rename = "renotifyInterval")]
    pub r#renotify_interval: Box<Option<String>>,
}
