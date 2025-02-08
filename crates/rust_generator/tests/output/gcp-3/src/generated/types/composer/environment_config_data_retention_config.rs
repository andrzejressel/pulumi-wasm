#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct EnvironmentConfigDataRetentionConfig {
    /// Optional. The configuration setting for Task Logs.
    #[builder(into)]
    #[serde(rename = "taskLogsRetentionConfigs")]
    pub r#task_logs_retention_configs: Box<Vec<super::super::types::composer::EnvironmentConfigDataRetentionConfigTaskLogsRetentionConfig>>,
}
