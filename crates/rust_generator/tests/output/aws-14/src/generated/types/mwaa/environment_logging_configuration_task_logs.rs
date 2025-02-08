#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct EnvironmentLoggingConfigurationTaskLogs {
    #[builder(into, default)]
    #[serde(rename = "cloudWatchLogGroupArn")]
    pub r#cloud_watch_log_group_arn: Box<Option<String>>,
    /// Enabling or disabling the collection of logs
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Logging level. Valid values: `CRITICAL`, `ERROR`, `WARNING`, `INFO`, `DEBUG`. Will be `INFO` by default.
    #[builder(into, default)]
    #[serde(rename = "logLevel")]
    pub r#log_level: Box<Option<String>>,
}
