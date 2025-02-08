#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct HealthCheckLogConfig {
    /// Indicates whether or not to export logs. This is false by default,
    /// which means no health check logging will be done.
    #[builder(into, default)]
    #[serde(rename = "enable")]
    pub r#enable: Box<Option<bool>>,
}
