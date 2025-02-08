#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct EnvironmentLoggingConfiguration {
    /// (Optional) Log configuration options for processing DAGs. See Module logging configuration for more information. Disabled by default.
    #[builder(into, default)]
    #[serde(rename = "dagProcessingLogs")]
    pub r#dag_processing_logs: Box<Option<super::super::types::mwaa::EnvironmentLoggingConfigurationDagProcessingLogs>>,
    /// Log configuration options for the schedulers. See Module logging configuration for more information. Disabled by default.
    #[builder(into, default)]
    #[serde(rename = "schedulerLogs")]
    pub r#scheduler_logs: Box<Option<super::super::types::mwaa::EnvironmentLoggingConfigurationSchedulerLogs>>,
    /// Log configuration options for DAG tasks. See Module logging configuration for more information. Enabled by default with `INFO` log level.
    #[builder(into, default)]
    #[serde(rename = "taskLogs")]
    pub r#task_logs: Box<Option<super::super::types::mwaa::EnvironmentLoggingConfigurationTaskLogs>>,
    /// Log configuration options for the webservers. See Module logging configuration for more information. Disabled by default.
    #[builder(into, default)]
    #[serde(rename = "webserverLogs")]
    pub r#webserver_logs: Box<Option<super::super::types::mwaa::EnvironmentLoggingConfigurationWebserverLogs>>,
    /// Log configuration options for the workers. See Module logging configuration for more information. Disabled by default.
    #[builder(into, default)]
    #[serde(rename = "workerLogs")]
    pub r#worker_logs: Box<Option<super::super::types::mwaa::EnvironmentLoggingConfigurationWorkerLogs>>,
}
