#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceObservabilityConfiguration {
    /// ARN of the observability configuration that is associated with the service. Specified only when `observability_enabled` is `true`.
    #[builder(into, default)]
    #[serde(rename = "observabilityConfigurationArn")]
    pub r#observability_configuration_arn: Box<Option<String>>,
    /// When `true`, an observability configuration resource is associated with the service.
    #[builder(into)]
    #[serde(rename = "observabilityEnabled")]
    pub r#observability_enabled: Box<bool>,
}
