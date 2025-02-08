#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TopicIngestionDataSourceSettingsPlatformLogsSettings {
    /// The minimum severity level of Platform Logs that will be written. If unspecified,
    /// no Platform Logs will be written.
    /// Default value is `SEVERITY_UNSPECIFIED`.
    /// Possible values are: `SEVERITY_UNSPECIFIED`, `DISABLED`, `DEBUG`, `INFO`, `WARNING`, `ERROR`.
    #[builder(into, default)]
    #[serde(rename = "severity")]
    pub r#severity: Box<Option<String>>,
}
