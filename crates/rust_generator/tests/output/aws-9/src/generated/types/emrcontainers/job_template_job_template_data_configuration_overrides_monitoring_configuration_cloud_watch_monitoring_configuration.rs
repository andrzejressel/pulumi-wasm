#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobTemplateJobTemplateDataConfigurationOverridesMonitoringConfigurationCloudWatchMonitoringConfiguration {
    /// The name of the log group for log publishing.
    #[builder(into)]
    #[serde(rename = "logGroupName")]
    pub r#log_group_name: Box<String>,
    /// The specified name prefix for log streams.
    #[builder(into, default)]
    #[serde(rename = "logStreamNamePrefix")]
    pub r#log_stream_name_prefix: Box<Option<String>>,
}
