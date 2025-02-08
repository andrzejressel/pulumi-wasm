#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BudgetAllUpdatesRule {
    /// Boolean. When set to true, disables default notifications sent
    /// when a threshold is exceeded. Default recipients are
    /// those with Billing Account Administrators and Billing
    /// Account Users IAM roles for the target account.
    #[builder(into, default)]
    #[serde(rename = "disableDefaultIamRecipients")]
    pub r#disable_default_iam_recipients: Box<Option<bool>>,
    /// When set to true, and when the budget has a single project configured,
    /// notifications will be sent to project level recipients of that project.
    /// This field will be ignored if the budget has multiple or no project configured.
    /// Currently, project level recipients are the users with Owner role on a cloud project.
    #[builder(into, default)]
    #[serde(rename = "enableProjectLevelRecipients")]
    pub r#enable_project_level_recipients: Box<Option<bool>>,
    /// The full resource name of a monitoring notification
    /// channel in the form
    /// projects/{project_id}/notificationChannels/{channel_id}.
    /// A maximum of 5 channels are allowed.
    #[builder(into, default)]
    #[serde(rename = "monitoringNotificationChannels")]
    pub r#monitoring_notification_channels: Box<Option<Vec<String>>>,
    /// The name of the Cloud Pub/Sub topic where budget related
    /// messages will be published, in the form
    /// projects/{project_id}/topics/{topic_id}. Updates are sent
    /// at regular intervals to the topic.
    #[builder(into, default)]
    #[serde(rename = "pubsubTopic")]
    pub r#pubsub_topic: Box<Option<String>>,
    /// The schema version of the notification. Only "1.0" is
    /// accepted. It represents the JSON schema as defined in
    /// https://cloud.google.com/billing/docs/how-to/budgets#notification_format.
    #[builder(into, default)]
    #[serde(rename = "schemaVersion")]
    pub r#schema_version: Box<Option<String>>,
}
