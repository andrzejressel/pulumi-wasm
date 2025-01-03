#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobTemplateJobTemplateDataConfigurationOverrides {
    /// The configurations for the application running by the job run.
    #[builder(into, default)]
    #[serde(rename = "applicationConfigurations")]
    pub r#application_configurations: Box<Option<Vec<super::super::types::emrcontainers::JobTemplateJobTemplateDataConfigurationOverridesApplicationConfiguration>>>,
    /// The configurations for monitoring.
    #[builder(into, default)]
    #[serde(rename = "monitoringConfiguration")]
    pub r#monitoring_configuration: Box<Option<super::super::types::emrcontainers::JobTemplateJobTemplateDataConfigurationOverridesMonitoringConfiguration>>,
}
