#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ObservabilityConfigurationTraceConfiguration {
    /// Implementation provider chosen for tracing App Runner services. Valid values: `AWSXRAY`.
    #[builder(into, default)]
    #[serde(rename = "vendor")]
    pub r#vendor: Box<Option<String>>,
}
