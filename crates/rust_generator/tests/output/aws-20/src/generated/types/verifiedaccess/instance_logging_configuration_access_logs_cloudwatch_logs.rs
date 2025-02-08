#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceLoggingConfigurationAccessLogsCloudwatchLogs {
    /// Indicates whether logging is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// The name of the CloudWatch Logs Log Group.
    #[builder(into, default)]
    #[serde(rename = "logGroup")]
    pub r#log_group: Box<Option<String>>,
}
