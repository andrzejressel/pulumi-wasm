#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AwsClusterLoggingConfig {
    /// Configuration of the logging components.
    #[builder(into, default)]
    #[serde(rename = "componentConfig")]
    pub r#component_config: Box<Option<super::super::types::container::AwsClusterLoggingConfigComponentConfig>>,
}
