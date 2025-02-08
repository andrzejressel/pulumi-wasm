#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApplicationApplicationConfigurationFlinkApplicationConfigurationMonitoringConfiguration {
    /// Describes whether to use the default CloudWatch logging configuration for an application. Valid values: `CUSTOM`, `DEFAULT`. Set this attribute to `CUSTOM` in order for any specified `log_level` or `metrics_level` attribute values to be effective.
    #[builder(into)]
    #[serde(rename = "configurationType")]
    pub r#configuration_type: Box<String>,
    /// Describes the verbosity of the CloudWatch Logs for an application. Valid values: `DEBUG`, `ERROR`, `INFO`, `WARN`.
    #[builder(into, default)]
    #[serde(rename = "logLevel")]
    pub r#log_level: Box<Option<String>>,
    /// Describes the granularity of the CloudWatch Logs for an application. Valid values: `APPLICATION`, `OPERATOR`, `PARALLELISM`, `TASK`.
    #[builder(into, default)]
    #[serde(rename = "metricsLevel")]
    pub r#metrics_level: Box<Option<String>>,
}
