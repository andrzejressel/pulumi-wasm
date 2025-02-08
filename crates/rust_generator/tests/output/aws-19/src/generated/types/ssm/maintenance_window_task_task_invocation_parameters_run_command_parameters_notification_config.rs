#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersNotificationConfig {
    /// An Amazon Resource Name (ARN) for a Simple Notification Service (SNS) topic. Run Command pushes notifications about command status changes to this topic.
    #[builder(into, default)]
    #[serde(rename = "notificationArn")]
    pub r#notification_arn: Box<Option<String>>,
    /// The different events for which you can receive notifications. Valid values: `All`, `InProgress`, `Success`, `TimedOut`, `Cancelled`, and `Failed`
    #[builder(into, default)]
    #[serde(rename = "notificationEvents")]
    pub r#notification_events: Box<Option<Vec<String>>>,
    /// When specified with `Command`, receive notification when the status of a command changes. When specified with `Invocation`, for commands sent to multiple instances, receive notification on a per-instance basis when the status of a command changes. Valid values: `Command` and `Invocation`
    #[builder(into, default)]
    #[serde(rename = "notificationType")]
    pub r#notification_type: Box<Option<String>>,
}
