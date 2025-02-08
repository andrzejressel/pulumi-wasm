#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ServiceServiceConnectConfigurationLogConfiguration {
    /// Log driver to use for the container.
    #[builder(into)]
    #[serde(rename = "logDriver")]
    pub r#log_driver: Box<String>,
    /// Configuration options to send to the log driver.
    #[builder(into, default)]
    #[serde(rename = "options")]
    pub r#options: Box<Option<std::collections::HashMap<String, String>>>,
    /// Secrets to pass to the log configuration. See below.
    #[builder(into, default)]
    #[serde(rename = "secretOptions")]
    pub r#secret_options: Box<Option<Vec<super::super::types::ecs::ServiceServiceConnectConfigurationLogConfigurationSecretOption>>>,
}
