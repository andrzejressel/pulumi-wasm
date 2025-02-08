#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetJobDefinitionNodePropertyNodeRangePropertyContainerLogConfiguration {
    /// The log driver to use for the container.
    #[builder(into)]
    #[serde(rename = "logDriver")]
    pub r#log_driver: Box<String>,
    /// The configuration options to send to the log driver.
    #[builder(into)]
    #[serde(rename = "options")]
    pub r#options: Box<std::collections::HashMap<String, String>>,
    /// The secrets to pass to the log configuration.
    #[builder(into)]
    #[serde(rename = "secretOptions")]
    pub r#secret_options: Box<Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerLogConfigurationSecretOption>>,
}
