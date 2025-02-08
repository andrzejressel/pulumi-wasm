#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ServiceServiceConnectConfiguration {
    /// Whether to use Service Connect with this service.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Log configuration for the container. See below.
    #[builder(into, default)]
    #[serde(rename = "logConfiguration")]
    pub r#log_configuration: Box<Option<super::super::types::ecs::ServiceServiceConnectConfigurationLogConfiguration>>,
    /// Namespace name or ARN of the `aws.servicediscovery.HttpNamespace` for use with Service Connect.
    #[builder(into, default)]
    #[serde(rename = "namespace")]
    pub r#namespace: Box<Option<String>>,
    /// List of Service Connect service objects. See below.
    #[builder(into, default)]
    #[serde(rename = "services")]
    pub r#services: Box<Option<Vec<super::super::types::ecs::ServiceServiceConnectConfigurationService>>>,
}
